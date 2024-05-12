use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint128};
use num_enum::{FromPrimitive, IntoPrimitive};

use crate::state::{GameConfig, GameState, Raffle, TokenAllocation};

#[cw_serde]
pub struct InstantiateMsg {
    pub config: GameConfig,
    // pub raffle_cw721_token_ids: Vec<String>,
    // pub raffle_denom_amount: Uint128,
}

#[cw_serde]
pub enum ExecuteMsg {
    UpdateConfig {
        fee: Option<u64>,
        fee_reallocation: Option<u64>,
        fee_address: Option<Addr>,
        game_denom: Option<String>,
        game_cw721_addrs: Vec<Addr>, // this is the cw721 collection addy we use as optional raffle prize
        game_duration: Option<u64>,
        game_extend: Option<u64>,
        game_end_threshold: Option<u64>,
        min_pot_initial_allocation: Option<Uint128>,
        decay_factor: Option<Uint128>,
        reallocations_limit: Option<u64>,
    },
    AllocateTokens {
        pot_id: u8,
    },
    ReallocateTokens {
        from_pot_id: u8,
        to_pot_id: u8,
    },
    GameEnd {
        raffle_cw721_token_id: Option<String>,
        raffle_cw721_token_addr: Option<String>,
    },
}

/// Reply

#[derive(FromPrimitive, IntoPrimitive)]
#[repr(u64)]
pub enum ReplyMsg {
    GameEnd = 1,
    #[default]
    Unknown,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GameConfigResponse)]
    GameConfig {},
    #[returns(GameStateResponse)]
    GameState {},
    #[returns(BidRangeResponse)]
    BidRange { address: Option<String> },
    #[returns(PotStateResponse)]
    PotState { pot_id: u8 },
    #[returns(PotsStateResponse)]
    PotsState {},
    #[returns(WinningPotsResponse)]
    WinningPots {},
    #[returns(PlayerAllocationsResponse)]
    PlayerAllocations { address: String },
    #[returns(AllPlayersAllocationsResponse)]
    AllPlayersAllocations {},
    #[returns(ReallocationFeePoolResponse)]
    ReallocationFeePool {},
    #[returns(RaffleResponse)]
    Raffle {},
    #[returns(RaffleWinnerResponse)]
    RaffleWinner {},
    #[returns(RaffleDenomSplitResponse)]
    RaffleDenomSplit {},
}

#[cw_serde]
pub struct GameConfigResponse {
    pub config: GameConfig,
}

#[cw_serde]
pub struct GameStateResponse {
    pub state: GameState,
}

#[cw_serde]
pub struct BidRangeResponse {
    pub min_bid: Uint128,
    pub max_bid: Uint128,
}

#[cw_serde]
pub struct PotStateResponse {
    pub pot: TokenAllocation,
}

#[cw_serde]
pub struct PotsStateResponse {
    pub pots: Vec<TokenAllocation>,
}

#[cw_serde]
pub struct WinningPotsResponse {
    pub pots: Vec<u8>,
}

#[cw_serde]
pub struct PlayerAllocationsResponse {
    pub allocations: Vec<TokenAllocation>,
}

#[cw_serde]
pub struct AllPlayersAllocationsResponse {
    pub allocations: Vec<(String, Vec<TokenAllocation>)>,
}

#[cw_serde]
pub struct ReallocationFeePoolResponse {
    pub reallocation_fee_pool: Uint128,
}

#[cw_serde]
pub struct RaffleResponse {
    pub raffle: Raffle,
}

#[cw_serde]
pub struct RaffleWinnerResponse {
    pub raffle_winner: Option<String>,
}

#[cw_serde]
pub struct RaffleDenomSplitResponse {
    pub prize_to_distribute: Uint128,
    pub prize_to_treasury: Uint128,
}

#[cw_serde]
pub struct MigrateMsg {
    pub new_game_config: GameConfig,
}
