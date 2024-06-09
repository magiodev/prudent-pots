use cosmwasm_std::{coin, coins, testing::mock_info, Uint128};

use crate::msg::{PotStateResponse, QueryMsg};
use crate::state::TokenAllocation;
use crate::tests::integration::fixtures::{default_with_balances, DENOM_GAME};
use crate::tests::integration::helpers::allocate_tokens;
use crate::ContractError;

#[test]
fn test_allocate_tokens_works() {
    let (mut app, pp_addr, _cw721_addr) =
        default_with_balances(1, vec![coin(100_000_000u128, DENOM_GAME.to_string())], None);

    // Sending enough funds
    let info = mock_info("user1", &coins(1_000_000, DENOM_GAME));

    // Allocate
    let _res = allocate_tokens(&mut app, &pp_addr, &info, 1);
    // assert!(!res.events.is_empty());

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

// TODO_FUTURE: Test NFT allocation bid discount for Raffle NFT contract hodlers

#[test]
fn test_allocate_tokens_fails() {
    let (mut app, pp_addr, _cw721_addr) =
        default_with_balances(1, vec![coin(100_000_000u128, DENOM_GAME.to_string())], None);

    // Sending too little funds
    let info = mock_info("user1", &coins(1, DENOM_GAME));

    // Attempt to allocate tokens
    let res = allocate_tokens(&mut app, &pp_addr, &info, 1);

    // Check if the result is an error and if it matches the expected error
    match res {
        Err(e) => match e.downcast_ref::<ContractError>() {
            Some(ContractError::BidOutOfRange { min, max }) => {
                assert_eq!(*min, Uint128::new(1000000u128));
                assert_eq!(*max, Uint128::new(2000000u128));
            }
            Some(_) => panic!("Unexpected contract error type"),
            None => panic!("Expected ContractError::BidOutOfRange"),
        },
        _ => panic!("Expected an error"),
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
