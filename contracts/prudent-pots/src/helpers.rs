use cosmwasm_std::{
    Addr, BankMsg, Coin, CosmosMsg, DepsMut, Env, QuerierWrapper, StdError, StdResult, Storage,
    Uint128,
};

use crate::{
    state::{
        GameConfig, GameState, PlayerAllocations, TokenAllocation, GAME_CONFIG, GAME_STATE,
        PLAYER_ALLOCATIONS, POT_STATES, REALLOCATION_FEE_POOL,
    },
    ContractError,
};

pub fn is_contract_admin(
    querier: &QuerierWrapper,
    env: &Env,
    sus_admin: &Addr,
) -> Result<(), ContractError> {
    let contract_admin = querier
        .query_wasm_contract_info(&env.contract.address)?
        .admin;
    if let Some(contract_admin) = contract_admin {
        if contract_admin != *sus_admin {
            return Err(ContractError::Unauthorized {});
        }
    } else {
        return Err(ContractError::Unauthorized {});
    }
    Ok(())
}

// Helper to calculate the minimum bid based on the game's current state
pub fn calculate_min_bid(storage: &dyn Storage) -> StdResult<Uint128> {
    let average_tokens = calculate_average_tokens(storage)?;
    let config = GAME_CONFIG.load(storage)?;

    // If the average is less than the configured min_bid, use min_bid, otherwise use the average
    let min_bid = std::cmp::max(average_tokens, config.min_bid);
    Ok(min_bid)
}

// Helper to calculate the maximum bid based on the game's current state
pub fn calculate_max_bid(storage: &dyn Storage) -> StdResult<Uint128> {
    let min_bid = calculate_min_bid(storage)?;

    // Set the maximum bid as double the minimum bid or average, whichever is higher
    let max_bid = min_bid.checked_mul(Uint128::from(2u128))?;
    Ok(max_bid)
}

// Helper to calculate the average tokens across all pots
fn calculate_average_tokens(storage: &dyn Storage) -> StdResult<Uint128> {
    let pots = get_all_token_counts(storage)?;
    let total: Uint128 = pots.iter().sum();

    if pots.is_empty() {
        // Avoid division by zero if there are no pots
        Ok(Uint128::zero())
    } else {
        Ok(total.checked_div(Uint128::from(pots.len() as u128))?)
    }
}

// Retrieve the token count for each pot
fn get_all_token_counts(storage: &dyn Storage) -> StdResult<Vec<Uint128>> {
    let mut token_counts = Vec::new();
    for pot_id in 1..=5 {
        // Assuming 5 pots
        let pot_state = POT_STATES.load(storage, pot_id)?;
        token_counts.push(pot_state.amount);
    }
    Ok(token_counts)
}

// Helper to determine if a pot is a winning pot based on its unique rules
pub fn is_winning_pot(storage: &dyn Storage, pot_id: u8) -> StdResult<bool> {
    let pot_state = POT_STATES.load(storage, pot_id)?;

    // TODO: Implement in this scope the logic about making a winning pot looser if it has not player allocations in it.
    // Consider that this will make the FE always show red loosing pots even if they are winning.
    // What do we want to do?

    match pot_id {
        1 => {
            // For Median Pot: Compare with other pots to determine if it's the median
            let token_counts = get_all_token_counts(storage)?;
            Ok(is_median(&token_counts, pot_state.amount))
        }
        2 => {
            // For Highest Pot: Compare with other pots to determine if it's the highest
            let max_tokens = get_max_tokens(storage)?;
            Ok(pot_state.amount == max_tokens)
        }
        3 => {
            // For Even Pot: Check if the token count is even
            Ok((pot_state.amount % Uint128::from(2u128)).is_zero())
        }
        4 => {
            // For Lowest Pot: Compare with other pots to determine if it's the lowest
            let min_tokens = get_min_tokens(storage)?;
            Ok(pot_state.amount == min_tokens)
        }
        5 => {
            // For Prime Pot: Check if the token count is a prime number
            Ok(is_prime(pot_state.amount.u128()))
        }
        _ => Err(StdError::generic_err("Invalid pot ID")),
    }
}

// Check if a value is the median in a vector of token counts
fn is_median(token_counts: &Vec<Uint128>, value: Uint128) -> bool {
    let mut sorted_counts = token_counts.clone();
    sorted_counts.sort_unstable();
    let mid = sorted_counts.len() / 2;

    if sorted_counts.len() % 2 == 0 {
        (sorted_counts[mid - 1] <= value) && (value <= sorted_counts[mid])
    } else {
        value == sorted_counts[mid]
    }
}

// Get the maximum token count from all pots
fn get_max_tokens(storage: &dyn Storage) -> StdResult<Uint128> {
    let token_counts = get_all_token_counts(storage)?;
    Ok(*token_counts.iter().max().unwrap_or(&Uint128::zero()))
}

