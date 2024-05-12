use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct GameConfig {
    pub fee: u64,
    pub fee_reallocation: u64,
    pub fee_address: Addr,
    pub game_denom: String,
    pub game_cw721_addrs: Vec<Addr>, // these are the cw721 addys that grant minBid discount eligibility
    pub game_duration: u64,
    pub game_extend: u64,
    pub game_end_threshold: u64,
    pub min_pot_initial_allocation: Uint128, // i.e. 1000000 for 1 $OSMO
    pub decay_factor: Uint128,               // i.e. 95 as 95%
    pub reallocations_limit: u64,
}

#[cw_serde]
pub struct Raffle {
    pub cw721_token_id: Option<String>, // the tokenId of the raffle nft to be won
    pub cw721_addr: Option<String>,     // one of the whitelisted addys
    pub denom_amount: Uint128, // this is limited to the same game_config.game_denom for now
}

#[cw_serde]
#[derive(Default)]
pub struct GameState {
    pub round_count: u64,
    pub extend_count: u32,
    pub start_time: u64,
    pub end_time: u64,
}

#[cw_serde]
pub struct TokenAllocation {
    pub pot_id: u8,
    pub amount: Uint128,
}

#[cw_serde]
pub struct PlayerAllocations {
    pub allocations: Vec<TokenAllocation>,
}

#[cw_serde]
pub struct FirstBidder {
    pub bidder: String,
    pub time: u64,
}

pub const GAME_CONFIG: Item<GameConfig> = Item::new("game_config");
pub const GAME_STATE: Item<GameState> = Item::new("game_state");
pub const POT_STATES: Map<u8, TokenAllocation> = Map::new("pot_states");
pub const PLAYER_ALLOCATIONS: Map<String, Vec<TokenAllocation>> = Map::new("player_allocations");
pub const PLAYER_REALLOCATIONS: Map<String, u64> = Map::new("player_reallocations");
pub const REALLOCATION_FEE_POOL: Item<Uint128> = Item::new("reallocation_fee_pool");
pub const RAFFLE: Item<Raffle> = Item::new("raffle");
pub const FIRST_BIDDER: Map<u8, FirstBidder> = Map::new("first_bidder");
