use cosmwasm_std::{coin, coins, testing::mock_info, Uint128};

use crate::msg::{PotStateResponse, QueryMsg};
use crate::state::TokenAllocation;
use crate::tests::integration::fixtures::{
    default_with_balances, increase_app_time, DENOM_GAME, GAME_EXTEND,
};
use crate::tests::integration::helpers::allocate_tokens;
use crate::ContractError;

#[test]
fn test_allocate_tokens_works() {
    let (mut app, pp_addr, _cw721_addr) =
        default_with_balances(1, vec![coin(100_000_000u128, DENOM_GAME.to_string())], None);

    // Sending enough funds
    let info = mock_info("user1", &coins(1_000_000, DENOM_GAME));
    allocate_tokens(&mut app, &pp_addr, &info, 1).unwrap();
    // Query states
    let pots_state: PotStateResponse = app
        .wrap()
        .query_wasm_smart(pp_addr.clone(), &QueryMsg::PotState { pot_id: 1 })
        .unwrap();
    let expected_pots_state = PotStateResponse {
        pot: TokenAllocation {
            pot_id: 1,
            amount: Uint128::new(2_000_000),
        },
    };
    assert_eq!(pots_state, expected_pots_state);
}

#[test]
fn test_allocate_tokens_fails() {
    let (mut app, pp_addr, _cw721_addr) =
        default_with_balances(1, vec![coin(100_000_000u128, DENOM_GAME.to_string())], None);

    // Sending too little funds
    let info = mock_info("user1", &coins(1, DENOM_GAME));
    // Attempt to allocate tokens
    let res = allocate_tokens(&mut app, &pp_addr, &info, 1).unwrap_err();
    // Check if the result is an error and if it matches the expected error
    if let Some(ContractError::BidOutOfRange { min, max }) = res.downcast_ref::<ContractError>() {
        assert_eq!(*min, Uint128::new(1000000u128));
        assert_eq!(*max, Uint128::new(2000000u128));
    } else {
        panic!("Expected ContractError::BidOutOfRange");
    }

    // Query states to ensure no changes in the pot state after the failed allocation
    let pots_state: PotStateResponse = app
        .wrap()
        .query_wasm_smart(pp_addr.clone(), &QueryMsg::PotState { pot_id: 1 })
        .unwrap();
    let expected_pots_state = PotStateResponse {
        pot: TokenAllocation {
            pot_id: 1,
            amount: Uint128::new(1_000_000),
        },
    };
    assert_eq!(pots_state, expected_pots_state);
}

