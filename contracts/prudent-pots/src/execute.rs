use cosmwasm_std::{attr, DepsMut, Env, MessageInfo, Response, Uint128};

use crate::{
    helpers::{
        calculate_max_bid, calculate_min_bid, calculate_total_losing_tokens,
        check_existing_allocation, get_distribute_bank_msgs, is_contract_admin, is_winning_pot,
        prepare_next_game, update_player_allocation, update_pot_state,
        validate_and_extend_game_time, validate_and_sum_funds, validate_pot_limit_not_exceeded,
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
    let config = GAME_CONFIG.load(deps.storage)?;

    validate_and_extend_game_time(deps.storage, &env)?;
    let amount = validate_and_sum_funds(&info.funds, &config.game_denom)?;
    validate_pot_limit_not_exceeded(deps.storage, pot_id, amount)?;
    check_existing_allocation(deps.storage, &info.sender, pot_id)?;

    // Implementing dynamic bid constraints
    let min_bid = calculate_min_bid(deps.storage)?;
    let max_bid = calculate_max_bid(deps.storage)?;

    if amount < min_bid || amount > max_bid {
        return Err(ContractError::BidOutOfRange {
            min: min_bid,
            max: max_bid,
        });
    }

    // Update the player's allocation and pot state
    update_player_allocation(deps.storage, &info.sender, pot_id, amount)?;
    update_pot_state(deps.storage, pot_id, amount)?;

    Ok(Response::new().add_attributes(vec![
        attr("method", "execute"),
        attr("action", "allocate_tokens"),
        attr("player", info.sender),
        attr("pot_id", pot_id.to_string()),
        attr("amount", amount.to_string()),
    ]))
}

pub fn reallocate_tokens(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    from_pot_id: u8,
    to_pot_id: u8,
) -> Result<Response, ContractError> {
    let config = GAME_CONFIG.load(deps.storage)?;

    if from_pot_id == to_pot_id {
        return Err(ContractError::InvalidPot {});
    }

    validate_and_extend_game_time(deps.storage, &env)?;
    check_existing_allocation(deps.storage, &info.sender, to_pot_id)?;

    // Load the player's allocations
    let mut player_allocations = PLAYER_ALLOCATIONS.load(deps.storage, info.sender.clone())?;

    // Find the allocation for the from_pot and determine the amount to reallocate
    let amount = player_allocations
        .allocations
        .iter()
        .find(|a| a.pot_id == from_pot_id)
        .map_or(Uint128::zero(), |allocation| allocation.amount);

    // Ensure there is an amount to reallocate
    if amount.is_zero() {
        return Err(ContractError::InsufficientFunds {});
    }

    validate_pot_limit_not_exceeded(deps.storage, to_pot_id, amount)?;

    let fee = amount.multiply_ratio(config.fee_reallocation, 100u128);
    let net_amount = amount.checked_sub(fee).unwrap();

    // Deduct the reallocation fee and update the reallocation fee pool
    REALLOCATION_FEE_POOL.update(deps.storage, |mut current| -> Result<_, ContractError> {
        current = current.checked_add(fee).unwrap();
        Ok(current)
    })?;

    // Check if the player has enough tokens in the from_pot to reallocate
    let from_allocation = player_allocations
        .allocations
        .iter_mut()
        .find(|a| a.pot_id == from_pot_id);
    match from_allocation {
        Some(allocation) if allocation.amount >= amount => {
            allocation.amount = Uint128::zero();
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
            allocation.amount = allocation.amount.checked_add(net_amount).unwrap();
        }
        None => {
            player_allocations.allocations.push(TokenAllocation {
                pot_id: to_pot_id,
                amount: net_amount,
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
            state.amount = state.amount.checked_sub(amount).unwrap(); // remove it adding the fee to avoid inflating
            Ok(state)
        },
    )?;

    // Update the pot's state for the pot to which the tokens are being reallocated
    POT_STATES.update(
        deps.storage,
        to_pot_id,
        |pot_state| -> Result<_, ContractError> {
            let mut state = pot_state.unwrap();
            state.amount = state.amount.checked_add(net_amount).unwrap();
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

    // Redistribute the tokens from losing to winning pots, and also winning pots amount with users' allocations based on contributors shares:
    let bank_msgs = get_distribute_bank_msgs(deps.storage, &winning_pots, total_losing_tokens)?;

    // Prepare for the next game
    prepare_next_game(deps, &env, &bank_msgs)?;

    // Construct the response with appropriate attributes
    Ok(Response::new().add_messages(bank_msgs).add_attributes(vec![
        attr("method", "execute"),
        attr("action", "game_end"),
        attr("winning_pots", format!("{:?}", winning_pots)),
        attr("total_losing_tokens", total_losing_tokens),
    ]))
}
