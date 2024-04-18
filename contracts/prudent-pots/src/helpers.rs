use cosmwasm_std::{
    Addr, BankMsg, Coin, CosmosMsg, DepsMut, Env, QuerierWrapper, Storage, Uint128,
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
pub fn calculate_min_bid(storage: &dyn Storage) -> Result<Uint128, ContractError> {
    let average_tokens = calculate_average_tokens(storage)?;
    let config = GAME_CONFIG.load(storage)?;

    // If the average is less than the configured min_bid, use min_bid, otherwise use the average
    let min_bid = std::cmp::max(average_tokens, config.min_bid);
    Ok(min_bid)
}

// Helper to calculate the maximum bid based on the game's current state
pub fn calculate_max_bid(storage: &dyn Storage) -> Result<Uint128, ContractError> {
    let min_bid = calculate_min_bid(storage)?;

    // Set the maximum bid as double the minimum bid or average, whichever is higher
    let max_bid = min_bid.checked_mul(Uint128::from(2u128)).unwrap();
    Ok(max_bid)
}

// Helper to calculate the average tokens across all pots
fn calculate_average_tokens(storage: &dyn Storage) -> Result<Uint128, ContractError> {
    let pots = get_all_token_counts(storage)?;
    let total: Uint128 = pots.iter().sum();

    if pots.is_empty() {
        // Avoid division by zero if there are no pots
        Ok(Uint128::zero())
    } else {
        Ok(total
            .checked_div(Uint128::from(pots.len() as u128))
            .unwrap())
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
fn get_max_tokens(storage: &dyn Storage) -> Result<Uint128, ContractError> {
    let token_counts = get_all_token_counts(storage)?;
    Ok(*token_counts.iter().max().unwrap_or(&Uint128::zero()))
}

// Get the minimum token count from all pots
fn get_min_tokens(storage: &dyn Storage) -> Result<Uint128, ContractError> {
    let token_counts = get_all_token_counts(storage)?;
    Ok(*token_counts.iter().min().unwrap_or(&Uint128::zero()))
}

// Helper to calculate the total tokens in losing pots and winning pots without allocations
pub fn calculate_total_losing_tokens(
    storage: &dyn Storage,
    winning_pots: &[u8],
) -> Result<Uint128, ContractError> {
    let mut total_losing_tokens = Uint128::zero();

    // Iterate through all pots
    for pot_id in 1..=5 {
        // Assuming 5 pots
        let pot_state = POT_STATES.load(storage, pot_id)?;

        // Check if the pot is a losing pot or a winning pot without allocations
        if !winning_pots.contains(&pot_id) || !has_player_allocations(storage, pot_id)? {
            total_losing_tokens = total_losing_tokens.checked_add(pot_state.amount).unwrap();
        }
    }

    Ok(total_losing_tokens)
}

// Helper to check if a pot has player allocations
fn has_player_allocations(storage: &dyn Storage, pot_id: u8) -> Result<bool, ContractError> {
    let allocations = PLAYER_ALLOCATIONS.range(storage, None, None, cosmwasm_std::Order::Ascending);

    for item in allocations {
        let (_, player_allocations) = item?;
        if player_allocations
            .allocations
            .iter()
            .any(|a| a.pot_id == pot_id && !a.amount.is_zero())
        // Check for non-zero amounts
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
) -> Result<Vec<CosmosMsg>, ContractError> {
    let config = GAME_CONFIG.load(storage)?;
    let total_distribution_amount = total_losing_tokens.multiply_ratio(1u128, 2u128);

    let mut pot_contributions: Vec<Uint128> = vec![Uint128::zero(); 5]; // Assumes 5 pots
    let mut total_winning_tokens = Uint128::zero();

    // Calculate total token amounts for each winning pot and store them
    for &pot_id in winning_pots {
        if has_player_allocations(storage, pot_id)? {
            let pot_state = POT_STATES.load(storage, pot_id)?;
            pot_contributions[pot_id as usize - 1] = pot_state.amount;
            total_winning_tokens += pot_state.amount;
        }
    }

    let mut messages: Vec<CosmosMsg> = Vec::new();
    let mut total_fee = Uint128::zero();

    // Distribute tokens to winning pots based on their contribution to the total
    for &pot_id in winning_pots {
        if pot_contributions[pot_id as usize - 1].is_zero() {
            continue; // Skip pots without player allocations or tokens
        }

        let pot_share = total_distribution_amount
            .multiply_ratio(pot_contributions[pot_id as usize - 1], total_winning_tokens);
        let pot_state = POT_STATES.load(storage, pot_id)?;
        let pot_total_distribution_amount = pot_share + pot_state.amount;
        let fee = pot_total_distribution_amount.multiply_ratio(config.fee, 100u128);
        total_fee += fee;
        let net_distribution_amount = pot_total_distribution_amount.checked_sub(fee).unwrap();

        distribute_tokens_to_players(
            storage,
            &config,
            pot_id,
            net_distribution_amount,
            &mut messages,
        )?;
    }

    // Deduct the total fee and add to messages
    if !total_fee.is_zero() {
        messages.push(CosmosMsg::Bank(BankMsg::Send {
            to_address: config.fee_address.to_string(),
            amount: vec![Coin {
                denom: config.game_denom.clone(),
                amount: total_fee,
            }],
        }));
    }

    Ok(messages)
}

fn distribute_tokens_to_players(
    storage: &dyn Storage,
    config: &GameConfig,
    pot_id: u8,
    net_distribution_amount: Uint128,
    messages: &mut Vec<CosmosMsg>,
) -> Result<(), ContractError> {
    // Retrieve all player allocations from storage. This pulls the entire list of allocations, filtering out any errors.
    let player_allocations: Vec<_> = PLAYER_ALLOCATIONS
        .range(storage, None, None, cosmwasm_std::Order::Ascending)
        .filter_map(Result::ok)
        .collect();

    // Calculate the total contributions to the specified pot by all players.
    let total_player_contributions: Uint128 = player_allocations
        .iter()
        .flat_map(|(_, allocations)| allocations.allocations.iter()) // Flatten the structure to iterate over all allocations.
        .filter(|allocation| allocation.pot_id == pot_id && !allocation.amount.is_zero()) // Filter for allocations to the specific pot that are non-zero.
        .map(|allocation| allocation.amount)
        .sum();

    // Early return if there are no contributions to prevent division by zero in later calculations.
    if total_player_contributions.is_zero() {
        return Ok(());
    }

    // Loop through all player allocations to distribute the net amount based on each player's contribution to the pot.
    for (addr, allocations) in player_allocations {
        for allocation in &allocations.allocations {
            // Check if the allocation belongs to the current pot and is not zero.
            if allocation.pot_id == pot_id && !allocation.amount.is_zero() {
                // Calculate the share for this player based on their contribution relative to the total contributions.
                let player_share = net_distribution_amount
                    .multiply_ratio(allocation.amount, total_player_contributions);
                // Create a bank message to send the player's share of tokens and push it to the messages vector.
                messages.push(CosmosMsg::Bank(BankMsg::Send {
                    to_address: addr.to_string(),
                    amount: vec![Coin {
                        denom: config.game_denom.clone(), // Note: `config` needs to be accessible here, consider passing it as an argument if not globally accessible.
                        amount: player_share,
                    }],
                }));
            }
        }
    }

    Ok(())
}

/// Checks if the specified player has already allocated tokens to the specified pot.
/// Returns `Ok(())` if no allocations exist or if the allocation is zero, or `Err(ContractError::AlreadyAllocated)` if a non-zero allocation is found.
pub fn check_existing_allocation(
    storage: &dyn Storage,
    player: &Addr,
    pot_id: u8,
) -> Result<(), ContractError> {
    let player_allocs_opt = PLAYER_ALLOCATIONS.may_load(storage, player.clone())?;

    if let Some(player_allocs) = player_allocs_opt {
        // If allocations exist, check if any non-zero allocation is made to the specified pot.
        if player_allocs
            .allocations
            .iter()
            .any(|alloc| alloc.pot_id == pot_id && !alloc.amount.is_zero())
        {
            return Err(ContractError::AlreadyAllocated {});
        }
    }

    Ok(())
}

// Helper to prepare for the next game
pub fn prepare_next_game(
    deps: DepsMut,
    env: &Env,
    messages: &Vec<CosmosMsg>,
) -> Result<(), ContractError> {
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
    let mut total_tokens_for_next_game = deps
        .querier
        .query_balance(&env.contract.address, &config.game_denom)?
        .amount;

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

    total_tokens_for_next_game = total_tokens_for_next_game
        .checked_sub(total_outgoing_tokens)
        .unwrap();

    // Distribute the initial tokens and the reallocation fee pool to the pots for the next game
    let initial_tokens_per_pot = total_tokens_for_next_game
        .checked_div(Uint128::from(5u128))
        .unwrap();

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
pub fn create_fee_message(
    config: &GameConfig,
    fee: Uint128,
) -> Result<Vec<CosmosMsg>, ContractError> {
    if fee.is_zero() {
        Ok(vec![])
    } else {
        Ok(vec![CosmosMsg::Bank(BankMsg::Send {
            to_address: config.fee_address.to_string(),
            amount: vec![Coin {
                denom: config.game_denom.clone(),
                amount: fee,
            }],
        })])
    }
}
