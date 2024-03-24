use cosmwasm_std::{Deps, DepsMut, StdResult};

use crate::{
    helpers::{calculate_max_bid, calculate_min_bid},
    msg::{QueryBidBoundsResponse, QueryGameConfigResponse, QueryGameStateResponse},
    state::{GAME_CONFIG, GAME_STATE},
};

pub fn query_game_config(deps: Deps) -> StdResult<QueryGameConfigResponse> {
    let config = GAME_CONFIG.load(deps.storage)?;
    Ok(QueryGameConfigResponse { config })
}

pub fn query_game_state(deps: Deps) -> StdResult<QueryGameStateResponse> {
    let state = GAME_STATE.load(deps.storage)?;
    Ok(QueryGameStateResponse { state })
}

pub fn query_bid_bounds(deps: DepsMut) -> StdResult<QueryBidBoundsResponse> {
    let min_bid = calculate_min_bid(&deps)?;
    let max_bid = calculate_max_bid(&deps)?;
    Ok(QueryBidBoundsResponse { min_bid, max_bid })
}