#[test]
fn test_allocate_tokens_min_bet_works() {
    let (mut app, pp_addr, _cw721_addr) =
        default_with_balances(1, vec![coin(100_000_000u128, DENOM_GAME.to_string())], None);

    // Contract started with 5 $DENOM and a min_pot_initial_allocation of 1.0 $DENOM.
    // Subsequently the minimum bet starts by 1.0 $DENOM with a 0.0 multiplier as we are still in the epoch 0.
    // The max bet, which still follows the average based calculation will allow 2.0 $DENOM to be sent to the pot.

    // Game duration is 3600 and game_duration_epoch is 600 as well as the game_extend.
    // This means we have 6 epochs in total and the decay factor is 0.05.

    // THe game_extend_count will bump this 0.05 by 0, 1, 2, 3, and so on... until the game ends,
    // the extend_count is automatically bumped by the allocate / reallocate entrypoint if something happens during the late game.

    // Sending the minimum bet amount to pot 1. We expect to be able send the minimum bet.
    let info = mock_info("user1", &coins(1_000_000, DENOM_GAME));
    allocate_tokens(&mut app, &pp_addr, &info, 1).unwrap();

    // Query states
    let pots_state: PotStateResponse = app
        .wrap()
        .query_wasm_smart(pp_addr.clone(), &QueryMsg::PotState { pot_id: 1 })
        .unwrap();
    let expected_pots_state = PotStateResponse {
        pot: TokenAllocation {
            pot_id: 1,
            amount: Uint128::new(2_000_000),
        },
    };
    assert_eq!(pots_state, expected_pots_state);

    // Sending the minimum bet amount to pot 2. We expect to be able send the minimum bet as now the minimum bet is not average tokens based.
    let info = mock_info("user1", &coins(1_000_000, DENOM_GAME));
    allocate_tokens(&mut app, &pp_addr, &info, 2).unwrap();

    // Query states
    let pots_state: PotStateResponse = app
        .wrap()
        .query_wasm_smart(pp_addr.clone(), &QueryMsg::PotState { pot_id: 2 })
        .unwrap();
    let expected_pots_state = PotStateResponse {
        pot: TokenAllocation {
            pot_id: 2,
            amount: Uint128::new(2_000_000),
        },
    };
    assert_eq!(pots_state, expected_pots_state);

    // Elapse some time in order to enter the epoch 1 which will bump the decay factor by 0.05 the first time
    increase_app_time(&mut app, GAME_EXTEND);

    // At this point the min bet is the base 1.0 * 1.05 which gives 1.05 $DENOM
    // But the max bet amount still follows the average tokens based calculation, meaning we have now 5 initial tokens + 2 previous bets = 7tokens / 5pots * 2times = 2.8 $DENOM

    // Sending NOT enough funds,
    let info = mock_info("user1", &coins(1_000_000, DENOM_GAME));
    let res = allocate_tokens(&mut app, &pp_addr, &info, 3).unwrap_err();
    if let Some(ContractError::BidOutOfRange { min, max }) = res.downcast_ref::<ContractError>() {
        assert_eq!(*min, Uint128::new(1050000u128));
        assert_eq!(*max, Uint128::new(2800000u128));
    } else {
        panic!("Expected ContractError::BidOutOfRange");
    }

    // Query states
    let pots_state: PotStateResponse = app
        .wrap()
        .query_wasm_smart(pp_addr.clone(), &QueryMsg::PotState { pot_id: 3 })
        .unwrap();
    let expected_pots_state = PotStateResponse {
        pot: TokenAllocation {
            pot_id: 3,
            amount: Uint128::new(1_000_000), // this is still just the initial pot funds, as the allocation failed
        },
    };
    assert_eq!(pots_state, expected_pots_state);

    // Elapse some time in order to enter the epoch 5, the second before the game ends (-1 second)
    increase_app_time(&mut app, GAME_EXTEND * 5 - 1); // as we already elapsed the first epoch previously

    // Here the min bet it should be 1 + (0.05 * 5) = 1.25 $DENOM and the extend_count is still 0

    // Sending NOT enough funds,
    let info = mock_info("user1", &coins(1_200_000, DENOM_GAME));
    let res = allocate_tokens(&mut app, &pp_addr, &info, 3).unwrap_err();
    if let Some(ContractError::BidOutOfRange { min, max }) = res.downcast_ref::<ContractError>() {
        assert_eq!(*min, Uint128::new(1250000u128));
        assert_eq!(*max, Uint128::new(2800000u128));
    } else {
        panic!("Expected ContractError::BidOutOfRange");
    }
    // Sending the minimum bet amount to pot 3. We expect to be able send the minimum bet as now the minimum bet is not average tokens based.
    let info = mock_info("user1", &coins(1_250_000, DENOM_GAME));
    allocate_tokens(&mut app, &pp_addr, &info, 3).unwrap();
    let pots_state: PotStateResponse = app
        .wrap()
        .query_wasm_smart(pp_addr.clone(), &QueryMsg::PotState { pot_id: 3 })
        .unwrap();
    let expected_pots_state = PotStateResponse {
        pot: TokenAllocation {
            pot_id: 3,
            amount: Uint128::new(2_250_000),
        },
    };
    assert_eq!(pots_state, expected_pots_state);

    // Time has been reset to -GAME_EXTEND, but we should be still be in epoch 5.
    // This time game_extend_count is 1

    // Here the min bet it should be 1 + (0.05 * 5 * 2) = 1.5 $DENOM and the extend_count is still 0
    // Max bet should be 7 + 1.25 = 8.25 $DENOM / 5 = 1.65 * 2 = 3.3 $DENOM

    // Sending NOT enough funds,
    let info = mock_info("user1", &coins(1_450_000, DENOM_GAME));
    let res = allocate_tokens(&mut app, &pp_addr, &info, 4).unwrap_err();
    if let Some(ContractError::BidOutOfRange { min, max }) = res.downcast_ref::<ContractError>() {
        assert_eq!(*min, Uint128::new(1500000u128));
        assert_eq!(*max, Uint128::new(3300000u128));
    } else {
        panic!("Expected ContractError::BidOutOfRange");
    }
    // Sending the minimum bet amount to pot 3. We expect to be able send the minimum bet as now the minimum bet is not average tokens based.
    let info = mock_info("user1", &coins(1_500_000, DENOM_GAME));
    allocate_tokens(&mut app, &pp_addr, &info, 4).unwrap();
    let pots_state: PotStateResponse = app
        .wrap()
        .query_wasm_smart(pp_addr.clone(), &QueryMsg::PotState { pot_id: 4 })
        .unwrap();
    let expected_pots_state = PotStateResponse {
        pot: TokenAllocation {
            pot_id: 4,
            amount: Uint128::new(2_500_000),
        },
    };
    assert_eq!(pots_state, expected_pots_state);
}

// TODO_FUTURE: Test NFT allocation bid discount for Raffle NFT contract hodlers
