use cosmwasm_std::{Deps, StdResult};

use crate::{
    helpers::{
        game_end::{get_raffle_denom_prize_amounts, get_raffle_winner},
        pot::{calculate_max_bid, calculate_min_bid, get_winning_pots},
    },
    msg::{
        AllPlayersAllocationsResponse, BidRangeResponse, GameConfigResponse, GameStateResponse,
        PlayerAllocationsResponse, PlayerReallocationsResponse, PotStateResponse,
        PotsStateResponse, RaffleDenomSplitResponse, RaffleResponse, RaffleWinnerResponse,
        ReallocationFeePoolResponse, WinningPotsResponse,
    },
    state::{
        GAME_CONFIG, GAME_STATE, PLAYER_ALLOCATIONS, PLAYER_REALLOCATIONS, POT_STATES, RAFFLE,
        REALLOCATION_FEE_POOL,
    },
};

pub fn query_game_config(deps: Deps) -> StdResult<GameConfigResponse> {
    let config = GAME_CONFIG.load(deps.storage)?;
    Ok(GameConfigResponse { config })
}

pub fn query_game_state(deps: Deps) -> StdResult<GameStateResponse> {
    let state = GAME_STATE.load(deps.storage)?;
    Ok(GameStateResponse { state })
}

pub fn query_bid_range(deps: Deps, address: Option<String>) -> StdResult<BidRangeResponse> {
    let min_bid = calculate_min_bid(&deps, address).unwrap();
    let max_bid = calculate_max_bid(&deps).unwrap();
    Ok(BidRangeResponse { min_bid, max_bid })
}

pub fn query_pot_state(deps: Deps, pot_id: u8) -> StdResult<PotStateResponse> {
    let pot = POT_STATES.load(deps.storage, pot_id)?;
    Ok(PotStateResponse { pot })
}

pub fn query_pots_state(deps: Deps) -> StdResult<PotsStateResponse> {
    let mut pots = Vec::new();

    for pot_id in 1..=5 {
        if let Ok(pot_state) = POT_STATES.load(deps.storage, pot_id) {
            pots.push(pot_state);
        }
    }

    Ok(PotsStateResponse { pots })
}

pub fn query_winning_pots(deps: Deps) -> StdResult<WinningPotsResponse> {
    let pots = get_winning_pots(deps.storage).unwrap_or_default();

    Ok(WinningPotsResponse { pots })
}

pub fn query_player_allocations(
    deps: Deps,
    address: String,
) -> StdResult<PlayerAllocationsResponse> {
    // Attempt to load player allocations. If not found, return an empty PlayerAllocations struct.
    let allocations = PLAYER_ALLOCATIONS
        .may_load(deps.storage, address)?
        .unwrap_or_default();

    Ok(PlayerAllocationsResponse { allocations })
}

pub fn query_player_reallocations(
    deps: Deps,
    address: String,
) -> StdResult<PlayerReallocationsResponse> {
    // Attempt to load player reallocations count. If not found, return an deafult PlayerReallocations count 0.
    let reallocations = PLAYER_REALLOCATIONS
        .may_load(deps.storage, address)?
        .unwrap_or_default();

    Ok(PlayerReallocationsResponse { reallocations })
}

pub fn query_all_players_allocations(deps: Deps) -> StdResult<AllPlayersAllocationsResponse> {
    let mut all_allocations = Vec::new();

    // Assuming we have a method to iterate over all items in PLAYER_ALLOCATIONS
    let allocations_iter =
        PLAYER_ALLOCATIONS.range(deps.storage, None, None, cosmwasm_std::Order::Ascending);
    for item in allocations_iter {
        let (address, player_allocations) = item?;
        all_allocations.push((address, player_allocations));
    }

    Ok(AllPlayersAllocationsResponse {
        allocations: all_allocations,
    })
}

pub fn query_reallocation_fee_pool(deps: Deps) -> StdResult<ReallocationFeePoolResponse> {
    let reallocation_fee_pool = REALLOCATION_FEE_POOL.load(deps.storage)?;
    Ok(ReallocationFeePoolResponse {
        reallocation_fee_pool,
    })
}

pub fn query_raffle(deps: Deps) -> StdResult<RaffleResponse> {
    let raffle = RAFFLE.load(deps.storage)?;

    Ok(RaffleResponse { raffle })
}

pub fn query_raffle_winner(deps: Deps) -> StdResult<RaffleWinnerResponse> {
    let winning_pots = query_winning_pots(deps)?;
    let raffle_winner = get_raffle_winner(deps.storage, &winning_pots.pots).unwrap();

    Ok(RaffleWinnerResponse { raffle_winner })
}

pub fn query_raffle_denom_split(deps: Deps) -> StdResult<RaffleDenomSplitResponse> {
    let (prize_to_distribute, prize_to_treasury) = get_raffle_denom_prize_amounts(&deps).unwrap();

    Ok(RaffleDenomSplitResponse {
        prize_to_distribute,
        prize_to_treasury,
    })
}
