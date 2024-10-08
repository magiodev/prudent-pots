use std::str::FromStr;

use cosmwasm_std::{
    attr, to_json_binary, BankMsg, CosmosMsg, Decimal, DepsMut, Env, MessageInfo, Response, SubMsg,
    Uint128, WasmMsg,
};

use crate::{
    helpers::{
        game_end::{
            calculate_total_losing_tokens, get_distribution_send_msgs, prepare_next_game,
            process_raffle_winner,
        },
        pot::{
            calculate_max_bid, calculate_min_bid, get_winning_pots, set_first_bidder_if_not_set,
            update_player_allocation, update_pot_state,
        },
        validate::{
            extend_game_time, validate_existing_allocation, validate_funds, validate_game_end_time,
            validate_game_time, validate_increase_player_reallocations, validate_is_contract_admin,
            validate_is_contract_admin_game_end, validate_pot_limit_not_exceeded,
        },
    },
    msg::{ReplyMsg, UpdateGameConfig},
    state::{GAME_CONFIG, GAME_STATE, PLAYER_ALLOCATIONS, RAFFLE, REALLOCATION_FEE_POOL},
    ContractError,
};

pub fn update_config(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    update_config: UpdateGameConfig,
) -> Result<Response, ContractError> {
    validate_is_contract_admin(&deps.querier, &env, &info.sender)?;

    let mut game_config = GAME_CONFIG.load(deps.storage)?;

    if let Some(fee) = update_config.fee {
        if fee > 10 {
            return Err(ContractError::InvalidInput {});
        }
        game_config.fee = fee;
    }
    if let Some(fee_reallocation) = update_config.fee_reallocation {
        if fee_reallocation > 50 {
            return Err(ContractError::InvalidInput {});
        }
        game_config.fee_reallocation = fee_reallocation;
    }
    if let Some(fee_address) = update_config.fee_address {
        game_config.fee_address = deps.api.addr_validate(fee_address.as_str())?;
    }
    if let Some(game_denom) = update_config.game_denom {
        game_config.game_denom = game_denom;
    }
    if !game_config
        .game_cw721_addrs
        .iter()
        .eq(update_config.game_cw721_addrs.iter())
    {
        for address in &update_config.game_cw721_addrs {
            deps.api.addr_validate(address.as_str())?;
        }
        game_config.game_cw721_addrs = update_config.game_cw721_addrs;
    }
    if let Some(game_duration) = update_config.game_duration {
        game_config.game_duration = game_duration;
    }
    if let Some(game_duration_epoch) = update_config.game_duration_epoch {
        game_config.game_duration_epoch = game_duration_epoch;
    }
    if let Some(game_extend) = update_config.game_extend {
        if game_extend > game_config.game_duration {
            return Err(ContractError::InvalidInput {});
        }
        game_config.game_extend = game_extend;
    }
    if let Some(game_end_threshold) = update_config.game_end_threshold {
        game_config.game_end_threshold = game_end_threshold;
    }
    if let Some(min_pot_initial_allocation) = update_config.min_pot_initial_allocation {
        game_config.min_pot_initial_allocation = min_pot_initial_allocation;
    }
    if let Some(decay_factor) = update_config.decay_factor {
        if decay_factor.lt(&Decimal::from_str("0.01")?)
            || decay_factor.gt(&Decimal::from_str("0.99")?)
        {
            return Err(ContractError::InvalidInput {});
        }
        game_config.decay_factor = decay_factor;
    }
    if let Some(reallocations_limit) = update_config.reallocations_limit {
        game_config.reallocations_limit = reallocations_limit;
    }
    GAME_CONFIG.save(deps.storage, &game_config)?;

    Ok(Response::new().add_attributes(vec![
        attr("method", "execute"),
        attr("action", "update_config"),
        attr("admin", info.sender),
        attr("config", format!("{:?}", game_config)),
    ]))
}

