use cosmwasm_std::{attr, DepsMut, Env, MessageInfo, Response, Uint128};

use crate::{
    helpers::{
        calculate_max_bid, calculate_min_bid, calculate_total_losing_tokens, create_fee_message,
        distribute_tokens, is_contract_admin, is_winning_pot, prepare_next_game,
        update_player_allocation, update_pot_state, validate_and_extend_game_time,
        validate_and_sum_funds,
    },
    state::{
        GameConfig, TokenAllocation, GAME_CONFIG, GAME_STATE, PLAYER_ALLOCATIONS, POT_STATES,
        REALLOCATION_FEE_POOL,
    },
    ContractError,
};

pub fn update_config(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    config: GameConfig,
) -> Result<Response, ContractError> {
    is_contract_admin(&deps.querier, &env, &info.sender)?;

    // Implement update config logic here
    Ok(Response::new().add_attributes(vec![
        attr("method", "execute"),
        attr("action", "update_config"),
        attr("admin", info.sender),
        attr("config", format!("{:?}", config)),
    ]))
}

pub fn allocate_tokens(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    pot_id: u8,
) -> Result<Response, ContractError> {
    // Validate the game's end time and extend it if necessary
    validate_and_extend_game_time(deps.storage, &env)?;

    let config = GAME_CONFIG.load(deps.storage)?;

    let total_amount = validate_and_sum_funds(&info, &config.game_denom)?;

    // Implementing dynamic bid constraints
    let min_bid = calculate_min_bid(deps.storage)?;
    let max_bid = calculate_max_bid(deps.storage)?;

    if total_amount < min_bid || total_amount > max_bid {
        return Err(ContractError::BidOutOfRange {
            min: min_bid,
            max: max_bid,
        });
    }

    let fee = total_amount.multiply_ratio(config.fee_allocation, 100u128);
    let net_amount = total_amount.checked_sub(fee).unwrap();

    // Update the player's allocation and pot state
    update_player_allocation(deps.storage, &info.sender, pot_id, net_amount)?;
    update_pot_state(deps.storage, pot_id, net_amount)?;

    // Deducting fee and sending it to the fee allocation address
    let messages = create_fee_message(&config, fee)?;

    Ok(Response::new().add_messages(messages).add_attributes(vec![
        attr("method", "execute"),
        attr("action", "allocate_tokens"),
        attr("player", info.sender),
        attr("pot_id", pot_id.to_string()),
        attr("amount", net_amount.to_string()),
        attr("fee", fee.to_string()),
    ]))
}

pub fn reallocate_tokens(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    from_pot_id: u8,
    to_pot_id: u8,
    mut amount: Uint128,
) -> Result<Response, ContractError> {
    // Validate the game's end time and extend it if necessary
    validate_and_extend_game_time(deps.storage, &env)?;

    let config = GAME_CONFIG.load(deps.storage)?;

    let fee = amount.multiply_ratio(config.fee_reallocation, 100u128);
    amount = amount.checked_sub(fee).unwrap();

    // Deduct the reallocation fee and update the reallocation fee pool
    REALLOCATION_FEE_POOL.update(deps.storage, |mut current| -> Result<_, ContractError> {
        current = current.checked_add(fee).unwrap();
        Ok(current)
    })?;

    // Ensure the reallocation amount is within the set minimum and maximum bid limits
    let min_bid = calculate_min_bid(deps.storage)?; // Convert DepsMut to Deps with as_ref()
    let max_bid = calculate_max_bid(deps.storage)?;

    if amount < min_bid || amount > max_bid {
        return Err(ContractError::BidOutOfRange {
            min: min_bid,
            max: max_bid,
        });
    }

    // Load the player's allocations
    let mut player_allocations = PLAYER_ALLOCATIONS.load(deps.storage, info.sender.clone())?;

    // Check if the player has enough tokens in the from_pot to reallocate
    let from_allocation = player_allocations
        .allocations
        .iter_mut()
        .find(|a| a.pot_id == from_pot_id);
    match from_allocation {
        Some(allocation) if allocation.amount >= amount => {
            allocation.amount = allocation.amount.checked_sub(amount).unwrap();
        }
        _ => return Err(ContractError::InsufficientFunds {}),
    }

    // Add the amount to the to_pot
    let to_allocation = player_allocations
        .allocations
        .iter_mut()
        .find(|a| a.pot_id == to_pot_id);
    match to_allocation {
        Some(allocation) => {
            allocation.amount = allocation.amount.checked_add(amount).unwrap();
        }
        None => {
            player_allocations.allocations.push(TokenAllocation {
                pot_id: to_pot_id,
                amount,
            });
        }
    }

    // Save the updated allocations
    PLAYER_ALLOCATIONS.save(deps.storage, info.sender.clone(), &player_allocations)?;

    // Update the pot's state for the pot from which the tokens are being reallocated
    POT_STATES.update(
        deps.storage,
        from_pot_id,
        |pot_state| -> Result<_, ContractError> {
            let mut state = pot_state.unwrap();
            state.amount = state.amount.checked_sub(amount + fee).unwrap(); // remove it adding the fee to avoid inflating
            Ok(state)
        },
    )?;

    // Update the pot's state for the pot to which the tokens are being reallocated
    POT_STATES.update(
        deps.storage,
        to_pot_id,
        |pot_state| -> Result<_, ContractError> {
            let mut state = pot_state.unwrap();
            state.amount = state.amount.checked_add(amount).unwrap();
            Ok(state)
        },
    )?;

    Ok(Response::new().add_attributes(vec![
        attr("method", "execute"),
        attr("action", "reallocate_tokens"),
        attr("player", info.sender.to_string()),
        attr("from_pot_id", from_pot_id.to_string()),
        attr("to_pot_id", to_pot_id.to_string()),
        attr("amount", amount.to_string()),
        attr("fee", fee.to_string()),
    ]))
}

