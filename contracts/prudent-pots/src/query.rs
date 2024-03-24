use cosmwasm_std::{Deps, StdResult};

use crate::{
    msg::{QueryGameConfigResponse, QueryGameStateResponse},
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
