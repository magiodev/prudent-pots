use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GameConfig {
    pub fee_allocation: u64,
    pub fee_reallocation: u64,
    pub team_fee_address: Addr,
    pub game_duration: u64,
    pub initial_pot_tokens: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TokenAllocation {
    pub pot_id: u8,
    pub amount: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PlayerAllocations {
    pub allocations: Vec<TokenAllocation>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PotState {
    pub total_tokens: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GameState {
    pub is_active: bool,
    pub start_time: u64,
    pub end_time: u64,
}

pub const GAME_CONFIG: Item<GameConfig> = Item::new("game_config");
pub const PLAYER_ALLOCATIONS: Map<Addr, PlayerAllocations> = Map::new("player_allocations");
pub const POT_STATES: Map<u8, PotState> = Map::new("pot_states");
pub const GAME_STATE: Item<GameState> = Item::new("game_state");
