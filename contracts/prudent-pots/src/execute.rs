use cosmwasm_std::{attr, Addr, BankMsg, CosmosMsg, DepsMut, Env, MessageInfo, Response, Uint128};

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
            validate_and_extend_game_time, validate_existing_allocation, validate_funds,
            validate_game_end_time, validate_is_contract_admin,
            validate_is_contract_admin_game_end, validate_pot_limit_not_exceeded,
        },
    },
    state::{GAME_CONFIG, GAME_STATE, PLAYER_ALLOCATIONS, REALLOCATION_FEE_POOL},
    ContractError,
};

pub fn update_config(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    fee: Option<u64>,
    fee_reallocation: Option<u64>,
    fee_address: Option<Addr>,
    game_denom: Option<String>,
    game_cw721_addrs: Vec<Addr>, // this is the cw721 collection addy we use as optional raffle prize
    game_duration: Option<u64>,
    game_extend: Option<u64>,
    game_end_threshold: Option<u64>,
    min_pot_initial_allocation: Option<Uint128>,
    decay_factor: Option<Uint128>, // i.e. 95 as 95%
) -> Result<Response, ContractError> {
    validate_is_contract_admin(&deps.querier, &env, &info.sender)?;

    let mut game_config = GAME_CONFIG.load(deps.storage)?;

    if let Some(fee) = fee {
        if fee > 10 {
            return Err(ContractError::InvalidInput {});
        }
        game_config.fee = fee;
    }
    if let Some(fee_reallocation) = fee_reallocation {
        if fee_reallocation > 50 {
            return Err(ContractError::InvalidInput {});
        }
        game_config.fee_reallocation = fee_reallocation;
    }
    if let Some(fee_address) = fee_address {
        game_config.fee_address = deps.api.addr_validate(fee_address.as_str())?;
    }
    if let Some(game_denom) = game_denom {
        game_config.game_denom = game_denom;
    }
    if !game_config
        .game_cw721_addrs
        .iter()
        .eq(game_cw721_addrs.iter())
    {
        for address in &game_cw721_addrs {
            deps.api.addr_validate(address.as_str())?;
        }
        game_config.game_cw721_addrs = game_cw721_addrs;
    }
    if let Some(game_duration) = game_duration {
        game_config.game_duration = game_duration;
    }
    if let Some(game_extend) = game_extend {
        if game_extend > game_config.game_duration {
            return Err(ContractError::InvalidInput {});
        }
        game_config.game_extend = game_extend;
    }
    if let Some(game_end_threshold) = game_end_threshold {
        game_config.game_end_threshold = game_end_threshold;
    }
    if let Some(min_pot_initial_allocation) = min_pot_initial_allocation {
        game_config.min_pot_initial_allocation = min_pot_initial_allocation;
    }
    if let Some(decay_factor) = decay_factor {
        if decay_factor.lt(&Uint128::new(50u128)) || decay_factor.gt(&Uint128::new(99u128)) {
            return Err(ContractError::InvalidInput {});
        }
        game_config.decay_factor = decay_factor;
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

    validate_and_extend_game_time(deps.storage, &env)?;
    let amount = validate_funds(&info.funds, &game_config.game_denom)?;
    validate_pot_limit_not_exceeded(deps.storage, pot_id, amount)?;
    validate_existing_allocation(deps.storage, &info.sender, pot_id)?;

    // Dynamic bid constraints
    let min_bid = calculate_min_bid(&deps.as_ref(), Some(info.sender.to_string()))?;
    let max_bid = calculate_max_bid(&deps.as_ref())?;
    if amount < min_bid || amount > max_bid {
        return Err(ContractError::BidOutOfRange {
            min: min_bid,
            max: max_bid,
        });
    }

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

    validate_and_extend_game_time(deps.storage, &env)?;
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
) -> Result<Response, ContractError> {
    validate_game_end_time(deps.storage, &env)?;
    validate_is_contract_admin_game_end(deps.storage, &deps.querier, &env, &info.sender)?;

    // Ensure both or neither options are provided
    if new_raffle_cw721_id.is_some() != new_raffle_cw721_addr.is_some() {
        // TODO: Decide if we want the prizes to be part of whitelisted cw721_addrs in GameConfig,
        // or if the whitelist should jsut be the ones that gives minBid discount eligibility
        return Err(ContractError::InvalidRaffleNft {});
    }

    // Determine the winning pots and calculate total losing tokens
    let winning_pots = get_winning_pots(deps.storage)?;
    let total_losing_tokens = calculate_total_losing_tokens(deps.storage, &winning_pots)?;

    let mut msgs: Vec<CosmosMsg> = vec![];

    // Process raffle winner and prepare distribution messages
    let (
        raffle_msgs,    // bank sends
        raffle_submsgs, // nft transfer
        raffle_response_attributes,
        new_raffle_denom_amount,
        updated_new_raffle_cw721_id,
        updated_new_raffle_cw721_addr,
    ) = process_raffle_winner(
        &deps.as_ref(),
        &env,
        &info.funds,
        &winning_pots,
        new_raffle_cw721_id,
        new_raffle_cw721_addr,
    )?;
    msgs.extend(raffle_msgs.clone());

    // Add messages for redistributing tokens from losing to winning pots
    let (send_msgs, treasury_outgoing_tokens) =
        get_distribution_send_msgs(&deps.as_ref(), &winning_pots, total_losing_tokens)?;
    msgs.extend(send_msgs.clone());

    // Iterate again the msgs generated to know how much tokens effectively we send,
    // as total_losing_tokens contains also next game funds we want to preserve.
    // TODO: Make this an helper function
    let total_outgoing_raffle: Uint128 = raffle_msgs
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
        updated_new_raffle_cw721_id,
        updated_new_raffle_cw721_addr,
        Some(new_raffle_denom_amount),
    )?;

    Ok(Response::new()
        .add_messages(msgs)
        .add_submessages(raffle_submsgs)
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
        .add_attributes(raffle_response_attributes) // this contains the raffle event attributes including the treasury denom fee split, which is not included above
        .add_attribute("total_outgoing_tokens", total_outgoing_tokens)) // this is the total of distribution + raffle + treasury
}
