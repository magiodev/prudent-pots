use cosmwasm_std::{assert_approx_eq, coin, coins, testing::mock_info, Uint128};
use cw721::{Cw721QueryMsg, TokensResponse};

use crate::{
    msg::{
        GameStateResponse, PlayerAllocationsResponse, PotsStateResponse, QueryMsg,
        RaffleDenomSplitResponse, RaffleResponse, RaffleWinnerResponse,
        ReallocationFeePoolResponse, WinningPotsResponse,
    },
    state::{Raffle, TokenAllocation},
    tests::integration::{
        fixtures::{default_with_balances, ADMIN_ADDRESS, DENOM_GAME},
        helpers::{game_end, reallocate_tokens},
    },
};

use super::{
    fixtures::{increase_app_time, GAME_DURATION},
    helpers::allocate_tokens,
};

#[test]
fn test_game_end_one_winner_simple_works() {
    let (mut app, pp_addr, _cw721_addr) =
        default_with_balances(5, vec![coin(100_000_000u128, DENOM_GAME.to_string())], None);

    // Game state extend_count after
    let game_state: GameStateResponse = app
        .wrap()
        .query_wasm_smart(&pp_addr, &QueryMsg::GameState {})
        .unwrap();
    assert_eq!(game_state.state.extend_count, 0);
    assert_eq!(game_state.state.round_count, 1); // this is a side effect of raffle which can be set only on 2nd and next rounds

    // Allocate tokens with 5 users, starting from a minBid of 1 $DENOM (so also 1 $DENOM per pot)
    let info_1 = mock_info("user1", &coins(2_000_000, DENOM_GAME));
    allocate_tokens(&mut app, &pp_addr, &info_1, 1).unwrap(); // tot. 7.0, minBid is now 1.4 - Pot 1 has 3.0 tokens (L)
    let info_2 = mock_info("user2", &coins(1_400_001, DENOM_GAME));
    allocate_tokens(&mut app, &pp_addr, &info_2, 2).unwrap(); // tot. 8,400001, minBid is now 1_680_000(2) - Pot 2 has 2.400001 (L)
    let info_3 = mock_info("user3", &coins(1_680_000, DENOM_GAME));
    allocate_tokens(&mut app, &pp_addr, &info_3, 3).unwrap(); // tot. 10,080001, minBid is now 2,016000(2) - Pot 3 has 2,68 (L)
    let info_4 = mock_info("user4", &coins(2_016_000, DENOM_GAME));
    allocate_tokens(&mut app, &pp_addr, &info_4, 4).unwrap(); // tot. 12,096001, minBid is now 2,419200(24) - Pot 4 has 3,016 (L)
    let info_5 = mock_info("user5", &coins(2_419_200, DENOM_GAME));
    allocate_tokens(&mut app, &pp_addr, &info_5, 5).unwrap(); // tot. 14,515201, minBid is now 2,903040(2) - Pot 5 has 3.4192 (W)

    // Total tokens: 14,515201 / Winning 3.4192 / Losing 11,096001.div(2) === 5,548000(5)
    let total_tokens = Uint128::new(5_000_000) // raffle is adding 5_000_000 to the contract balance
        + info_1.funds[0].amount
        + info_2.funds[0].amount
        + info_3.funds[0].amount
        + info_4.funds[0].amount
        + info_5.funds[0].amount;
    assert_eq!(total_tokens.u128(), 14_515_201u128);

    // Winning pot 5 has initial balance (1) + user 5 allocation
    let winning_tokens = 1_000_000 + info_5.funds[0].amount.u128();
    assert_eq!(winning_tokens, 3_419_200u128);

    // Losing pots 1, 2, 3, 4 have initial balance (4) + losers allocation
    let losing_tokens = total_tokens.u128() - winning_tokens;
    assert_eq!(losing_tokens, 11_096_001u128);

    // we only pay 50% of the losing tokens, the other 50% goes to next round
    let winning_pots_get = losing_tokens / 2;
    assert_eq!(winning_pots_get, 5_548_000u128);

    // user 5 should receive 95% of the winning tokens + (losing tokens / 2)
    let winner_gets = (winning_tokens + winning_pots_get) * 95 / 100;
    assert_eq!(winner_gets, 8_518_840u128);

    // Get user balance after allocating funds, before game_end
    let user5_balance_before = app.wrap().query_balance("user5", DENOM_GAME).unwrap();

    // Assert get_winning_pots only returns pot 5
    let winning_pots: WinningPotsResponse = app
        .wrap()
        .query_wasm_smart(&pp_addr, &QueryMsg::WinningPots {})
        .unwrap();
    assert_eq!(winning_pots.pots.len(), 1); // only one winner
    assert_eq!(winning_pots.pots[0], 5); // pot id 5

    // Increase time by GAME_DURATION + 1 second to make the game expire
    increase_app_time(&mut app, GAME_DURATION + 1);

    // Game end and new raffles
    let info = mock_info(ADMIN_ADDRESS, &vec![]);
    game_end(&mut app, &pp_addr, &info, None, None).unwrap();

    // Get user balance after game_end
    let user5_balance_after = app.wrap().query_balance("user5", DENOM_GAME).unwrap();
    assert_eq!(
        user5_balance_after.amount.u128(),
        user5_balance_before.amount.u128() + 8_518_840u128
    );

    // Query pots_state and sum all of them
    let pots_state: PotsStateResponse = app
        .wrap()
        .query_wasm_smart(&pp_addr, &QueryMsg::PotsState {})
        .unwrap();
    let pots_state_total_amount: Uint128 = pots_state.pots.iter().map(|pot| pot.amount).sum();
    // Query contract balance and compare with summed pots_state after new round is started
    let contract_balance = app.wrap().query_balance(&pp_addr, DENOM_GAME).unwrap();
    // Assert contract balance is consistent with new pot_state
    assert_approx_eq!(contract_balance.amount, pots_state_total_amount, "0.000001");
}

