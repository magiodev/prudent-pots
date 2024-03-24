use cosmwasm_schema::{cw_serde, QueryResponses};

use crate::state::{GameConfig, GameState};

#[cw_serde]
pub struct InstantiateMsg {
    pub config: GameConfig,
}

#[cw_serde]
pub enum ExecuteMsg {
    UpdateConfig {
        config: GameConfig,
    },
    AllocateTokens {
        pot_id: u8,
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
