use cosmwasm_std::{
    attr, Addr, BankMsg, Coin, CosmosMsg, DepsMut, Env, MessageInfo, Response, StdResult, Storage,
    Uint128,
};

use crate::{
    helpers::{
        calculate_total_losing_tokens, get_all_token_counts, is_contract_admin, is_winning_pot,
        prepare_next_game, redistribute_losing_tokens,
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
    info: MessageInfo,
    pot_id: u8,
) -> Result<Response, ContractError> {
    let config = GAME_CONFIG.load(deps.storage)?;

    let total_amount = info.funds.iter().fold(Uint128::zero(), |acc, coin| {
        if coin.denom == config.game_denom {
            acc.checked_add(coin.amount).unwrap()
        } else {
            acc
        }
    });

    if total_amount.is_zero() {
        return Err(ContractError::NoFunds {});
    }

    // Implementing dynamic bid constraints
    let min_bid = calculate_min_bid(&deps)?;
    let max_bid = calculate_max_bid(&deps)?;

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

// Helper to calculate the average tokens across all pots
fn calculate_average_tokens(deps: &DepsMut) -> StdResult<Uint128> {
    let pots = get_all_token_counts(deps)?;
    let total: Uint128 = pots.iter().sum();
    Ok(total.checked_div(Uint128::from(pots.len() as u128))?)
}

// Helper to calculate the minimum bid based on the game's current state
fn calculate_min_bid(deps: &DepsMut) -> StdResult<Uint128> {
    let average_tokens = calculate_average_tokens(deps)?;
    // Set minimum bid as the average tokens in pots
    Ok(average_tokens)
}

// Helper to calculate the maximum bid based on the game's current state
fn calculate_max_bid(deps: &DepsMut) -> StdResult<Uint128> {
    let average_tokens = calculate_average_tokens(deps)?;
    // Set maximum bid as double the average tokens in pots
    Ok(average_tokens.checked_mul(Uint128::from(2u128))?)
}

// Helper to update the player's allocation
fn update_player_allocation(
    storage: &mut dyn Storage,
    player: &Addr,
    pot_id: u8,
    amount: Uint128,
) -> Result<(), ContractError> {
    PLAYER_ALLOCATIONS.update(
        storage,
        player.clone(),
        |allocations| -> Result<_, ContractError> {
            let mut allocs = allocations.unwrap();
            if let Some(allocation) = allocs.allocations.iter_mut().find(|a| a.pot_id == pot_id) {
                allocation.amount.checked_add(amount).unwrap();
            } else {
                allocs.allocations.push(TokenAllocation { pot_id, amount });
            }
            Ok(allocs)
        },
    )?;
    Ok(())
}

// Helper to update the pot's state
fn update_pot_state(
    storage: &mut dyn Storage,
    pot_id: u8,
    amount: Uint128,
) -> Result<(), ContractError> {
    POT_STATES.update(storage, pot_id, |pot_state| -> Result<_, ContractError> {
        let mut state = pot_state.unwrap();
        state = state.checked_add(amount).unwrap();
        Ok(state)
    })?;
    Ok(())
}

// Helper to create a bank message for the fee transaction
fn create_fee_message(config: &GameConfig, fee: Uint128) -> StdResult<Vec<CosmosMsg>> {
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

pub fn reallocate_tokens(
    deps: DepsMut,
    info: MessageInfo,
    from_pot_id: u8,
    to_pot_id: u8,
    mut amount: Uint128,
) -> Result<Response, ContractError> {
    let config = GAME_CONFIG.load(deps.storage)?;

    let fee = amount.multiply_ratio(config.fee_reallocation, 100u128);
    amount = amount.checked_sub(fee).unwrap();

    // Deduct the reallocation fee and update the reallocation fee pool
    REALLOCATION_FEE_POOL.update(deps.storage, |mut current| -> Result<_, ContractError> {
        current = current.checked_add(fee).unwrap();
        Ok(current)
    })?;

    // Ensure the reallocation amount is within the set minimum and maximum bid limits
    let min_bid = calculate_min_bid(&deps)?;
    let max_bid = calculate_max_bid(&deps)?;

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
            state = state.checked_sub(amount).unwrap();
            Ok(state)
        },
    )?;

    // Update the pot's state for the pot to which the tokens are being reallocated
    POT_STATES.update(
        deps.storage,
        to_pot_id,
        |pot_state| -> Result<_, ContractError> {
            let mut state = pot_state.unwrap();
            state = state.checked_add(amount).unwrap();
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

pub fn game_end(mut deps: DepsMut, env: Env) -> Result<Response, ContractError> {
    // Verify if the game's end time has been reached
    let game_state = GAME_STATE.load(deps.storage)?;
    if env.block.time.seconds() < game_state.end_time {
        return Err(ContractError::GameStillActive {});
    }

    // Determine the winning pot(s) based on the unique rules for each pot
    let mut winning_pots = Vec::new();
    for pot_id in 1..=5 {
        if is_winning_pot(&deps, pot_id)? {
            winning_pots.push(pot_id);
        }
    }

    // Calculate the total amount in losing pots to be redistributed
    let total_losing_tokens = calculate_total_losing_tokens(&deps.as_ref(), &winning_pots)?;

    // Redistribute the tokens from losing pots:
    redistribute_losing_tokens(&mut deps, &winning_pots, total_losing_tokens)?;

    // Prepare for the next game
    prepare_next_game(&mut deps, &env)?;

    // Construct the response with appropriate attributes
    Ok(Response::new().add_attributes(vec![
        attr("method", "execute"),
        attr("action", "game_end"),
        attr("winning_pots", format!("{:?}", winning_pots)),
        attr("total_losing_tokens", total_losing_tokens),
    ]))
}
