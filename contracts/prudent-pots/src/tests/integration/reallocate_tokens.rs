use cosmwasm_std::{coin, coins, testing::mock_info, Uint128};

use crate::msg::{PotsStateResponse, QueryMsg};
use crate::state::TokenAllocation;
use crate::tests::integration::fixtures::{default_with_balances, DENOM_GAME};
use crate::tests::integration::helpers::{allocate_tokens, reallocate_tokens};

#[test]
fn test_reallocate_tokens_works() {
    let (mut app, pp_addr, _cw721_addr) = default_with_balances(
        1,
        vec![coin(100_000_000u128, DENOM_GAME.to_string())],
        None,
        None,
    );

    let info = mock_info("user1", &coins(1_000_000, DENOM_GAME));

    // Allocate
    allocate_tokens(&mut app, &pp_addr, &info, 1).unwrap();

    // Reallocate
    let res = reallocate_tokens(&mut app, &pp_addr, &info, 1, 2).unwrap();
    assert!(!res.events.is_empty());

    // Query states

    let pots_state: PotsStateResponse = app
        .wrap()
        .query_wasm_smart(pp_addr.clone(), &QueryMsg::PotsState {})
        .unwrap();

    let expected_pots_state = PotsStateResponse {
        pots: vec![
            TokenAllocation {
                pot_id: 1,
                amount: Uint128::new(1_000_000),
            },
            TokenAllocation {
                pot_id: 2,
                amount: Uint128::new((1_000_000 * 95 / 100) + 1_000_000),
            },
            TokenAllocation {
                pot_id: 3,
                amount: Uint128::new(1_000_000),
            },
            TokenAllocation {
                pot_id: 4,
                amount: Uint128::new(1_000_000),
            },
            TokenAllocation {
                pot_id: 5,
                amount: Uint128::new(1_000_000),
            },
        ],
    };
    assert_eq!(pots_state, expected_pots_state);
}
