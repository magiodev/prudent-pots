use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GameConfig {
    pub fee_allocation: u64,
    pub fee_reallocation: u64,
    pub fee_allocation_address: Addr,
    pub game_duration: u64,
    pub game_denom: String,
    pub min_bid: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TokenAllocation {
    pub pot_id: u8,
    pub amount: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PlayerAllocations {
    pub allocations: Vec<TokenAllocation>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GameState {
    pub start_time: u64,
    pub end_time: u64,
}

pub const GAME_CONFIG: Item<GameConfig> = Item::new("game_config");
pub const PLAYER_ALLOCATIONS: Map<Addr, PlayerAllocations> = Map::new("player_allocations");
pub const POT_STATES: Map<u8, Uint128> = Map::new("pot_states");
pub const GAME_STATE: Item<GameState> = Item::new("game_state");
pub const REALLOCATION_FEE_POOL: Item<Uint128> = Item::new("reallocation_fee_pool");