pub fn game_end(deps: DepsMut, env: Env) -> Result<Response, ContractError> {
    // Verify if the game's end time has been reached
    let game_state = GAME_STATE.load(deps.storage)?;
    if env.block.time.seconds() < game_state.end_time {
        return Err(ContractError::GameStillActive {});
    }

    // Determine the winning pot(s) based on the unique rules for each pot
    let mut winning_pots = Vec::new();
    for pot_id in 1..=5 {
        if is_winning_pot(deps.storage, pot_id)? {
            winning_pots.push(pot_id);
        }
    }

    // Calculate the total amount in losing pots to be redistributed
    let total_losing_tokens = calculate_total_losing_tokens(deps.storage, &winning_pots)?;

    // Redistribute the tokens from losing pots:
    let messages = distribute_tokens(deps.storage, &winning_pots, total_losing_tokens)?;

    // Prepare for the next game
    prepare_next_game(deps, &env, &messages)?;

    // Construct the response with appropriate attributes
    Ok(Response::new().add_messages(messages).add_attributes(vec![
        attr("method", "execute"),
        attr("action", "game_end"),
        attr("winning_pots", format!("{:?}", winning_pots)),
        attr("total_losing_tokens", total_losing_tokens),
    ]))
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies_with_balance, mock_env};
    use cosmwasm_std::{coins, Addr, Env, Uint128};

    use crate::state::{GameState, TokenAllocation};

    fn setup_game_end(deps: &mut DepsMut, env: &mut Env) {
        let game_config = GameConfig {
            game_duration: 3600, // 1 hour
            fee_allocation: 2,
            fee_reallocation: 5,
            fee_allocation_address: Addr::unchecked("fee_address"),
            game_denom: "token".to_string(),
            min_bid: Uint128::new(1000000u128),
        };
        GAME_CONFIG.save(deps.storage, &game_config).unwrap();

        let game_state = GameState {
            start_time: env.block.time.seconds() - 3600, // Started 1 hour ago
            end_time: env.block.time.seconds(),          // Ends now
        };
        GAME_STATE.save(deps.storage, &game_state).unwrap();

        // Initialize the REALLOCATION_FEE_POOL with zero
        REALLOCATION_FEE_POOL
            .save(deps.storage, &Uint128::zero())
            .unwrap();

        // Set up pots with a simulated initial balance
        for pot_id in 1..=5 {
            POT_STATES
                .save(
                    deps.storage,
                    pot_id,
                    &TokenAllocation {
                        pot_id,
                        amount: Uint128::from(1000u128),
                    },
                )
                .unwrap();
        }
    }

    #[test]
    fn test_game_end_with_no_player_allocations() {
        let mut deps = mock_dependencies_with_balance(&coins(5000, "token"));
        let mut env = mock_env();
        setup_game_end(&mut deps.as_mut(), &mut env);

        game_end(deps.as_mut(), env.clone()).unwrap();

        let new_game_state = GAME_STATE.load(deps.as_ref().storage).unwrap();
        assert!(
            new_game_state.start_time > env.block.time.seconds(),
            "New game should start in the future"
        );

        for pot_id in 1..=5 {
            let pot_state = POT_STATES.load(deps.as_ref().storage, pot_id).unwrap();
            assert!(
                pot_state.amount.gt(&Uint128::zero()),
                "Pot {} should have initial tokens for the next game",
                pot_id
            );
        }

        let reallocation_fee_pool = REALLOCATION_FEE_POOL.load(deps.as_ref().storage).unwrap();
        assert_eq!(
            reallocation_fee_pool,
            Uint128::zero(),
            "Reallocation fee pool should be reset to zero for the next game"
        );
    }
}