// Get the minimum token count from all pots
fn get_min_tokens(storage: &dyn Storage) -> StdResult<Uint128> {
    let token_counts = get_all_token_counts(storage)?;
    Ok(*token_counts.iter().min().unwrap_or(&Uint128::zero()))
}

// Check if a number is prime
fn is_prime(number: u128) -> bool {
    if number <= 1 {
        return false;
    }
    for i in 2..=(number as f64).sqrt() as u128 {
        if number % i == 0 {
            return false;
        }
    }
    true
}

// Helper to calculate the total tokens in losing pots and winning pots without allocations
pub fn calculate_total_losing_tokens(
    storage: &dyn Storage,
    winning_pots: &[u8],
) -> StdResult<Uint128> {
    let mut total_losing_tokens = Uint128::zero();

    // Iterate through all pots
    for pot_id in 1..=5 {
        // Assuming 5 pots
        let pot_state = POT_STATES.load(storage, pot_id)?;

        // Check if the pot is a losing pot or a winning pot without allocations
        if !winning_pots.contains(&pot_id) || !has_player_allocations(storage, pot_id)? {
            total_losing_tokens = total_losing_tokens.checked_add(pot_state.amount)?;
        }
    }

    Ok(total_losing_tokens)
}

// Helper to check if a pot has player allocations
fn has_player_allocations(storage: &dyn Storage, pot_id: u8) -> StdResult<bool> {
    let allocations = PLAYER_ALLOCATIONS.range(storage, None, None, cosmwasm_std::Order::Ascending);

    for item in allocations {
        let (_, player_allocations) = item?;
        if player_allocations
            .allocations
            .iter()
            .any(|a| a.pot_id == pot_id)
        {
            return Ok(true);
        }
    }

    Ok(false)
}

pub fn get_distribute_bank_msgs(
    storage: &dyn Storage,
    winning_pots: &[u8],
    total_losing_tokens: Uint128,
) -> StdResult<Vec<CosmosMsg>> {
    let config = GAME_CONFIG.load(storage)?;
    let total_distribution_amount = total_losing_tokens.multiply_ratio(1u128, 2u128);

    // Count winning pots with player allocations
    let winning_pots_with_allocations_count: usize = winning_pots
        .iter()
        .filter(|&&pot_id| has_player_allocations(storage, pot_id).unwrap_or(false))
        .count();

    // Split the total distribution amount equally among the winning pots with player allocations
    let individual_pot_distribution_amount = if winning_pots_with_allocations_count > 0 {
        total_distribution_amount / Uint128::from(winning_pots_with_allocations_count as u128)
    } else {
        Uint128::zero()
    };

    // TODO: Append fee and remove from allocate

    let mut messages: Vec<CosmosMsg> = vec![];

    // Iterate through winning pots with player allocations
    for &pot_id in winning_pots {
        if has_player_allocations(storage, pot_id)? {
            // The total amount to be distributed for this pot includes its share of the distribution amount plus the player contributions
            let pot_state = POT_STATES.load(storage, pot_id)?;
            let pot_total_distribution_amount =
                individual_pot_distribution_amount + pot_state.amount;

            let player_allocations: Vec<_> = PLAYER_ALLOCATIONS
                .range(storage, None, None, cosmwasm_std::Order::Ascending)
                .filter_map(Result::ok)
                .collect();

            // Calculate the total player contributions for this pot
            let total_player_contributions: Uint128 = player_allocations
                .iter()
                .flat_map(|(_, allocations)| allocations.allocations.iter())
                .filter(|allocation| allocation.pot_id == pot_id)
                .map(|allocation| allocation.amount)
                .sum();

            // Distribute the pot's total amount among its players
            for (addr, allocations) in player_allocations {
                for allocation in &allocations.allocations {
                    if allocation.pot_id == pot_id {
                        let player_share = pot_total_distribution_amount
                            .multiply_ratio(allocation.amount, total_player_contributions);

                        messages.push(CosmosMsg::Bank(BankMsg::Send {
                            to_address: addr.to_string(),
                            amount: vec![Coin {
                                denom: config.game_denom.clone(),
                                amount: player_share,
                            }],
                        }));
                    }
                }
            }
        }
    }

    Ok(messages)
}

