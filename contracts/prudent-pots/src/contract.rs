#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::execute::{
    allocate_tokens, game_end, reallocate_tokens, update_config, validate_and_sum_funds,
};
use crate::helpers::prepare_next_game;
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::query::{
    query_bid_range, query_game_config, query_game_state, query_player_allocations,
    query_pot_state, query_pots_state, query_reallocation_fee_pool, query_winning_pots,
};
use crate::state::{GAME_CONFIG, REALLOCATION_FEE_POOL};

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
    if msg.config.fee_allocation > 100 || msg.config.fee_reallocation > 100 {
        return Err(ContractError::InvalidInput {});
    }
    if msg.config.game_duration == 0 {
        return Err(ContractError::InvalidInput {});
    }
    if msg.config.min_bid.is_zero() {
        return Err(ContractError::InvalidInput {});
    }

    // Validate and sum initial funds
    validate_and_sum_funds(&info, &msg.config.game_denom)?;

    GAME_CONFIG.save(deps.storage, &msg.config)?;
    REALLOCATION_FEE_POOL.save(deps.storage, &Uint128::zero())?;

    // Initialize game state and pots for the next game
    prepare_next_game(deps, &env)?;

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
        ExecuteMsg::AllocateTokens { pot_id } => allocate_tokens(deps, info, pot_id),
        ExecuteMsg::ReallocateTokens {
            from_pot_id,
            to_pot_id,
            amount,
        } => reallocate_tokens(deps, info, from_pot_id, to_pot_id, amount),
        ExecuteMsg::GameEnd {} => game_end(deps, env),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::QueryGameConfig {} => to_json_binary(&query_game_config(deps)?),
        QueryMsg::QueryGameState {} => to_json_binary(&query_game_state(deps)?),
        QueryMsg::QueryBidRange {} => to_json_binary(&query_bid_range(deps)?),
        QueryMsg::QueryPotState { pot_id } => to_json_binary(&query_pot_state(deps, pot_id)?),
        QueryMsg::QueryPotsState {} => to_json_binary(&query_pots_state(deps)?),
        QueryMsg::QueryWinningPots {} => to_json_binary(&query_winning_pots(deps)?),
        QueryMsg::QueryPlayerAllocations { address } => {
            to_json_binary(&query_player_allocations(deps, address)?)
        }
        QueryMsg::QueryReallocationFeePool {} => {
            to_json_binary(&query_reallocation_fee_pool(deps)?)
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    Ok(Response::new().add_attribute("migrate", "successful"))
}
