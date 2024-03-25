use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint128};

use crate::state::{GameConfig, GameState, PlayerAllocations};

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
        amount: Uint128,
    },
    GameEnd {},
}

#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(QueryGameConfigResponse)]
    QueryGameConfig {},
    #[returns(QueryGameStateResponse)]
    QueryGameState {},
    #[returns(QueryBidRangeResponse)]
    QueryBidRange {},
    #[returns(QueryPotStateResponse)]
    QueryPotState { pot_id: u8 },
    #[returns(QueryPlayerAllocationsResponse)]
    QueryPlayerAllocations { address: Addr },
    #[returns(QueryReallocationFeePoolResponse)]
    QueryReallocationFeePool {},
}

#[cw_serde]
pub struct QueryGameConfigResponse {
    pub config: GameConfig,
}

#[cw_serde]
pub struct QueryGameStateResponse {
    pub state: GameState,
}

#[cw_serde]
pub struct QueryBidRangeResponse {
    pub min_bid: Uint128,
    pub max_bid: Uint128,
}

#[cw_serde]
pub struct QueryPotStateResponse {
    pub pot_id: u8,
    pub pot_state: Uint128,
}

#[cw_serde]
pub struct QueryPlayerAllocationsResponse {
    pub allocations: PlayerAllocations,
}

#[cw_serde]
pub struct QueryReallocationFeePoolResponse {
    pub reallocation_fee_pool: Uint128,
}