// TODO: fn test_game_end_multiple_winners_simple_works

// We instantiate the contract with 5_000_000 "udenom"
// We do not play the first round.
// We game_end passing cw_721_token_ids [1] and 100_000_000 "udenom" as raffle prize.
// Now the contract should have:
// > 1x NFT (id 1)
// > 100 + 5 $DENOM (105_000_000 udenom)
// Winning pot initially is pot 2 as Even with 1.0. Minimum bid also is 1.0.
// Player1 bets on pot_5 amount 1_000_000 (raises minBid to 1.20)
// Player2 bets on pot_4 amount 1_500_000 (raises minBid to 1.50)
// Player3 bets on pot_3 amount 2_500_000 (raises minBid to 2.00)
// Player4 bets on pot_1 amount 2_500_000 (raises minBid to 2.50)
// Player5 bets on pot_1 amount 3_000_000 (raises minBid to 3.10)
// > We increase time here first to make reallocate a late-game action
// Player5 reallocates fro pot_1 to pot_2 to make it winner with 3_000_000 * 0.95 = 2_850_000 ( 150_000 fee)
// Final pot allocations should be like:
// 	 Initial:    Bet:        Total:
// > 1: 1_000_000 + 2_500_000 = 3_500_000 (loser because its not Lower)
// > 2: 1_000_000 + 2_850_000 = 3_850_000 (winner because its Even)
// > 3: 1_000_000 + 2_500_000 = 3_500_000 (loser because its not Median)
// > 4: 1_000_000 + 1_500_000 = 2_500_000 (loser because its not Odd)
// > 5: 1_000_000 + 1_000_000 = 2_000_000 (loser because its not Highest)
// Total losing: 3_500_000 + 3_500_000 + 2_500_000 + 2_000_000 = 11_500_000
// Should be split between next_game and distribution amount:
// > Next game funds: 5_750_000
// > Distribution to winners: 5_750_000
// Player5 should receive
// > (Initial pot amount + Bet) * 0.95 = (1_000_000 + 2_850_000) * 0.95 = 3_657_500 (192_500 fee)
// > (Total losing / 2) = (11_500_000 / 2) * 0.95 = 5_462_500 (287_500 fee).
// Player5 is also the raffle winner, so he should receive an additional 100_000_000 udenom but as the extend_count is 1, this will be split * 0.95 with the treasury.
// - 95_000_000 to Player5
// - 5_000_000 to Treasury
// Final balances should be:
// > Player5:
// 	$DENOM: Any amount from before + 3_657_500 + 5_462_500 + 95_000_000 = 104_120_000
// 	NFTS: token_id_1
// > Treasury:
// 	$DENOM: 150_000 + 192_500 + 287_500 + 5_000_000 = 5_630_000
// > Contract:
// 	$DENOM: 5_750_000 / 5 = 1_150_000 per pot. Plus the new raffle amount of 200 $DENOM.
// 		Total here should be 205_750_000 where 5_750_000 are the sum of the initial pot amount, and 200M matches the Raffle.denom_amount value.
// 	NFTS: new token_ids 2, 3
// Final useful asserts:
// > Contract balance at start of a new game should always matches the sum of all POT_STATES[*].amount
#[test]
fn test_game_end_one_winner_raffle_both_works() {
    let (mut app, pp_addr, cw721_addr) = default_with_balances(
        5,
        vec![coin(100_000_000u128, DENOM_GAME.to_string())],
        Some(Raffle {
            cw721_token_id: Some("1".to_string()),
            cw721_addr: None, // this will be overridden by the fixture after cw721 contract instantiation
            denom_amount: Uint128::new(100_000_000u128),
        }),
    );

    // Game state extend_count after
    let game_state: GameStateResponse = app
        .wrap()
        .query_wasm_smart(&pp_addr, &QueryMsg::GameState {})
        .unwrap();
    assert_eq!(game_state.state.extend_count, 0);
    assert_eq!(game_state.state.round_count, 2); // this is a side effect of raffle which can be set only on 2nd and next rounds

    // Query raffle
    let raffle: RaffleResponse = app
        .wrap()
        .query_wasm_smart(&pp_addr, &QueryMsg::Raffle {})
        .unwrap();
    assert_eq!(
        raffle.raffle,
        Raffle {
            cw721_token_id: Some("1".to_string()),
            cw721_addr: Some(cw721_addr.to_string()),
            denom_amount: Uint128::new(100_000_000u128)
        }
    );

    // Allocate tokens with 5 users, starting from a minBid of 1 $DENOM (so also 1 $DENOM per pot)
    let info_1 = mock_info("user1", &coins(1_000_000, DENOM_GAME));
    allocate_tokens(&mut app, &pp_addr, &info_1, 5).unwrap(); // tot. 7.0, minBid is now 1.4 - Pot 1 has 3.0 tokens (L)
    let info_2 = mock_info("user2", &coins(1_500_000, DENOM_GAME));
    allocate_tokens(&mut app, &pp_addr, &info_2, 4).unwrap(); // tot. 8,400001, minBid is now 1_680_000(2) - Pot 2 has 2.400001 (L)
    let info_3 = mock_info("user3", &coins(2_500_000, DENOM_GAME));
    allocate_tokens(&mut app, &pp_addr, &info_3, 3).unwrap(); // tot. 10,080001, minBid is now 2,016000(2) - Pot 3 has 2,68 (L)
    let info_4 = mock_info("user4", &coins(2_500_000, DENOM_GAME));
    allocate_tokens(&mut app, &pp_addr, &info_4, 1).unwrap(); // tot. 12,096001, minBid is now 2,419200(24) - Pot 4 has 3,016 (L)
                                                              // we allocate twice to pot_id 4, so we will reallocate as user5 to pot_id 5 during late-game time window
    let info_5 = mock_info("user5", &coins(3_000_000, DENOM_GAME));
    allocate_tokens(&mut app, &pp_addr, &info_5, 1).unwrap(); // tot. 14,515201, minBid is now 2,903040(2) - Pot 4 has 6,4352 (W)

    // pot id 2 remains at 1.0 initial allocated tokens

    // get initial balance for user 5 post allocation of funds, before game_end and reallocation
    let user5_balance_before = app.wrap().query_balance("user5", DENOM_GAME).unwrap();

    // Extend the game once, this should cause the contract to reduce the raffle winner prize, and start splitting it with treasury
    increase_app_time(&mut app, GAME_DURATION);

    // Reallocate to make pot 5 winner and extend once the game time due to late-game action
    let info = mock_info("user5", &vec![]);
    reallocate_tokens(&mut app, &pp_addr, &info, 1, 2).unwrap();
    let user5_allocations: PlayerAllocationsResponse = app
        .wrap()
        .query_wasm_smart(
            &pp_addr,
            &QueryMsg::PlayerAllocations {
                address: "user5".to_string(),
            },
        )
        .unwrap();
    assert_eq!(
        user5_allocations.allocations,
        vec![
            TokenAllocation {
                pot_id: 1,
                amount: Uint128::zero()
            },
            TokenAllocation {
                pot_id: 2,
                amount: Uint128::new(3_000_000).multiply_ratio(95u128, 100u128) // 150_000 fee collected here
            }
        ]
    );

    // Assert reallocation fee pool is 5% of previous move
    let reallocation_fee_pool: ReallocationFeePoolResponse = app
        .wrap()
        .query_wasm_smart(&pp_addr, &QueryMsg::ReallocationFeePool {})
        .unwrap();
    assert_eq!(
        reallocation_fee_pool.reallocation_fee_pool,
        Uint128::new(3_000_000).multiply_ratio(5u128, 100u128) // 150_000
    );

    // Game state extend_count after
    let game_state: GameStateResponse = app
        .wrap()
        .query_wasm_smart(&pp_addr, &QueryMsg::GameState {})
        .unwrap();
    assert_eq!(game_state.state.extend_count, 1);

    // Check split has been effective with a late-game action as reallocate when 1 second is left
    let raffle_denom_split: RaffleDenomSplitResponse = app
        .wrap()
        .query_wasm_smart(&pp_addr, &QueryMsg::RaffleDenomSplit {})
        .unwrap();
    assert_eq!(
        raffle_denom_split.prize_to_distribute,
        Uint128::new(95_000_000u128)
    );
    assert_eq!(
        raffle_denom_split.prize_to_treasury,
        Uint128::new(5_000_000u128)
    );

    // Increase time by GAME_DURATION + 1 second to make the game expire
    increase_app_time(&mut app, 601);

    let expected_sum = (Uint128::new(5_000_000) // raffle is adding 5_000_000 to the contract balance
        + info_1.funds[0].amount
        + info_2.funds[0].amount
        + info_3.funds[0].amount
        + info_4.funds[0].amount
        + info_5.funds[0].amount.multiply_ratio(95u128, 100u128))
    .u128();

    let pots_state: PotsStateResponse = app
        .wrap()
        .query_wasm_smart(&pp_addr, &QueryMsg::PotsState {})
        .unwrap();
    let total_sum: u128 = pots_state
        .pots
        .iter()
        .fold(0, |acc, pot| acc + pot.amount.u128());
    assert_eq!(expected_sum, total_sum);

    // Assert get_winning_pots only returns pot 2
    let winning_pots: WinningPotsResponse = app
        .wrap()
        .query_wasm_smart(&pp_addr, &QueryMsg::WinningPots {})
        .unwrap();
    assert_eq!(winning_pots.pots.len(), 1); // only one winner
    assert_eq!(winning_pots.pots[0], 2);

    let winning = pots_state
        .pots
        .iter()
        .find(|pot| pot.pot_id == winning_pots.pots[0])
        .unwrap()
        .amount
        .u128();
    assert_eq!(
        winning,
        1_000_000 + info_5.funds[0].amount.u128() * 95u128 / 100u128
    );

    // we only pay 50% of the losing tokens, the other 50% goes to next round
    let losing = total_sum - winning;
    assert_eq!(losing, 11_500_000);
    let winning_pot_gets = winning + losing / 2;
    assert_eq!(winning_pot_gets, 3_850_000 + 5_750_000);

    // Assert winner user of raffle
    let raffle_winner: RaffleWinnerResponse = app
        .wrap()
        .query_wasm_smart(&pp_addr, &QueryMsg::RaffleWinner {})
        .unwrap();
    assert_eq!(raffle_winner.raffle_winner, Some("user5".to_string()));

    // Game end and new raffles
    let info = mock_info(ADMIN_ADDRESS, &coins(200_000_000u128, DENOM_GAME));
    let _res = game_end(
        &mut app,
        &pp_addr,
        &info,
        Some("2".to_string()),
        Some(cw721_addr.to_string()),
    )
    .unwrap();

    // Game state extend_count after
    let game_state: GameStateResponse = app
        .wrap()
        .query_wasm_smart(&pp_addr, &QueryMsg::GameState {})
        .unwrap();
    assert_eq!(game_state.state.extend_count, 0);
    assert_eq!(game_state.state.round_count, 3);

    // Get user balance after game_end
    let user5_balance_after = app.wrap().query_balance("user5", DENOM_GAME).unwrap();
    assert_eq!(
        (user5_balance_after.amount - user5_balance_before.amount).u128(),
        (1_000_000+info_5.funds[0].amount.u128())
            - 192_500u128 // fee paid over winning amount to treasury
            - reallocation_fee_pool.reallocation_fee_pool.u128() // fee paid on reallocating to pot 2 over player's funds
            + 5_750_000u128 // losing amount / 2
            - 287_500u128 // fee on losing amount / 2
            + raffle_denom_split.prize_to_distribute.u128()
    );

    // Assert NFT balance for user5, it should have the NFT #1
    let raffle_winner_nft_balance: TokensResponse = app
        .wrap()
        .query_wasm_smart(
            &cw721_addr,
            &Cw721QueryMsg::Tokens {
                owner: "user5".to_string(),
                start_after: None,
                limit: None,
            },
        )
        .unwrap();
    assert_eq!(raffle_winner_nft_balance.tokens.len(), 1); // check length 1
    assert_eq!(raffle_winner_nft_balance.tokens[0], "1");

    // Assert NFT balance for contract pp_addr, it should have the NFT #2
    let contract_new_nft_balance: TokensResponse = app
        .wrap()
        .query_wasm_smart(
            &cw721_addr,
            &Cw721QueryMsg::Tokens {
                owner: pp_addr.to_string(),
                start_after: None,
                limit: None,
            },
        )
        .unwrap();
    assert_eq!(contract_new_nft_balance.tokens.len(), 1);
    assert_eq!(contract_new_nft_balance.tokens[0], "2");

    // Assert raffle state
    let raffle: RaffleResponse = app
        .wrap()
        .query_wasm_smart(&pp_addr, &QueryMsg::Raffle {})
        .unwrap();
    assert_eq!(
        raffle.raffle,
        Raffle {
            cw721_token_id: Some("2".to_string()),
            cw721_addr: Some(cw721_addr.to_string()),
            denom_amount: Uint128::new(200_000_000u128)
        }
    );

    // Query pots_state and sum all of them
    let pots_state: PotsStateResponse = app
        .wrap()
        .query_wasm_smart(&pp_addr, &QueryMsg::PotsState {})
        .unwrap();
    let pots_state_total_amount: Uint128 = pots_state.pots.iter().map(|pot| pot.amount).sum();

    // Query contract balance and compare with summed pots_state after new round is started
    let contract_balance = app.wrap().query_balance(&pp_addr, DENOM_GAME).unwrap();
    // Assert contract balance is consistent with new pot_state
    assert_approx_eq!(
        contract_balance
            .amount
            .checked_sub(Uint128::new(200_000_000u128))
            .unwrap(),
        pots_state_total_amount,
        "0.000001"
    );

    // Assert treasury balance: sources are (fee, realloc_fee, raffle_denom_prize_split)
    let treasury_balance = app
        .wrap()
        .query_balance("treasury_addr", DENOM_GAME)
        .unwrap();

    // TOOD: Fix this assert
    assert_eq!(
        treasury_balance.amount.u128(),
        150_000u128 // reallocation fee from player5 realloc from pot 1 to 2
        + 192_500u128 // fee on winning pot (init alloc + player 5 bet)
        + 287_500u128 // fee on total losing / 2
        + 5_000_000u128 // from raffle split denom amount
    );
}

// TODO: fn test_game_end_multiple_winners_raffle_both_works

// TODO: fn test_game_end_multiple_winners_raffle_both_tie_works
