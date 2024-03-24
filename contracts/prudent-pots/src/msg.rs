use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;

use crate::state::{GameConfig, GameState};

#[cw_serde]
pub struct InstantiateMsg {
    pub fee_allocation: u64,
    pub fee_reallocation: u64,
    pub team_fee_address: Addr,
    pub game_duration: u64,
    pub initial_pot_tokens: u64,
}

#[cw_serde]
pub enum ExecuteMsg {
    UpdateConfig {
        config: GameConfig,
    },
    AllocateTokens {
        pot_id: u8,
        amount: u64,
    },
    ReallocateTokens {
        from_pot_id: u8,
        to_pot_id: u8,
        amount: u64,
    },
    GameEnd {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(QueryGameConfigResponse)]
    QueryGameConfig {},
    #[returns(QueryGameStateResponse)]
    QueryGameState {},
}

#[cw_serde]
pub struct QueryGameConfigResponse {
    pub config: GameConfig,
}

#[cw_serde]
pub struct QueryGameStateResponse {
    pub state: GameState,
}
