use cosmwasm_std::{
    Addr, BankMsg, Coin, CosmosMsg, Deps, DepsMut, Env, MessageInfo, QuerierWrapper, StdError,
    StdResult, Storage, Uint128,
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
pub fn calculate_min_bid(deps: &Deps) -> StdResult<Uint128> {
    let average_tokens = calculate_average_tokens(deps)?;
    let config = GAME_CONFIG.load(deps.storage)?;

    // If the average is less than the configured min_bid, use min_bid, otherwise use the average
    let min_bid = std::cmp::max(average_tokens, config.min_bid);
    Ok(min_bid)
}

// Helper to calculate the maximum bid based on the game's current state
pub fn calculate_max_bid(deps: &Deps) -> StdResult<Uint128> {
    let min_bid = calculate_min_bid(deps)?;

    // Set the maximum bid as double the minimum bid or average, whichever is higher
    let max_bid = min_bid.checked_mul(Uint128::from(2u128))?;
    Ok(max_bid)
}

// Helper to calculate the average tokens across all pots
fn calculate_average_tokens(deps: &Deps) -> StdResult<Uint128> {
    let pots = get_all_token_counts(deps)?;
    let total: Uint128 = pots.iter().sum();

    if pots.is_empty() {
        // Avoid division by zero if there are no pots
        Ok(Uint128::zero())
    } else {
        Ok(total.checked_div(Uint128::from(pots.len() as u128))?)
    }
}

// Helper to determine if a pot is a winning pot based on its unique rules
pub fn is_winning_pot(deps: &Deps, pot_id: u8) -> StdResult<bool> {
    let pot_state = POT_STATES.load(deps.storage, pot_id)?;

    match pot_id {
        1 => {
            // For Median Pot: Compare with other pots to determine if it's the median
            let token_counts = get_all_token_counts(&deps)?;
            Ok(is_median(&token_counts, pot_state.amount))
        }
        2 => {
            // For Highest Pot: Compare with other pots to determine if it's the highest
            let max_tokens = get_max_tokens(deps)?;
            Ok(pot_state.amount == max_tokens)
        }
        3 => {
            // For Even Pot: Check if the token count is even
            Ok((pot_state.amount % Uint128::from(2u128)).is_zero())
        }
        4 => {
            // For Lowest Pot: Compare with other pots to determine if it's the lowest
            let min_tokens = get_min_tokens(deps)?;
            Ok(pot_state.amount == min_tokens)
        }
        5 => {
            // For Prime Pot: Check if the token count is a prime number
            Ok(is_prime(pot_state.amount.u128()))
        }
        _ => Err(StdError::generic_err("Invalid pot ID")),
    }
}

// Retrieve the token count for each pot
fn get_all_token_counts(deps: &Deps) -> StdResult<Vec<Uint128>> {
    let mut token_counts = Vec::new();
    for pot_id in 1..=5 {
        // Assuming 5 pots
        let pot_state = POT_STATES.load(deps.storage, pot_id)?;
        token_counts.push(pot_state.amount);
    }
    Ok(token_counts)
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
fn get_max_tokens(deps: &Deps) -> StdResult<Uint128> {
    let token_counts = get_all_token_counts(&deps)?;
    Ok(*token_counts.iter().max().unwrap_or(&Uint128::zero()))
}

// Get the minimum token count from all pots
fn get_min_tokens(deps: &Deps) -> StdResult<Uint128> {
    let token_counts = get_all_token_counts(&deps)?;
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
pub fn calculate_total_losing_tokens(deps: &Deps, winning_pots: &[u8]) -> StdResult<Uint128> {
    let mut total_losing_tokens = Uint128::zero();

    // Iterate through all pots
    for pot_id in 1..=5 {
        // Assuming 5 pots
        let pot_state = POT_STATES.load(deps.storage, pot_id)?;

        // Check if the pot is a losing pot or a winning pot without allocations
        if !winning_pots.contains(&pot_id) || !has_player_allocations(deps, pot_id)? {
            total_losing_tokens = total_losing_tokens.checked_add(pot_state.amount)?;
        }
    }

    Ok(total_losing_tokens)
}

pub fn distribute_tokens(
    deps: &mut DepsMut,
    winning_pots: &[u8],
    total_losing_tokens: Uint128,
) -> StdResult<Vec<CosmosMsg>> {
    let config = GAME_CONFIG.load(deps.storage)?;

    // Calculate the amount to distribute (50% of the total losing tokens)
    let distribution_amount = total_losing_tokens.multiply_ratio(1u128, 2u128);

    let mut messages: Vec<CosmosMsg> = vec![];

    // Iterate through the winning pots
    for pot_id in winning_pots {
        // Check if the pot has player allocations
        if has_player_allocations(&deps.as_ref(), *pot_id)? {
            let player_allocations: Vec<_> = PLAYER_ALLOCATIONS
                .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
                .filter_map(Result::ok)
                .collect();

            // Calculate total player-contributed amount for the pot
            let total_player_contributions: Uint128 = player_allocations
                .iter()
                .flat_map(|(_, allocations)| allocations.allocations.iter())
                .filter(|allocation| allocation.pot_id == *pot_id)
                .map(|allocation| allocation.amount)
                .sum();

            // Calculate the pot's share of the distribution amount
            let pot_share =
                distribution_amount.multiply_ratio(total_player_contributions, total_losing_tokens);

            // Distribute the pot's share among its players based on their allocations
            for (addr, allocations) in player_allocations {
                for allocation in &allocations.allocations {
                    if allocation.pot_id == *pot_id {
                        // Calculate the player's share based on their contribution relative to total player contributions
                        let player_share =
                            pot_share.multiply_ratio(allocation.amount, total_player_contributions);

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

// Helper to check if a pot has player allocations
pub fn has_player_allocations(deps: &Deps, pot_id: u8) -> StdResult<bool> {
    let allocations =
        PLAYER_ALLOCATIONS.range(deps.storage, None, None, cosmwasm_std::Order::Ascending);

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

    // Extend the game time by 600 seconds
    game_state.end_time = std::cmp::max(env.block.time.seconds() + 600, game_state.end_time);
    GAME_STATE.save(storage, &game_state)?;

    Ok(())
}

// Helper to validate and sum the funds in the specified denomination
pub fn validate_and_sum_funds(
    info: &MessageInfo,
    expected_denom: &str,
) -> Result<Uint128, ContractError> {
    let total_amount = info.funds.iter().fold(Uint128::zero(), |acc, coin| {
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

#[cfg(test)]
mod tests {
    use crate::state::{GameConfig, PlayerAllocations, TokenAllocation};

    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env};
    use cosmwasm_std::Uint128;

    // Setup fixtures

    fn setup_pots(deps: &mut DepsMut, tokens: Vec<Uint128>) {
        for (i, &amount) in tokens.iter().enumerate() {
            let pot_id = i as u8 + 1; // Pot IDs are 1-indexed
            POT_STATES
                .save(deps.storage, pot_id, &TokenAllocation { pot_id, amount })
                .unwrap();
        }
    }

    fn setup_pots_and_allocations(
        deps: &mut DepsMut,
        pots: Vec<Uint128>,
        allocations: Vec<(u8, Addr, Uint128)>,
    ) {
        GAME_CONFIG
            .save(
                deps.storage,
                &GameConfig {
                    fee_allocation: 2,
                    fee_reallocation: 5,
                    fee_allocation_address: Addr::unchecked("addy"),
                    game_duration: 3600,
                    game_denom: "token".to_string(),
                    min_bid: Uint128::new(1000000u128),
                },
            )
            .unwrap();

        REALLOCATION_FEE_POOL
            .save(deps.storage, &Uint128::zero())
            .unwrap();

        for (i, &amount) in pots.iter().enumerate() {
            let pot_id = i as u8 + 1; // Pot IDs are 1-indexed
            POT_STATES
                .save(deps.storage, pot_id, &TokenAllocation { pot_id, amount })
                .unwrap();
        }

        for (pot_id, player, amount) in allocations {
            // Load existing allocations or initialize if not found
            let mut player_allocations = PLAYER_ALLOCATIONS
                .load(deps.storage, player.clone())
                .unwrap_or_else(|_| PlayerAllocations {
                    allocations: vec![],
                });

            // Add new allocation
            player_allocations
                .allocations
                .push(TokenAllocation { pot_id, amount });

            // Save the updated allocations
            PLAYER_ALLOCATIONS
                .save(deps.storage, player, &player_allocations)
                .unwrap();

            // Update the pot's total tokens to reflect the player's allocation
            POT_STATES
                .update(deps.storage, pot_id, |pot_state| -> Result<_, StdError> {
                    let mut state = pot_state.unwrap();
                    state.amount = state.amount.checked_add(amount).unwrap();
                    Ok(state)
                })
                .unwrap();
        }
    }

    // is_winning_pot

    #[test]
    fn identify_all_winning_pots_for_median() {
        let mut deps = mock_dependencies();

        setup_pots(
            &mut deps.as_mut(),
            vec![
                Uint128::new(27), // Median
                Uint128::new(27), // Highest
                Uint128::new(31), // Even
                Uint128::new(25), // Lowest
                Uint128::new(10), // Prime
            ],
        );

        // Pot 1 has 27 tokens and should be the median in this setup
        let result = is_winning_pot(&deps.as_ref(), 1).unwrap();
        assert_eq!(
            result, true,
            "Pot 1 should be winning as it has the median token count when tie-breaking by pot ID"
        );

        // Ensure that other pots are not falsely reported as winners
        let result = is_winning_pot(&deps.as_ref(), 2).unwrap();
        assert_eq!(result, false, "Pot 2 should not be winning.");
        let result = is_winning_pot(&deps.as_ref(), 3).unwrap();
        assert_eq!(result, false, "Pot 3 should not be winning.");
        let result = is_winning_pot(&deps.as_ref(), 4).unwrap();
        assert_eq!(result, false, "Pot 4 should not be winning.");
        let result = is_winning_pot(&deps.as_ref(), 5).unwrap();
        assert_eq!(result, false, "Pot 5 should not be winning.");
    }

    #[test]
    fn is_winning_pot_highest() {
        let mut deps = mock_dependencies();

        setup_pots(
            &mut deps.as_mut(),
            vec![
                Uint128::new(10), // Median
                Uint128::new(60), // Highest
                Uint128::new(31), // Even
                Uint128::new(25), // Lowest
                Uint128::new(10), // Prime
            ],
        );

        // Pot 2 has 60 tokens and should be the highest in this setup
        let result = is_winning_pot(&deps.as_ref(), 2).unwrap();
        assert_eq!(
            result, true,
            "Pot 2 should be winning as it has the highest token count when"
        );

        // Ensure that other pots are not falsely reported as winners
        let result = is_winning_pot(&deps.as_ref(), 1).unwrap();
        assert_eq!(result, false, "Pot 1 should not be winning.");
        let result = is_winning_pot(&deps.as_ref(), 3).unwrap();
        assert_eq!(result, false, "Pot 3 should not be winning.");
        let result = is_winning_pot(&deps.as_ref(), 4).unwrap();
        assert_eq!(result, false, "Pot 4 should not be winning.");
        let result = is_winning_pot(&deps.as_ref(), 5).unwrap();
        assert_eq!(result, false, "Pot 5 should not be winning.");
    }

    #[test]
    fn is_winning_pot_even() {
        let mut deps = mock_dependencies();

        setup_pots(
            &mut deps.as_mut(),
            vec![
                Uint128::new(10), // Median
                Uint128::new(30), // Highest
                Uint128::new(60), // Even
                Uint128::new(25), // Lowest
                Uint128::new(10), // Prime
            ],
        );

        // Pot 3 has 60 tokens and should be the even in this setup
        let result = is_winning_pot(&deps.as_ref(), 3).unwrap();
        assert_eq!(
            result, true,
            "Pot 3 should be winning as it has the even token count when"
        );

        // Ensure that other pots are not falsely reported as winners
        let result = is_winning_pot(&deps.as_ref(), 1).unwrap();
        assert_eq!(result, false, "Pot 1 should not be winning.");
        let result = is_winning_pot(&deps.as_ref(), 2).unwrap();
        assert_eq!(result, false, "Pot 2 should not be winning.");
        let result = is_winning_pot(&deps.as_ref(), 4).unwrap();
        assert_eq!(result, false, "Pot 4 should not be winning.");
        let result = is_winning_pot(&deps.as_ref(), 5).unwrap();
        assert_eq!(result, false, "Pot 5 should not be winning.");
    }

    #[test]
    fn is_winning_pot_lowest() {
        let mut deps = mock_dependencies();

        setup_pots(
            &mut deps.as_mut(),
            vec![
                Uint128::new(32), // Median
                Uint128::new(30), // Highest
                Uint128::new(61), // Even
                Uint128::new(1),  // Lowest
                Uint128::new(10), // Prime
            ],
        );

        // Pot 4 has 1 tokens and should be the lowest in this setup
        let result = is_winning_pot(&deps.as_ref(), 4).unwrap();
        assert_eq!(
            result, true,
            "Pot 4 should be winning as it has the lowest token count when"
        );

        // Ensure that other pots are not falsely reported as winners
        let result = is_winning_pot(&deps.as_ref(), 1).unwrap();
        assert_eq!(result, false, "Pot 1 should not be winning.");
        let result = is_winning_pot(&deps.as_ref(), 2).unwrap();
        assert_eq!(result, false, "2 should not be winning.");
        let result = is_winning_pot(&deps.as_ref(), 3).unwrap();
        assert_eq!(result, false, "Pot 3 should not be winning.");
        let result = is_winning_pot(&deps.as_ref(), 5).unwrap();
        assert_eq!(result, false, "Pot 5 should not be winning.");
    }

    #[test]
    fn is_winning_pot_prime() {
        let mut deps = mock_dependencies();

        setup_pots(
            &mut deps.as_mut(),
            vec![
                Uint128::new(32), // Median
                Uint128::new(30), // Highest
                Uint128::new(61), // Even
                Uint128::new(4),  // Lowest
                Uint128::new(3),  // Prime
            ],
        );

        // Pot 5 has 3 tokens and should be the prime in this setup
        let result = is_winning_pot(&deps.as_ref(), 5).unwrap();
        assert_eq!(
            result, true,
            "Pot 5 should be winning as it has the prime token count when"
        );

        // Ensure that other pots are not falsely reported as winners
        let result = is_winning_pot(&deps.as_ref(), 1).unwrap();
        assert_eq!(result, false, "Pot 1 should not be winning.");
        let result = is_winning_pot(&deps.as_ref(), 2).unwrap();
        assert_eq!(result, false, "Pot 2 should not be winning.");
        let result = is_winning_pot(&deps.as_ref(), 3).unwrap();
        assert_eq!(result, false, "Pot 3 should not be winning.");
        let result = is_winning_pot(&deps.as_ref(), 4).unwrap();
        assert_eq!(result, false, "Pot 4 should not be winning.");
    }

    // calculate_total_losing_tokens

    #[test]
    fn total_losing_tokens_single_winner() {
        let mut deps = mock_dependencies();
        setup_pots(
            &mut deps.as_mut(),
            vec![
                Uint128::new(10),
                Uint128::new(20),
                Uint128::new(30),
                Uint128::new(40),
                Uint128::new(50),
            ],
        );

        // Let's say pot 3 is the winner
        let total_losing_tokens = calculate_total_losing_tokens(&deps.as_ref(), &[3]).unwrap();
        assert_eq!(
            total_losing_tokens,
            Uint128::new(10 + 20 + 40 + 50),
            "Should sum tokens from all pots except pot 3"
        );
    }

    #[test]
    fn total_losing_tokens_multiple_winners() {
        let mut deps = mock_dependencies();
        setup_pots(
            &mut deps.as_mut(),
            vec![
                Uint128::new(10),
                Uint128::new(20),
                Uint128::new(30),
                Uint128::new(40),
                Uint128::new(50),
            ],
        );

        // Let's say pots 2 and 4 are winners
        let total_losing_tokens = calculate_total_losing_tokens(&deps.as_ref(), &[2, 4]).unwrap();
        assert_eq!(
            total_losing_tokens,
            Uint128::new(10 + 30 + 50),
            "Should sum tokens from pots 1, 3, and 5"
        );
    }

    #[test]
    fn total_losing_tokens_no_winners() {
        let mut deps = mock_dependencies();
        setup_pots(
            &mut deps.as_mut(),
            vec![
                Uint128::new(10),
                Uint128::new(20),
                Uint128::new(30),
                Uint128::new(40),
                Uint128::new(50),
            ],
        );

        // No winners
        let total_losing_tokens = calculate_total_losing_tokens(&deps.as_ref(), &[]).unwrap();
        assert_eq!(
            total_losing_tokens,
            Uint128::new(10 + 20 + 30 + 40 + 50),
            "Should sum all pots' tokens"
        );
    }

    #[test]
    fn total_losing_tokens_all_winners() {
        let mut deps = mock_dependencies();
        setup_pots(
            &mut deps.as_mut(),
            vec![
                Uint128::new(10),
                Uint128::new(20),
                Uint128::new(30),
                Uint128::new(40),
                Uint128::new(50),
            ],
        );

        // All pots are winners
        let total_losing_tokens =
            calculate_total_losing_tokens(&deps.as_ref(), &[1, 2, 3, 4, 5]).unwrap();
        assert_eq!(
            total_losing_tokens,
            Uint128::zero(),
            "Should not sum any tokens as all pots are winners"
        );
    }

    #[test]
    fn total_losing_tokens_with_player_allocations() {
        let mut deps = mock_dependencies();

        let player1 = Addr::unchecked("player1");
        let player2 = Addr::unchecked("player2");

        // Setup pots and player allocations
        setup_pots_and_allocations(
            &mut deps.as_mut(),
            vec![
                Uint128::new(10),
                Uint128::new(10),
                Uint128::new(10),
                Uint128::new(10),
                Uint128::new(10),
            ],
            vec![
                (1, player1.clone(), Uint128::new(10)), // Player 1 allocates 10 tokens to pot 1
                (2, player1.clone(), Uint128::new(20)), // Player 1 allocates 20 tokens to pot 2
                (1, player2.clone(), Uint128::new(15)), // Player 2 allocates 15 tokens to pot 1
                (3, player2.clone(), Uint128::new(25)), // Player 2 allocates 25 tokens to pot 3
            ],
        );

        // Let's say pot 2 is the winner
        let total_losing_tokens = calculate_total_losing_tokens(&deps.as_ref(), &[2]).unwrap();
        assert_eq!(
            total_losing_tokens,
            Uint128::new(10 + 10 + 10 + 10 + 10 + 10 + 20 + 15 + 25 - 10 - 20), // Exclude pot 2's initial tokens and allocations
            "Should sum tokens from all pots except pot 2"
        );
    }

    // // distribute_tokens

    // #[test]
    // fn distribute_tokens_all_pots_win() {
    //     let mut deps = mock_dependencies();
    //     let player1 = Addr::unchecked("player1");
    //     let player2 = Addr::unchecked("player2");

    //     // Setup pots with initial allocations
    //     setup_pots_and_allocations(
    //         &mut deps.as_mut(),
    //         vec![
    //             Uint128::new(1000000), // Median and Lowest
    //             Uint128::new(2000000), // Highest
    //             Uint128::new(1200000), // Even
    //             Uint128::new(1000000), // Median and Lowest
    //             Uint128::new(1100000), // Prime
    //         ],
    //         vec![
    //             (1, player1.clone(), Uint128::new(1000000)),
    //             (2, player2.clone(), Uint128::new(2000000)),
    //             (3, player1.clone(), Uint128::new(1200000)),
    //             (4, player2.clone(), Uint128::new(1000000)),
    //             (5, player1.clone(), Uint128::new(1100000)),
    //         ],
    //     );

    //     // All pots win, so no redistribution should happen
    //     // Assert that each pot retains its tokens
    //     for pot_id in 1..=5 {
    //         let pot_state = POT_STATES.load(deps.as_mut().storage, pot_id).unwrap();
    //         let expected_tokens = match pot_id {
    //             1 | 4 => Uint128::new(2000000), // 1000000 initial + 1000000 allocated
    //             2 => Uint128::new(4000000),     // 2000000 initial + 2000000 allocated
    //             3 => Uint128::new(2400000),     // 1200000 initial + 1200000 allocated
    //             5 => Uint128::new(2200000),     // 1100000 initial + 1100000 allocated
    //             _ => unreachable!(),
    //         };
    //         assert_eq!(
    //             pot_state.amount, expected_tokens,
    //             "Pot {}'s total tokens should remain unchanged",
    //             pot_id
    //         );
    //     }
    // }

    // #[test]
    // fn distribute_tokens_no_pots_win() {
    //     let mut deps = mock_dependencies();
    //     let player1 = Addr::unchecked("player1");
    //     let player2 = Addr::unchecked("player2");

    //     // Setup pots with initial allocations such that no pot wins
    //     setup_pots_and_allocations(
    //         &mut deps.as_mut(),
    //         vec![1500001u128, 1500003, 1500004, 1500007, 1500009]
    //             .into_iter()
    //             .map(Uint128::new)
    //             .collect(),
    //         vec![
    //             (1, player1.clone(), Uint128::new(3)), // Adjust to maintain odd total
    //             (2, player2.clone(), Uint128::new(3)), // Adjust to maintain odd total
    //             (3, player1.clone(), Uint128::new(3)), // Adjust to maintain odd total
    //             (4, player2.clone(), Uint128::new(3)), // Adjust to maintain odd total
    //             (5, player1.clone(), Uint128::new(3)), // Adjust to maintain odd total
    //         ],
    //     );

    //     // Assert that no pot wins
    //     for pot_id in 1..=5 {
    //         let result = is_winning_pot(&deps.as_ref(), pot_id).unwrap();
    //         assert!(!result, "Pot {} should not be winning.", pot_id);
    //     }

    //     // Since no pots win, no redistribution should occur, and each pot should retain its total tokens.
    //     for pot_id in 1..=5 {
    //         let initial_tokens = match pot_id {
    //             1 => 1500001u128,
    //             2 => 1500003,
    //             3 => 1500004,
    //             4 => 1500007,
    //             5 => 1500009,
    //             _ => unreachable!(),
    //         };
    //         let pot_state = POT_STATES.load(deps.as_mut().storage, pot_id).unwrap();
    //         let expected_tokens = Uint128::new(initial_tokens + 3); // Initial + allocated tokens.
    //         assert_eq!(
    //             pot_state.amount, expected_tokens,
    //             "Pot {}'s total tokens should remain unchanged",
    //             pot_id
    //         );
    //     }
    // }

    #[test]
    fn distribute_tokens_pot_3_wins() {
        let mut deps = mock_dependencies();
        let player1 = Addr::unchecked("player1");
        let player2 = Addr::unchecked("player2");

        // Setup pots with initial allocations
        setup_pots_and_allocations(
            &mut deps.as_mut(),
            vec![
                Uint128::new(1000000),
                Uint128::new(1000000),
                Uint128::new(1000000), // 10 winning tokens
                Uint128::new(1000000),
                Uint128::new(1000000), // 40 losing tokens
            ],
            vec![
                (2, player2.clone(), Uint128::new(2000000)), // Player 2 allocates 20 tokens to pot 2 <- losing
                (3, player1.clone(), Uint128::new(5000000)), // Player 1 allocates 50 tokens to pot 3 <- winning
                (4, player2.clone(), Uint128::new(1500000)), // Player 2 allocates 15 tokens to pot 4 <- losing
            ],
        );

        let result = is_winning_pot(&deps.as_ref(), 3).unwrap();
        assert_eq!(result, true, "Pot 3 should be winning.");

        // Ensure that other pots are not falsely reported as winners
        let result = is_winning_pot(&deps.as_ref(), 1).unwrap();
        assert_eq!(result, false, "Pot 1 should not be winning.");
        let result = is_winning_pot(&deps.as_ref(), 2).unwrap();
        assert_eq!(result, false, "Pot 2 should not be winning.");
        let result = is_winning_pot(&deps.as_ref(), 4).unwrap();
        assert_eq!(result, false, "Pot 4 should not be winning.");
        let result = is_winning_pot(&deps.as_ref(), 5).unwrap();
        assert_eq!(result, false, "Pot 5 should not be winning.");

        // Here we have 75 loosing tokens and 60 winning tokens.
        // So we should distribute 37.5 to the 60. And maintain 37.5 for next game.

        assert_eq!(
            REALLOCATION_FEE_POOL.load(&deps.storage).unwrap(),
            Uint128::zero(),
            "Reallocation fee pool should be 0."
        );

        // Assume pot 3 is the winner, redistribute from pots 1, 2, 4, and 5
        let total_losing_tokens =
            Uint128::new(1000000 + 1000000 + 1000000 + 1000000 + 2000000 + 1500000); // Sum of initial and allocated tokens in losing pots
        let half_losing_tokens = total_losing_tokens.multiply_ratio(1u128, 2u128);
        distribute_tokens(&mut deps.as_mut(), &[3], total_losing_tokens).unwrap();

        // Check that the tokens were redistributed to pot 3
        let pot_state = POT_STATES.load(deps.as_mut().storage, 3).unwrap();
        let expected_tokens_for_pot_3 = Uint128::new(1000000 + 5000000) + half_losing_tokens; // Initial + allocated + redistributed amount for pot 3
        assert_eq!(
            pot_state.amount, expected_tokens_for_pot_3,
            "Pot 3's total tokens should include redistributed amount"
        );

        // Here all 135 tokens have been assigned:
        // 97.5 rounded low to 37 to winners, 37.5 rounded up as idle balance funds for the next game round

        // Check that the tokens were redistributed to pot 1,2,4,5 expecting 0 for all of the losing pots
        let pot_state = POT_STATES.load(deps.as_mut().storage, 1).unwrap();
        let expected_tokens_for_pot_1 = Uint128::new(0); // Initial amount for pot 1
        assert_eq!(
            pot_state.amount, expected_tokens_for_pot_1,
            "Pot 1's total tokens should include initial amount"
        );
        let pot_state = POT_STATES.load(deps.as_mut().storage, 2).unwrap();
        let expected_tokens_for_pot_2 = Uint128::new(0); // Initial amount for pot 2
        assert_eq!(
            pot_state.amount, expected_tokens_for_pot_2,
            "Pot 2's total tokens should include initial amount"
        );
        let pot_state = POT_STATES.load(deps.as_mut().storage, 4).unwrap();
        let expected_tokens_for_pot_4 = Uint128::new(0); // Initial amount for pot 4
        assert_eq!(
            pot_state.amount, expected_tokens_for_pot_4,
            "Pot 4's total tokens should include initial amount"
        );
        let pot_state = POT_STATES.load(deps.as_mut().storage, 5).unwrap();
        let expected_tokens_for_pot_5 = Uint128::new(0); // Initial amount for pot 5
        assert_eq!(
            pot_state.amount, expected_tokens_for_pot_5,
            "Pot 5's total tokens should include initial amount"
        );

        // TODO: Assert contract balance
    }

    // - prepare_next_game

    #[test]
    fn prepare_next_game_works() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let config = GameConfig {
            game_duration: 3600,                                    // 1 hour in seconds
            fee_allocation: 2,                                      // Assuming a 2% allocation fee
            fee_reallocation: 5, // Assuming a 5% reallocation fee
            fee_allocation_address: Addr::unchecked("fee_address"), // An example fee address
            game_denom: "token".to_string(),
            min_bid: Uint128::new(1000000u128),
        };
        GAME_CONFIG.save(deps.as_mut().storage, &config).unwrap();

        // Simulate some initial state for pots and reallocation fee pool
        let initial_pots = vec![10, 10, 10, 10, 10]
            .into_iter()
            .map(Uint128::new)
            .collect();

        setup_pots(&mut deps.as_mut(), initial_pots);
        REALLOCATION_FEE_POOL
            .save(deps.as_mut().storage, &Uint128::new(100))
            .unwrap(); // example value

        // Invoke prepare_next_game
        prepare_next_game(deps.as_mut(), &env, &vec![]).unwrap();

        // Verify GAME_STATE
        let game_state = GAME_STATE.load(deps.as_mut().storage).unwrap();
        assert_eq!(game_state.start_time, env.block.time.seconds() + 1);
        assert_eq!(
            game_state.end_time,
            env.block.time.seconds() + 1 + config.game_duration
        );

        // TODO: Fix this
        // Verify pots have been reset and include the reallocation fee pool
        // let expected_token_per_pot = (deps
        //     .querier
        //     .query_all_balances(&env.contract.address)
        //     .unwrap()
        //     .iter()
        //     .sum::<Coin>()
        //     .amount
        //     + Uint128::new(100))
        // .u128()
        //     / 5;
        // for pot_id in 1..=5 {
        //     let pot_state = POT_STATES.load(deps.as_mut().storage, pot_id).unwrap();
        //     assert_eq!(pot_state, Uint128::new(expected_token_per_pot));
        // }

        // Verify reallocation fee pool reset
        let reallocation_fee_pool = REALLOCATION_FEE_POOL.load(deps.as_mut().storage).unwrap();
        assert_eq!(reallocation_fee_pool, Uint128::zero());
    }
}