pub fn allocate_tokens(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    pot_id: u8,
) -> Result<Response, ContractError> {
    let game_config = GAME_CONFIG.load(deps.storage)?;
    let game_state = GAME_STATE.load(deps.storage)?;

    validate_game_time(deps.storage, &env)?;
    let amount = validate_funds(&info.funds, &game_config.game_denom)?;
    validate_pot_limit_not_exceeded(deps.storage, pot_id, amount)?;
    validate_existing_allocation(deps.storage, &info.sender, pot_id)?;

    // Dynamic bid constraints

    // min bid based on current addy so we discount by NFT holding
    let min_bid = calculate_min_bid(&deps.as_ref(), &env, Some(info.sender.to_string()))?;

    // get the originial min bid calculation without taking in account NFT holding discount
    let original_min_bid = calculate_min_bid(&deps.as_ref(), &env, None)?;
    // max bid based on original min bid, so we don't discount by NFT holding
    let max_bid = calculate_max_bid(&deps.as_ref(), original_min_bid)?;
    if amount < min_bid || amount > max_bid {
        return Err(ContractError::BidOutOfRange {
            min: min_bid,
            max: max_bid,
        });
    }
    // we do that here so the extend_count doesnt increase before we evaluate the min max bid amounts
    extend_game_time(deps.storage, &env)?;

    // Update the player's allocation and pot state
    update_player_allocation(deps.storage, &info.sender, pot_id, amount, true)?;
    update_pot_state(deps.storage, pot_id, amount, true)?;

    // Update the first bidder for the current pot_id
    set_first_bidder_if_not_set(deps.storage, pot_id, &info.sender, env.block.time.seconds())?;

    Ok(Response::new().add_attributes(vec![
        attr("method", "execute"),
        attr("action", "allocate_tokens"),
        attr("round_count", game_state.round_count.to_string()),
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
    let game_config = GAME_CONFIG.load(deps.storage)?;
    let game_state = GAME_STATE.load(deps.storage)?;

    if from_pot_id == to_pot_id {
        return Err(ContractError::InvalidPot {});
    }
    validate_game_time(deps.storage, &env)?;
    extend_game_time(deps.storage, &env)?;
    validate_increase_player_reallocations(deps.storage, &info.sender)?;
    validate_existing_allocation(deps.storage, &info.sender, to_pot_id)?;

    // Load and check the player's allocations
    let amount = PLAYER_ALLOCATIONS
        .load(deps.storage, info.sender.to_string())?
        .into_iter()
        .find(|a| a.pot_id == from_pot_id)
        .map_or(Uint128::zero(), |allocation| allocation.amount);

    if amount.is_zero() {
        return Err(ContractError::InsufficientFunds {});
    }

    validate_pot_limit_not_exceeded(deps.storage, to_pot_id, amount)?;

    let fee = amount.multiply_ratio(game_config.fee_reallocation, 100u128);
    let net_amount = amount.checked_sub(fee)?;

    // Deduct the burning fee and update the burning fee pool
    REALLOCATION_FEE_POOL.update(deps.storage, |mut current| -> Result<_, ContractError> {
        current = current.checked_add(fee)?;
        Ok(current)
    })?;

    // Update allocations and pot states using helper functions
    update_player_allocation(deps.storage, &info.sender, from_pot_id, amount, false)?; // sub
    update_player_allocation(deps.storage, &info.sender, to_pot_id, net_amount, true)?; // add
    update_pot_state(deps.storage, from_pot_id, amount, false)?; // sub
    update_pot_state(deps.storage, to_pot_id, net_amount, true)?; // add

    Ok(Response::new().add_attributes(vec![
        attr("method", "execute"),
        attr("action", "reallocate_tokens"),
        attr("round_count", game_state.round_count.to_string()),
        attr("player", info.sender.to_string()),
        attr("from_pot_id", from_pot_id.to_string()),
        attr("to_pot_id", to_pot_id.to_string()),
        attr("amount", amount.to_string()),
        attr("fee", fee.to_string()),
    ]))
}

pub fn game_end(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    new_raffle_cw721_id: Option<String>,
    new_raffle_cw721_addr: Option<String>,
    next_game_start: Option<u64>,
) -> Result<Response, ContractError> {
    validate_game_end_time(deps.storage, &env)?;
    validate_is_contract_admin_game_end(deps.storage, &deps.querier, &env, &info.sender)?;

    // if passed, it should be in the future
    if next_game_start.is_some() && next_game_start.unwrap() <= env.block.time.seconds() {
        return Err(ContractError::InvalidNextGameStart {});
    }

    // Ensure both or neither options are provided
    if new_raffle_cw721_id.is_some() != new_raffle_cw721_addr.is_some() {
        return Err(ContractError::InvalidRaffleNft {});
    }

    // Determine the winning pots and calculate total losing tokens
    let winning_pots = get_winning_pots(deps.storage)?;
    let total_losing_tokens = calculate_total_losing_tokens(deps.storage, &winning_pots)?;

    let mut msgs: Vec<CosmosMsg> = vec![];

    // Process raffle winner and prepare distribution messages
    let process_raffle_winner_resp = process_raffle_winner(
        &deps.as_ref(),
        &env,
        &info.funds,
        &winning_pots,
        new_raffle_cw721_id,
        new_raffle_cw721_addr,
    )?;
    msgs.extend(process_raffle_winner_resp.msgs.clone());

    // Add messages for redistributing tokens from losing to winning pots
    let (send_msgs, treasury_outgoing_tokens) =
        get_distribution_send_msgs(&deps.as_ref(), &winning_pots, total_losing_tokens)?;
    msgs.extend(send_msgs.clone());
    REALLOCATION_FEE_POOL.save(deps.storage, &Uint128::zero())?;

    // Iterate again the msgs generated to know how much tokens effectively we send,
    // as total_losing_tokens contains also next game funds we want to preserve.
    let total_outgoing_raffle: Uint128 = process_raffle_winner_resp
        .msgs
        .iter()
        .filter_map(|msg| {
            if let CosmosMsg::Bank(BankMsg::Send { amount, .. }) = msg {
                Some(amount)
            } else {
                None
            }
        })
        .flatten()
        .map(|coin| coin.amount)
        .sum();
    let total_outgoing_distribution: Uint128 = send_msgs
        .iter()
        .filter_map(|msg| {
            if let CosmosMsg::Bank(BankMsg::Send { amount, .. }) = msg {
                Some(amount)
            } else {
                None
            }
        })
        .flatten()
        .map(|coin| coin.amount)
        .sum();
    let total_outgoing_tokens = total_outgoing_raffle.checked_add(total_outgoing_distribution)?;

    // Reset and prepare for the next game
    let (old_round_count, _new_round_count, old_extend_count) = prepare_next_game(
        deps,
        &env,
        total_outgoing_tokens,
        process_raffle_winner_resp.new_raffle_cw721_id,
        process_raffle_winner_resp.new_raffle_cw721_addr,
        Some(process_raffle_winner_resp.new_raffle_denom_amount),
        next_game_start,
    )?;

    Ok(Response::new()
        .add_messages(msgs)
        .add_submessages(process_raffle_winner_resp.submsgs)
        .add_attributes(vec![
            attr("method", "execute"),
            attr("action", "game_end"),
            attr("round_count", old_round_count.to_string()),
            attr("extend_count", old_extend_count.to_string()),
            attr("winning_pots", format!("{:?}", winning_pots)),
            attr(
                "winning_outgoing_tokens",
                total_outgoing_distribution.checked_sub(treasury_outgoing_tokens)?, // this is just about legacy distribution
            ),
            attr("treasury_outgoing_tokens", treasury_outgoing_tokens),
        ])
        .add_attributes(process_raffle_winner_resp.attributes) // this contains the raffle event attributes including the treasury denom fee split, which is not included above
        .add_attribute("total_outgoing_tokens", total_outgoing_tokens)) // this is the total of distribution + raffle + treasury
}

pub fn update_next_game(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    new_raffle_cw721_id: Option<String>,
    new_raffle_cw721_addr: Option<String>,
    next_game_start: Option<u64>,
) -> Result<Response, ContractError> {
    validate_is_contract_admin(&deps.querier, &env, &info.sender)?;

    let mut submsgs: Vec<SubMsg> = vec![];
    let mut response_attributes = vec![];

    // Handle start time update
    if let Some(start_time) = next_game_start {
        // If next_game_start is passed, it should be in the future
        if start_time <= env.block.time.seconds() {
            return Err(ContractError::InvalidNextGameStart {});
        }

        let game_duration = GAME_CONFIG.load(deps.storage)?.game_duration;
        GAME_STATE.update(deps.storage, |mut game_state| -> Result<_, ContractError> {
            game_state.start_time = start_time;
            game_state.end_time = start_time + game_duration;
            Ok(game_state)
        })?;

        response_attributes.push(attr("next_game_start", start_time.to_string()));
    }

    // Handle raffle NFT update, if a raffle_id is passed we assume we want to set an NFT for the next round's raffle
    if let Some(raffle_id) = &new_raffle_cw721_id {
        // Ensure both or neither options are provided or throw an error
        if new_raffle_cw721_id.is_some() != new_raffle_cw721_addr.is_some() {
            return Err(ContractError::InvalidRaffleNft {});
        }

        // Update the raffle CW721 token ID and address, only if there is no one yet
        RAFFLE.update(deps.storage, |mut raffle| -> Result<_, ContractError> {
            if raffle.cw721_token_id.is_none() {
                raffle.cw721_token_id = Some(raffle_id.clone());
                raffle.cw721_addr = new_raffle_cw721_addr.clone();
            } else {
                // Otherwise throw
                return Err(ContractError::InvalidRaffleNft {});
            }
            Ok(raffle)
        })?;

        // Transfer the NFT
        let transfer_nft_msg = SubMsg::reply_always(
            WasmMsg::Execute {
                contract_addr: new_raffle_cw721_addr.clone().unwrap(),
                msg: to_json_binary(&cw721::Cw721ExecuteMsg::TransferNft {
                    recipient: env.contract.address.to_string(),
                    token_id: raffle_id.to_string(),
                })?,
                funds: vec![],
            },
            ReplyMsg::TransferNft as u64,
        );
        submsgs.push(transfer_nft_msg);
        response_attributes.extend(vec![
            attr("raffle_cw721_addr", new_raffle_cw721_addr.unwrap()),
            attr("raffle_cw721_id", raffle_id),
        ]);
    }

    // Handle raffle funds update, validate_funds to obtain current sent funds
    let game_denom = GAME_CONFIG.load(deps.storage)?.game_denom;
    let total_amount = validate_funds(&info.funds, &game_denom).unwrap_or_default();
    if !total_amount.is_zero() {
        // Update the denom amount for the next raffle incrementing any previous value
        RAFFLE.update(deps.storage, |mut raffle| -> Result<_, ContractError> {
            raffle.denom_amount += total_amount;
            Ok(raffle)
        })?;
        response_attributes.push(attr("raffle_denom_amount", total_amount.to_string()));
        // this is only the new amount sent
    }

    // Throw an error if we did nothing in the previous workflow, this method is meant to be executed for a reason
    if response_attributes.is_empty() {
        return Err(ContractError::InvalidInput {});
    }

    // Create response with dynamic attributes based on the current execution
    let mut response = Response::new()
        .add_attributes(vec![
            attr("method", "execute"),
            attr("action", "update_next_game"),
        ])
        .add_attributes(response_attributes);

    // If submsgs is not empty, append the message to transfer the NFT
    if !submsgs.is_empty() {
        response = response.add_submessages(submsgs);
    }

    Ok(response)
}