// Helper to prepare for the next game
pub fn prepare_next_game(deps: DepsMut, env: &Env, messages: &Vec<CosmosMsg>) -> StdResult<()> {
    let config = GAME_CONFIG.load(deps.storage)?;
    let game_duration = config.game_duration;

    let next_game_start = env.block.time.seconds() + 1; // Start the next game 1 second in the future
    let next_game_end = next_game_start + game_duration;

    // Reset the game state for the next game
    let new_game_state = GameState {
        start_time: next_game_start,
        end_time: next_game_end,
    };
    GAME_STATE.save(deps.storage, &new_game_state)?;

    // Reset player allocations for the next game
    PLAYER_ALLOCATIONS.clear(deps.storage);

    // Calculate the initial tokens for each pot, considering the reallocation fee pool
    let reallocation_fee_pool = REALLOCATION_FEE_POOL.load(deps.storage)?;
    let mut total_tokens_for_next_game = deps
        .querier
        .query_balance(&env.contract.address, &config.game_denom)?
        .amount
        + reallocation_fee_pool;

    // Subtract the tokens that will be sent out from the total tokens for the next game
    let total_outgoing_tokens: Uint128 = messages
        .iter()
        .filter_map(|msg| {
            if let CosmosMsg::Bank(BankMsg::Send { amount, .. }) = msg {
                amount
                    .iter()
                    .find(|coin| coin.denom == config.game_denom)
                    .map(|coin| coin.amount)
            } else {
                None
            }
        })
        .sum();

    total_tokens_for_next_game = total_tokens_for_next_game.checked_sub(total_outgoing_tokens)?;

    // Distribute the initial tokens and the reallocation fee pool to the pots for the next game
    let initial_tokens_per_pot = total_tokens_for_next_game.checked_div(Uint128::from(5u128))?;

    for pot_id in 1..=5 {
        POT_STATES.save(
            deps.storage,
            pot_id,
            &TokenAllocation {
                pot_id,
                amount: initial_tokens_per_pot,
            },
        )?;
    }

    // Reset the reallocation fee pool for the next game
    REALLOCATION_FEE_POOL.save(deps.storage, &Uint128::zero())?;

    Ok(())
}

// Helper to validate the game's end time and extend it if necessary
pub fn validate_and_extend_game_time(
    storage: &mut dyn Storage,
    env: &Env,
) -> Result<(), ContractError> {
    let mut game_state = GAME_STATE.load(storage)?;

    // Check if the game has already ended
    if env.block.time.seconds() >= game_state.end_time {
        return Err(ContractError::GameAlreadyEnded {});
    }

    // Get the current configuration
    let game_config = GAME_CONFIG.load(storage)?;

    // Calculate the remaining time
    let remaining_time = game_state.end_time - env.block.time.seconds();

    // Extend the game time if the remaining time is less than or equal to game_config.game_extend
    if remaining_time <= game_config.game_extend {
        game_state.end_time = env.block.time.seconds() + game_config.game_extend;
        GAME_STATE.save(storage, &game_state)?;
    }

    Ok(())
}

// Helper to validate and sum the funds in the specified denomination
pub fn validate_and_sum_funds(
    funds: &Vec<Coin>,
    expected_denom: &str,
) -> Result<Uint128, ContractError> {
    let total_amount = funds.iter().fold(Uint128::zero(), |acc, coin| {
        if coin.denom == expected_denom {
            acc.checked_add(coin.amount).unwrap()
        } else {
            acc
        }
    });

    if total_amount.is_zero() {
        return Err(ContractError::InvalidFunds {});
    }

    Ok(total_amount)
}

// Helper to update the player's allocation
pub fn update_player_allocation(
    storage: &mut dyn Storage,
    player: &Addr,
    pot_id: u8,
    amount: Uint128,
) -> Result<(), ContractError> {
    PLAYER_ALLOCATIONS.update(
        storage,
        player.clone(),
        |existing_allocations| -> Result<_, ContractError> {
            let mut allocs = existing_allocations.unwrap_or_else(|| PlayerAllocations {
                allocations: Vec::new(),
            });
            if let Some(allocation) = allocs.allocations.iter_mut().find(|a| a.pot_id == pot_id) {
                allocation.amount = allocation.amount.checked_add(amount).unwrap();
            } else {
                allocs.allocations.push(TokenAllocation { pot_id, amount });
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
) -> Result<(), ContractError> {
    POT_STATES.update(storage, pot_id, |pot_state| -> Result<_, ContractError> {
        let mut state = pot_state.unwrap();
        state.amount = state.amount.checked_add(amount).unwrap();
        Ok(state)
    })?;
    Ok(())
}

// Helper to create a bank message for the fee transaction
pub fn create_fee_message(config: &GameConfig, fee: Uint128) -> StdResult<Vec<CosmosMsg>> {
    if fee.is_zero() {
        Ok(vec![])
    } else {
        Ok(vec![CosmosMsg::Bank(BankMsg::Send {
            to_address: config.fee_allocation_address.to_string(),
            amount: vec![Coin {
                denom: config.game_denom.clone(),
                amount: fee,
            }],
        })])
    }
}
