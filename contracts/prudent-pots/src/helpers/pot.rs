use cosmwasm_std::{Addr, Decimal, Deps, Env, Storage, Uint128};
use cw721::TokensResponse;

use crate::{
    state::{
        FirstBidder, TokenAllocation, FIRST_BIDDER, GAME_CONFIG, GAME_STATE, PLAYER_ALLOCATIONS,
        POT_STATES,
    },
    ContractError,
};

/// Helper to set the FirstBidder per pot during a round, for raffle winner tie handling.
/// This function checks if a FirstBidder already exists for a given pot. If not, it sets the
/// FirstBidder with the current sender and timestamp. If a FirstBidder already exists,
/// it leaves the existing record unchanged.
pub fn set_first_bidder_if_not_set(
    storage: &mut dyn Storage,
    pot_id: u8,
    sender: &Addr,
    current_time: u64,
) -> Result<FirstBidder, ContractError> {
    FIRST_BIDDER.update(storage, pot_id, |first_bidder| {
        match first_bidder {
            Some(existing_bidder) => Ok(existing_bidder), // Return existing without modifying
            None => Ok(FirstBidder {
                // Set new first bidder because none exists
                bidder: sender.to_string(),
                time: current_time,
            }),
        }
    })
}

// Helper to update the player's allocation
pub fn update_player_allocation(
    storage: &mut dyn Storage,
    player: &Addr,
    pot_id: u8,
    amount: Uint128,
    increase: bool, // true to increase, false to decrease
) -> Result<(), ContractError> {
    PLAYER_ALLOCATIONS.update(
        storage,
        player.to_string(),
        |existing_allocations| -> Result<_, ContractError> {
            let mut allocs = existing_allocations.unwrap_or_default();
            if let Some(allocation) = allocs.iter_mut().find(|a| a.pot_id == pot_id) {
                allocation.amount = if increase {
                    allocation.amount.checked_add(amount)?
                } else {
                    allocation.amount.checked_sub(amount)?
                };
            } else if increase {
                // Only add a new allocation if we are increasing (it makes no sense to create a new allocation with a negative balance)
                allocs.push(TokenAllocation { pot_id, amount });
            }
            Ok(allocs)
        },
    )?;
    Ok(())
}

// Helper to update the pot's state
pub fn update_pot_state(
    storage: &mut dyn Storage,
    pot_id: u8,
    amount: Uint128,
    increase: bool, // true to increase, false to decrease
) -> Result<(), ContractError> {
    POT_STATES.update(storage, pot_id, |pot_state| -> Result<_, ContractError> {
        let mut state = pot_state.unwrap_or_else(|| TokenAllocation {
            pot_id,
            amount: Uint128::zero(),
        });
        state.amount = if increase {
            state.amount.checked_add(amount)?
        } else {
            state.amount.checked_sub(amount)?
        };
        Ok(state)
    })?;
    Ok(())
}

// Helper to calculate the minimum bid based on the game's current state
pub fn calculate_min_bid(
    deps: &Deps,
    env: &Env,
    address: Option<String>,
) -> Result<Uint128, ContractError> {
    let game_config = GAME_CONFIG.load(deps.storage)?;
    let game_state = GAME_STATE.load(deps.storage)?;
    let current_timestamp = env.block.time.seconds();

    // Calculate the current epoch based on the game's start time and duration
    let elapsed_time = current_timestamp
        .checked_sub(game_state.start_time)
        .unwrap();
    let current_epoch_count = elapsed_time
        .checked_div(game_config.game_duration_epoch)
        .unwrap();

    // Calculate the adjusted decay factor considering the extend count
    let adjusted_decay_factor = game_config
        .decay_factor
        .checked_mul(Decimal::from_ratio(current_epoch_count, 1u64))?
        .checked_mul(Decimal::from_ratio(
            game_state.extend_count as u64 + 1,
            1u64,
        ))?;

    // Calculate the base multiplier based on the adjusted decay factor
    let base_multiplier = Decimal::one().checked_add(adjusted_decay_factor)?;
    println!("base_multiplier: {:?}", base_multiplier);

    // Calculate min_bid as an integer multiplication of the initial allocation and the multiplier
    let min_bid = game_config.min_pot_initial_allocation * base_multiplier;

    // Only proceed with querying cw721 tokens if an address is provided
    let mut cw721_count = 0;
    if let Some(owner) = address {
        // Query multiple cw721 addresses and count the total number of tokens
        for addr in &game_config.game_cw721_addrs {
            let tokens_resp: TokensResponse = deps.querier.query_wasm_smart(
                addr,
                &cw721::Cw721QueryMsg::Tokens {
                    owner: owner.clone(), // Pass the owner directly since it's now available
                    start_after: None,
                    limit: None,
                },
            )?;
            cw721_count += tokens_resp.tokens.len();
        }
    }

    // Apply discount based on the number of whitelisted NFT tokens owned
    let discounted_min_bid =
        calculate_discounted_bid(min_bid, cw721_count, game_config.decay_factor);

    Ok(discounted_min_bid)
}

