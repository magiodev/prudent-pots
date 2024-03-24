use cosmwasm_std::{attr, BankMsg, Coin, CosmosMsg, DepsMut, Env, MessageInfo, Response, Uint128};

use crate::{
    helpers::is_contract_admin,
    state::{GameConfig, PotState, TokenAllocation, GAME_CONFIG, PLAYER_ALLOCATIONS, POT_STATES},
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
    // Load the game configuration
    let config = GAME_CONFIG.load(deps.storage)?;

    // Check if the correct denom is sent and calculate the total amount
    let total_amount = Uint128::new(0u128);
    for coin in info.funds.iter() {
        if coin.denom == config.game_denom {
            total_amount.checked_add(coin.amount).unwrap();
        } else {
            // Prevent any incorrect denoms or additional coins to be sent
            return Err(ContractError::InvalidFunds {});
        }
    }

    if total_amount == Uint128::zero() {
        return Err(ContractError::NoFunds {});
    }

    // Calculate the allocation fee and the net amount to be allocated to the pot
    let fee = total_amount
        .checked_mul(Uint128::from(config.fee_allocation))
        .unwrap()
        .checked_div(Uint128::from(100u128))
        .unwrap();
    let net_amount = total_amount.checked_sub(fee).unwrap();

    // Update the player's allocations
    PLAYER_ALLOCATIONS.update(
        deps.storage,
        info.sender.clone(),
        |allocations| -> Result<_, ContractError> {
            let mut allocs = allocations.unwrap();
            if let Some(allocation) = allocs.allocations.iter_mut().find(|a| a.pot_id == pot_id) {
                allocation.amount += net_amount;
            } else {
                allocs.allocations.push(TokenAllocation {
                    pot_id,
                    amount: net_amount,
                });
            }
            Ok(allocs)
        },
    )?;

    // Update the pot's state
    POT_STATES.update(
        deps.storage,
        pot_id,
        |pot_state| -> Result<_, ContractError> {
            let state = pot_state.unwrap();
            state.total_tokens.checked_add(net_amount).unwrap();
            Ok(state)
        },
    )?;

    // Send the allocation fee to the team fee address
    let messages = if fee.gt(&Uint128::zero()) {
        vec![CosmosMsg::Bank(BankMsg::Send {
            to_address: config.fee_allocation_address.into_string(),
            amount: vec![Coin {
                denom: config.game_denom.to_string(),
                amount: fee.into(),
            }],
        })]
    } else {
        vec![]
    };

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
    info: MessageInfo,
    from_pot_id: u8,
    to_pot_id: u8,
    amount: Uint128,
) -> Result<Response, ContractError> {
    // Load the player's allocations
    let mut player_allocations = PLAYER_ALLOCATIONS.load(deps.storage, info.sender.clone())?;

    // Check if the player has enough tokens in the from_pot to reallocate
    let from_allocation = player_allocations
        .allocations
        .iter_mut()
        .find(|a| a.pot_id == from_pot_id);
    match from_allocation {
        Some(allocation) if allocation.amount >= amount => {
            // Deduct the amount from the from_pot
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
            let state = pot_state.unwrap();
            let new_total_tokens = state.total_tokens.checked_sub(amount).unwrap();
            Ok(PotState {
                total_tokens: new_total_tokens,
            })
        },
    )?;

    // Update the pot's state for the pot to which the tokens are being reallocated
    POT_STATES.update(
        deps.storage,
        to_pot_id,
        |pot_state| -> Result<_, ContractError> {
            let state = pot_state.unwrap();
            let new_total_tokens = state.total_tokens.checked_add(amount).unwrap();
            Ok(PotState {
                total_tokens: new_total_tokens,
            })
        },
    )?;

    Ok(Response::new().add_attributes(vec![
        attr("method", "execute"),
        attr("action", "reallocate_tokens"),
        attr("player", info.sender.to_string()),
        attr("from_pot_id", from_pot_id.to_string()),
        attr("to_pot_id", to_pot_id.to_string()),
        attr("amount", amount.to_string()),
    ]))
}

pub fn game_end(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    // Implement game end logic here
    Ok(Response::new().add_attributes(vec![
        attr("method", "execute"),
        attr("action", "game_end"),
        attr("sender", info.sender),
        // More attributes representing winner pots, token distribution to winners, next game allocaation, etc.
    ]))
}
