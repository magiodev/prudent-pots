use cosmwasm_std::{
    attr, coins, to_json_binary, Attribute, BankMsg, Coin, CosmosMsg, Deps, DepsMut, Env, Storage,
    SubMsg, Uint128, WasmMsg,
};

use crate::{
    msg::ReplyMsg,
    state::{
        GameConfig, GameState, Raffle, TokenAllocation, FIRST_BIDDER, GAME_CONFIG, GAME_STATE,
        PLAYER_ALLOCATIONS, PLAYER_REALLOCATIONS, POT_STATES, RAFFLE, REALLOCATION_FEE_POOL,
    },
    ContractError,
};

use super::validate::{validate_funds, validate_pot_initial_amount};

// Helper to prepare for the next game
pub fn prepare_next_game(
    deps: DepsMut,
    env: &Env,
    total_outgoing_tokens: Uint128,
    raffle_cw721_token_id: Option<String>,
    raffle_cw721_addr: Option<String>,
    raffle_denom_amount: Option<Uint128>,
) -> Result<(u64, u64, u32), ContractError> {
    let config = GAME_CONFIG.load(deps.storage)?;
    let game_state = GAME_STATE.may_load(deps.storage)?.unwrap_or_default(); // may load due instantiate invoke

    // Start the next game 1 second in the future
    let game_duration = config.game_duration;
    let next_game_start = env.block.time.seconds() + 1;
    let next_game_end = next_game_start + game_duration;

    // Validate game start and end times
    if next_game_start >= next_game_end {
        return Err(ContractError::InvalidInput {});
    }

    // Safely increment round_count
    let new_round_count = game_state
        .round_count
        .checked_add(1)
        .ok_or(ContractError::InvalidInput {})?;

    // Update the game state with new values
    let new_game_state = GameState {
        start_time: next_game_start,
        end_time: next_game_end,
        round_count: new_round_count,
        extend_count: 0,
    };
    GAME_STATE.save(deps.storage, &new_game_state)?;

    // Reset player allocations, player reallocations and first bidder states for the next game
    PLAYER_ALLOCATIONS.clear(deps.storage);
    PLAYER_REALLOCATIONS.clear(deps.storage);
    FIRST_BIDDER.clear(deps.storage);

    // Start initial tokens allocation workflow by querying the contract balance
    let net_contract_balance = deps
        .querier
        .query_balance(&env.contract.address, &config.game_denom)?
        .amount
        .checked_sub(total_outgoing_tokens)? // Subtract outgoing tokens from the total tokens
        .checked_sub(raffle_denom_amount.unwrap_or_default())?; // Subtract the new amount sent in this tx as info.funds reserved for next round denom raffle prize

    // Calculate the initial tokens for each pot after subtracting outgoing tokens and next round game_denom raffle prize amount.
    let initial_tokens_per_pot = net_contract_balance.checked_div(Uint128::from(5u128))?; // Divide the result by 5 to get tokens per pot

    // This will make the restart fail till someone funds the contract balance via bank send
    validate_pot_initial_amount(&config.min_pot_initial_allocation, &net_contract_balance)?;

    // Distribute the initial tokens to the pots for the next game
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

    // Save new raffle prizes for next game
    RAFFLE.save(
        deps.storage,
        &Raffle {
            cw721_token_id: raffle_cw721_token_id,
            cw721_addr: raffle_cw721_addr,
            denom_amount: raffle_denom_amount.unwrap_or_default(),
        },
    )?;

    Ok((
        game_state.round_count,
        new_game_state.round_count,
        game_state.extend_count,
    ))
}

