#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult, Uint128,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::execute::{allocate_tokens, game_end, reallocate_tokens, update_config};
use crate::helpers::game_end::prepare_next_game;
use crate::helpers::validate::{validate_funds, validate_pot_initial_amount};
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg, ReplyMsg};
use crate::query::{
    query_all_players_allocations, query_bid_range, query_game_config, query_game_state,
    query_player_allocations, query_player_reallocations, query_pot_state, query_pots_state,
    query_raffle, query_raffle_denom_split, query_raffle_winner, query_reallocation_fee_pool,
    query_winning_pots,
};
use crate::reply::game_end_reply;
use crate::state::{GameConfig, GAME_CONFIG, OLD_GAME_CONFIG, REALLOCATION_FEE_POOL};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:prudent-pots";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    // Validate game_config fields and initial funds
    if msg.config.fee > 100 || msg.config.fee_reallocation > 100 {
        return Err(ContractError::InvalidInput {});
    }
    if msg.config.game_duration == 0 {
        return Err(ContractError::InvalidInput {});
    }
    if msg.config.min_pot_initial_allocation.is_zero() {
        return Err(ContractError::InvalidInput {});
    }

    // On instantiation there is no raffle. All funds are always for the first no raffled round.
    let total_amount = validate_funds(&info.funds, &msg.config.game_denom)?;
    validate_pot_initial_amount(&msg.config.min_pot_initial_allocation, &total_amount)?;

    GAME_CONFIG.save(deps.storage, &msg.config)?;
    REALLOCATION_FEE_POOL.save(deps.storage, &Uint128::zero())?;

    // Initialize game state and pots for the next game
    prepare_next_game(deps, &env, Uint128::zero(), None, None, None)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("action", "initialize_game")
        .add_attribute("config", format!("{:?}", msg.config)))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::UpdateConfig { config } => update_config(deps, env, info, config),
        ExecuteMsg::AllocateTokens { pot_id } => allocate_tokens(deps, env, info, pot_id),
        ExecuteMsg::ReallocateTokens {
            from_pot_id,
            to_pot_id,
        } => reallocate_tokens(deps, env, info, from_pot_id, to_pot_id),
        ExecuteMsg::GameEnd {
            raffle_cw721_token_id,
            raffle_cw721_token_addr,
        } => game_end(
            deps,
            env,
            info,
            raffle_cw721_token_id,
            raffle_cw721_token_addr,
        ),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(_deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    match msg.id.into() {
        ReplyMsg::GameEnd {} => game_end_reply(msg.result),
        _ => Err(ContractError::UnknownReply {}),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GameConfig {} => to_json_binary(&query_game_config(deps)?),
        QueryMsg::GameState {} => to_json_binary(&query_game_state(deps)?),
        QueryMsg::BidRange { address } => to_json_binary(&query_bid_range(deps, env, address)?),
        QueryMsg::PotState { pot_id } => to_json_binary(&query_pot_state(deps, pot_id)?),
        QueryMsg::PotsState {} => to_json_binary(&query_pots_state(deps)?),
        QueryMsg::WinningPots {} => to_json_binary(&query_winning_pots(deps)?),
        QueryMsg::PlayerAllocations { address } => {
            to_json_binary(&query_player_allocations(deps, address)?)
        }
        QueryMsg::PlayerReallocations { address } => {
            to_json_binary(&query_player_reallocations(deps, address)?)
        }
        QueryMsg::AllPlayersAllocations {} => to_json_binary(&query_all_players_allocations(deps)?),
        QueryMsg::ReallocationFeePool {} => to_json_binary(&query_reallocation_fee_pool(deps)?),
        QueryMsg::Raffle {} => to_json_binary(&query_raffle(deps)?),
        QueryMsg::RaffleWinner {} => to_json_binary(&query_raffle_winner(deps)?),
        QueryMsg::RaffleDenomSplit {} => to_json_binary(&query_raffle_denom_split(deps)?),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, msg: MigrateMsg) -> Result<Response, ContractError> {
    // load the old game config
    let old_game_config = OLD_GAME_CONFIG.load(deps.storage)?;

    // Save new GameConfig at game_config_v2 storage key
    GAME_CONFIG.save(
        deps.storage,
        &GameConfig {
            fee: old_game_config.fee,
            fee_reallocation: old_game_config.fee_reallocation,
            fee_address: old_game_config.fee_address,
            game_denom: old_game_config.game_denom,
            game_cw721_addrs: old_game_config.game_cw721_addrs,
            game_duration: old_game_config.game_duration,
            game_duration_epoch: msg.game_duration_epoch, // this is the only one from migrateMsg
            game_extend: old_game_config.game_extend,
            game_end_threshold: old_game_config.game_end_threshold,
            min_pot_initial_allocation: old_game_config.min_pot_initial_allocation,
            decay_factor: old_game_config.decay_factor,
            reallocations_limit: old_game_config.reallocations_limit,
        },
    )?;

    // remove the old game config
    OLD_GAME_CONFIG.remove(deps.storage);

    Ok(Response::new().add_attribute("migrate", "successful"))
}