// Helper to calculate the average tokens across all pots
fn calculate_average_tokens(storage: &dyn Storage) -> Result<Uint128, ContractError> {
    let pots = get_all_token_counts(storage)?;
    let total: Uint128 = pots.iter().sum();

    if pots.is_empty() {
        // Avoid division by zero if there are no pots
        Ok(Uint128::zero())
    } else {
        Ok(total.checked_div(Uint128::from(pots.len() as u128))?)
    }
}

fn calculate_discounted_bid(
    mut min_bid: Uint128,
    token_amount: usize,
    decay_factor: Decimal,
) -> Uint128 {
    for _ in 0..token_amount {
        // Calculate the discount amount using Decimal
        let discount_amount = decay_factor * Decimal::from_ratio(min_bid, Uint128::one());
        // Convert the discount amount to Uint128
        let discount_amount_uint128 = Uint128::from(discount_amount.atomics());

        // Subtract the discount amount from the current min bid amount
        min_bid = min_bid
            .checked_sub(discount_amount_uint128)
            .unwrap_or(min_bid);
    }

    min_bid
}

// Helper to calculate the maximum bid based on the game's current state
pub fn calculate_max_bid(deps: &Deps, original_min_bid: Uint128) -> Result<Uint128, ContractError> {
    // TODO: What if nobody plays the game? The maxBid will be 0.2 forever and the minBid will be increasing, maybe too much?
    let average_tokens = calculate_average_tokens(deps.storage)?;

    // Set the maximum bid as double the average of tokens in the pots, whichever is higher
    let max_bid = average_tokens.checked_mul(Uint128::from(2u128))?;

    // if the max bid calculated by average amount among pots results in a lower value than the original min bid, return the original min bid times 2
    if max_bid.lt(&original_min_bid) {
        return Ok(original_min_bid.checked_mul(Uint128::from(2u128))?);
    }

    Ok(max_bid)
}

// Helper to determine if a pot is a winning pot based on its unique rules
pub fn is_winning_pot(storage: &dyn Storage, pot_id: u8) -> Result<bool, ContractError> {
    let pot_state = POT_STATES.load(storage, pot_id)?;

    match pot_id {
        // Lowest
        1 => {
            let min_tokens = get_min_tokens(storage)?;
            let is_lowest = pot_state.amount == min_tokens;
            let is_unique = get_all_token_counts(storage)?
                .iter()
                .filter(|&count| *count == min_tokens)
                .count()
                == 1;
            Ok(is_lowest && is_unique)
        }

        // Even
        2 => Ok((pot_state.amount % Uint128::from(2u128)).is_zero()),

        // Median
        3 => {
            let token_counts = get_all_token_counts(storage)?;
            let is_median = is_median(&token_counts, pot_state.amount);
            let is_unique = token_counts
                .iter()
                .filter(|&count| *count == pot_state.amount)
                .count()
                == 1;
            Ok(is_median && is_unique)
        }
        // Odd
        4 => Ok(!(pot_state.amount % Uint128::from(2u128)).is_zero()),

        // Highest
        5 => {
            let max_tokens = get_max_tokens(storage)?;
            let is_highest = pot_state.amount == max_tokens;
            let is_unique = get_all_token_counts(storage)?
                .iter()
                .filter(|&count| *count == max_tokens)
                .count()
                == 1;
            Ok(is_highest && is_unique)
        }

        _ => Err(ContractError::InvalidPot {}),
    }
}

// Check if a value is the median in a vector of token counts
fn is_median(token_counts: &[Uint128], value: Uint128) -> bool {
    let mut sorted_counts = token_counts.to_owned();
    sorted_counts.sort_unstable();
    let mid = sorted_counts.len() / 2;

    if sorted_counts.len() % 2 == 0 {
        (sorted_counts[mid - 1] <= value) && (value <= sorted_counts[mid])
    } else {
        value == sorted_counts[mid]
    }
}

// Retrieve the token count for each pot
fn get_all_token_counts(storage: &dyn Storage) -> Result<Vec<Uint128>, ContractError> {
    let mut token_counts = Vec::new();
    for pot_id in 1..=5 {
        // Assuming 5 pots
        let pot_state = POT_STATES.load(storage, pot_id)?;
        token_counts.push(pot_state.amount);
    }

    Ok(token_counts)
}

// Get the maximum token count from all pots
fn get_max_tokens(storage: &dyn Storage) -> Result<Uint128, ContractError> {
    let token_counts = get_all_token_counts(storage)?;
    Ok(*token_counts.iter().max().unwrap_or(&Uint128::zero()))
}

// Get the minimum token count from all pots
fn get_min_tokens(storage: &dyn Storage) -> Result<Uint128, ContractError> {
    let token_counts = get_all_token_counts(storage)?;
    Ok(*token_counts.iter().min().unwrap_or(&Uint128::zero()))
}

pub fn get_winning_pots(storage: &dyn Storage) -> Result<Vec<u8>, ContractError> {
    Ok((1..=5)
        .filter(|&pot_id| is_winning_pot(storage, pot_id).unwrap_or(false))
        .collect::<Vec<u8>>())
}