/// Compute the raffle winner based on the total tokens allocated among the winning pots.
pub fn get_raffle_winner(
    storage: &dyn Storage,
    winning_pots: &Vec<u8>,
) -> Result<Option<String>, ContractError> {
    let mut max_total = Uint128::zero();
    let mut winner: Option<String> = None;

    // TODO: Early return if there is not raffle.denom_amount nor raffle.cw721_id

    // Traverse all player allocations
    let all_allocations =
        PLAYER_ALLOCATIONS.range(storage, None, None, cosmwasm_std::Order::Ascending);
    for item in all_allocations {
        let (addr, player_allocations) = item?;

        let total_in_winning_pots: Uint128 = player_allocations
            .iter()
            .filter(|allocation| winning_pots.contains(&allocation.pot_id))
            .map(|allocation| allocation.amount)
            .sum();

        // Update the totals and check for the highest
        if total_in_winning_pots > max_total {
            max_total = total_in_winning_pots;
            winner = Some(addr.to_string());
        } else if total_in_winning_pots == max_total {
            // In case of a tie, find the earliest bidder among the winning pots
            let mut earliest_bid_time = u64::MAX;
            // TODO: handle unwrap safely, but I think this should never happen
            let mut current_winner = winner;

            for &pot_id in winning_pots.iter() {
                // if there is any first bidder
                if let Ok(fb) = FIRST_BIDDER.load(storage, pot_id) {
                    // if the current user is first better, and its earlier than the previously iterated one among wining pots
                    if fb.time < earliest_bid_time && fb.bidder == addr {
                        // so the new earliest is the current one
                        earliest_bid_time = fb.time;
                        current_winner = Some(fb.bidder); // we have a new winner
                    } else if fb.time == earliest_bid_time && fb.bidder == addr {
                        // otherwise if its equal, meaning we have another tie situation,
                        // set it to None and DO NOT distribute the raffle this round!
                        current_winner = None
                    }
                }
            }

            winner = current_winner;
        }
    }

    // If no allocations to winning pots were made, or all were zero, there's no raffle winner
    if max_total == Uint128::zero() {
        Ok(None)
    } else {
        Ok(winner)
    }
}

// Helper function to handle raffle logic distribution to winner
pub fn process_raffle_winner(
    deps: &Deps,
    env: &Env,
    funds: &Vec<Coin>,
    winning_pots: &Vec<u8>,
    mut new_raffle_cw721_id: Option<String>,
    mut new_raffle_cw721_addr: Option<String>,
) -> Result<
    (
        Vec<CosmosMsg>,
        Vec<SubMsg>,
        Vec<Attribute>,
        Uint128,
        Option<String>,
        Option<String>,
    ),
    ContractError,
