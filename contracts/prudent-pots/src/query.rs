use cosmwasm_std::{Addr, Deps, StdResult};

use crate::{
    helpers::{calculate_max_bid, calculate_min_bid, is_winning_pot},
    msg::{
        QueryBidRangeResponse, QueryGameConfigResponse, QueryGameStateResponse,
        QueryPlayerAllocationsResponse, QueryPotStateResponse, QueryPotsStateResponse,
        QueryReallocationFeePoolResponse, QueryWinningPotsReponse,
    },
    state::{
        PlayerAllocations, GAME_CONFIG, GAME_STATE, PLAYER_ALLOCATIONS, POT_STATES,
        REALLOCATION_FEE_POOL,
    },
};

pub fn query_game_config(deps: Deps) -> StdResult<QueryGameConfigResponse> {
    let config = GAME_CONFIG.load(deps.storage)?;
    Ok(QueryGameConfigResponse { config })
}

pub fn query_game_state(deps: Deps) -> StdResult<QueryGameStateResponse> {
    let state = GAME_STATE.load(deps.storage)?;
    Ok(QueryGameStateResponse { state })
}

pub fn query_bid_range(deps: Deps) -> StdResult<QueryBidRangeResponse> {
    let min_bid = calculate_min_bid(&deps)?;
    let max_bid = calculate_max_bid(&deps)?;
    Ok(QueryBidRangeResponse { min_bid, max_bid })
}

pub fn query_pot_state(deps: Deps, pot_id: u8) -> StdResult<QueryPotStateResponse> {
    let pot = POT_STATES.load(deps.storage, pot_id)?;
    Ok(QueryPotStateResponse { pot })
}

pub fn query_pots_state(deps: Deps) -> StdResult<QueryPotsStateResponse> {
    let mut pots = Vec::new();

    for pot_id in 0..5 {
        if let Ok(pot_state) = POT_STATES.load(deps.storage, pot_id) {
            pots.push(pot_state);
        }
    }

    Ok(QueryPotsStateResponse { pots })
}

pub fn query_winning_pots(deps: Deps) -> StdResult<QueryWinningPotsReponse> {
    let mut pots = Vec::new();

    for pot_id in 0..5 {
        if is_winning_pot(&deps, pot_id)? {
            pots.push(pot_id);
        }
    }

    Ok(QueryWinningPotsReponse { pots })
}

pub fn query_player_allocations(
    deps: Deps,
    address: Addr,
) -> StdResult<QueryPlayerAllocationsResponse> {
    // Attempt to load player allocations. If not found, return an empty PlayerAllocations struct.
    let allocations = PLAYER_ALLOCATIONS
        .may_load(deps.storage, address)?
        .unwrap_or_else(|| PlayerAllocations {
            allocations: Vec::new(),
        });

    Ok(QueryPlayerAllocationsResponse { allocations })
}

pub fn query_reallocation_fee_pool(deps: Deps) -> StdResult<QueryReallocationFeePoolResponse> {
    let reallocation_fee_pool = REALLOCATION_FEE_POOL.load(deps.storage)?;
    Ok(QueryReallocationFeePoolResponse {
        reallocation_fee_pool,
    })
}
