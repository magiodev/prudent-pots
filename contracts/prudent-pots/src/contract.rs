#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::execute::{allocate_tokens, game_end, reallocate_tokens, update_config};
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::query::{query_game_config, query_game_state};
use crate::state::{GAME_CONFIG, REALLOCATION_FEE_POOL};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:prudent-pot";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    // TODO: Validate game_config fields

    GAME_CONFIG.save(deps.storage, &msg.config)?;
    // pub const PLAYER_ALLOCATIONS: Map<Addr, PlayerAllocations> = Map::new("player_allocations");
    // pub const POT_STATES: Map<u8, PotState> = Map::new("pot_states");
    // pub const GAME_STATE: Item<GameState> = Item::new("game_state");
    REALLOCATION_FEE_POOL.save(deps.storage, &Uint128::zero())?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("action", "instantiate")
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
        // QueryMsg::QueryBidBounds {} => to_json_binary(&query_bid_bounds(&mut deps)?),
    }
}