> {
    let game_config = GAME_CONFIG.load(deps.storage)?;
    let raffle = RAFFLE.load(deps.storage)?;

    // TODO: Early return here if there is no raffle
    // if raffle.nft_id && addr is none and denom_prize is_zero() return all default values.

    let mut msgs = Vec::new();
    let mut submsgs = Vec::new();
    let mut raffle_response_attributes = vec![];

    // this is common for yes_raffle and no_raffle scenarios
    let mut new_raffle_denom_amount =
        validate_funds(funds, &game_config.game_denom).unwrap_or_default();

    let raffle_winner = get_raffle_winner(deps.storage, &winning_pots)?;

    match raffle_winner {
        Some(recipient) => {
            if let Some(token_id) = &raffle.cw721_token_id {
                let cw721_addr = raffle.cw721_addr.unwrap();
                let transfer_nft_msg = SubMsg::reply_always(
                    WasmMsg::Execute {
                        contract_addr: cw721_addr.to_string(),
                        msg: to_json_binary(&cw721::Cw721ExecuteMsg::TransferNft {
                            recipient: recipient.to_string(),
                            token_id: token_id.to_string(),
                        })?,
                        funds: vec![],
                    },
                    ReplyMsg::GameEnd as u64,
                );
                submsgs.push(transfer_nft_msg);
                // Append attributes
                raffle_response_attributes.extend(vec![
                    attr("raffle_winner", recipient.to_string()),
                    attr("raffle_outgoing_nft_addr", cw721_addr),
                    attr("raffle_outgoing_nft_id", token_id),
                ]);
            }

            let (prize_to_distribute, prize_to_treasury) = get_raffle_denom_prize_amounts(deps)?;
            if !prize_to_distribute.is_zero() {
                let send_msg = CosmosMsg::Bank(BankMsg::Send {
                    to_address: recipient.to_string(),
                    amount: coins(prize_to_distribute.u128(), game_config.game_denom.clone()),
                });
                msgs.push(send_msg);
                // Append attributes
                raffle_response_attributes.extend(vec![attr(
                    "raffle_outgoing_tokens_winner",
                    recipient.to_string(),
                )]);
            }
            if !prize_to_treasury.is_zero() {
                let send_msg = CosmosMsg::Bank(BankMsg::Send {
                    to_address: game_config.fee_address.to_string(),
                    amount: coins(prize_to_treasury.u128(), game_config.game_denom.clone()),
                });
                msgs.push(send_msg);
                // Append attributes
                raffle_response_attributes.extend(vec![attr(
                    "raffle_outgoing_tokens_treasury",
                    game_config.fee_address.to_string(),
                )]);
            }
        }
        None => {
            // Scenarios are:
            // - Admin sends new NFT, but old is unsent (ok)
            // - Admin sends new NFT, but old is sent (ok)
            // - Admin do not send new NFT, old is sent (ok)
            // - Admin do not send new NFT, but old is unsent (ok)
            // If there was a not-won Raffle NFT, and admin is trying to send a new one, throw error.
            if raffle.cw721_token_id.is_some() && new_raffle_cw721_id.is_some() {
                // we dont want any new NFT from admin if the previous one is unwon
                return Err(ContractError::PreviousRaffleNftIsUnwon {});
            }
            // if there is still the previous one, override the value

            // if there is still the previous one
            if raffle.cw721_token_id.is_some() {
                new_raffle_cw721_id = raffle.cw721_token_id;
                new_raffle_cw721_addr = raffle.cw721_addr;
            }
            // else we set either the new one already passed from above, or None

            // here for denom prize we increment the old value (possibly 0 or positive) with the new info.funds sent
            new_raffle_denom_amount = new_raffle_denom_amount.checked_add(raffle.denom_amount)?;
        }
    }

    // Send the new NFTs for the next raffle prize if any to process
    // TODO: Check if new_raffle_cw721_id is different from the old one and ensure the address is valid
    // if new_raffle_cw721_id != raffle.cw721_token_id || new_raffle_cw721_addr != raffle.cw721_addr {}
    if let (Some(cw721_id), Some(cw721_addr)) =
        (new_raffle_cw721_id.clone(), new_raffle_cw721_addr.clone())
    {
        let transfer_nft_msg = SubMsg::reply_always(
            WasmMsg::Execute {
                contract_addr: cw721_addr,
                msg: to_json_binary(&cw721::Cw721ExecuteMsg::TransferNft {
                    recipient: env.contract.address.to_string(),
                    token_id: cw721_id,
                })?,
                funds: vec![],
            },
            ReplyMsg::GameEnd as u64,
        );
        submsgs.push(transfer_nft_msg);
    }

    Ok((
        msgs,
        submsgs,
        raffle_response_attributes,
        new_raffle_denom_amount,
        new_raffle_cw721_id,
        new_raffle_cw721_addr,
    ))
}

/// Helper to calculate the prize amount for raffle distribution based on game extensions.
pub fn get_raffle_denom_prize_amounts(deps: &Deps) -> Result<(Uint128, Uint128), ContractError> {
    let game_config: GameConfig = GAME_CONFIG.load(deps.storage)?;
    let game_state: GameState = GAME_STATE.load(deps.storage)?;
    let raffle: Raffle = RAFFLE.load(deps.storage)?;

    // Apply the decay factor iteratively based on extend_count
    let mut prize_percentage = Uint128::from(100u128); // Starting from 100%
    for _ in 0..game_state.extend_count {
        prize_percentage =
            prize_percentage.multiply_ratio(game_config.decay_factor, Uint128::from(100u128));
    }

    let distributed_prize = raffle
        .denom_amount
        .multiply_ratio(prize_percentage, Uint128::from(100u128));
    let remaining_prize = raffle.denom_amount.checked_sub(distributed_prize)?;

    Ok((distributed_prize, remaining_prize))
}

pub fn get_distribution_send_msgs(
    deps: &Deps,
    winning_pots: &[u8],
    total_losing_tokens: Uint128,
) -> Result<(Vec<CosmosMsg>, Uint128), ContractError> {
    let game_config = GAME_CONFIG.load(deps.storage)?;
    let total_distribution_amount = total_losing_tokens.multiply_ratio(1u128, 2u128);

    let mut pot_contributions: Vec<Uint128> = vec![Uint128::zero(); 5]; // Assumes 5 pots
    let mut total_winning_tokens = Uint128::zero();

    // Calculate total token amounts for each winning pot and store them
    for &pot_id in winning_pots {
        if has_player_allocations(deps.storage, pot_id)? {
            let pot_state = POT_STATES.load(deps.storage, pot_id)?;
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
        let pot_state = POT_STATES.load(deps.storage, pot_id)?;
        let pot_total_distribution_amount = pot_share + pot_state.amount;
        let fee = pot_total_distribution_amount.multiply_ratio(game_config.fee, 100u128);
        total_fee += fee;
        let net_distribution_amount = pot_total_distribution_amount.checked_sub(fee)?;

        distribute_tokens_to_players(
            deps.storage,
            &game_config,
            pot_id,
            net_distribution_amount,
            &mut messages,
        )?;
    }

    // Send reallocation_fee_pool amount to treasury, we will reset it later on to avoid passing DepsMut state here.
    let reallocation_fee_pool = REALLOCATION_FEE_POOL.load(deps.storage)?;
    if !reallocation_fee_pool.is_zero() {
        messages.push(CosmosMsg::Bank(BankMsg::Send {
            to_address: game_config.fee_address.to_string(),
            amount: coins(reallocation_fee_pool.into(), game_config.game_denom.clone()),
        }))
    }

    // Deduct the total fee and add to messages
    if !total_fee.is_zero() {
        messages.push(CosmosMsg::Bank(BankMsg::Send {
            to_address: game_config.fee_address.to_string(),
            amount: vec![Coin {
                denom: game_config.game_denom,
                amount: total_fee,
            }],
        }));
    }

    Ok((
        messages,
        total_fee.checked_add(reallocation_fee_pool)?, // treasury_outgoing_tokens
    ))
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

        // Check if the pot is a losing pot, or a winning pot without allocations
        if !winning_pots.contains(&pot_id) || !has_player_allocations(storage, pot_id)? {
            total_losing_tokens = total_losing_tokens.checked_add(pot_state.amount)?;
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
            .iter()
            .any(|a| a.pot_id == pot_id && !a.amount.is_zero())
        // Check for non-zero amounts
        {
            return Ok(true);
        }
    }

    Ok(false)
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
        .flat_map(|(_, allocations)| allocations.iter()) // Flatten the structure to iterate over all allocations.
        .filter(|allocation| allocation.pot_id == pot_id && !allocation.amount.is_zero()) // Filter for allocations to the specific pot that are non-zero.
        .map(|allocation| allocation.amount)
        .sum();

    // Early return if there are no contributions to prevent division by zero in later calculations.
    if total_player_contributions.is_zero() {
        return Ok(());
    }

    // Loop through all player allocations to distribute the net amount based on each player's contribution to the pot.
    for (addr, allocations) in player_allocations {
        for allocation in &allocations {
            // Check if the allocation belongs to the current pot and is not zero.
            if allocation.pot_id == pot_id && !allocation.amount.is_zero() {
                // Calculate the share for this player based on their contribution relative to the total contributions.
                let player_share = net_distribution_amount
                    .multiply_ratio(allocation.amount, total_player_contributions);
                // Create a bank message to send the player's share of tokens and push it to the messages vector.
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

    Ok(())
}
