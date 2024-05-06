use cosmwasm_std::{coin, Addr, Uint128};

use crate::msg::{ExecuteMsg, GameConfigResponse, QueryMsg};
use crate::state::GameConfig;
use crate::tests::integration::fixtures::{default_with_balances, DENOM_GAME, GAME_DURATION};
use crate::tests::integration::helpers::update_config;

#[test]
fn test_update_config_works() {
    let (mut app, pp_addr, _cw721_addr) =
        default_with_balances(1, vec![coin(100_000_000u128, DENOM_GAME.to_string())], None);

    // Update config as admin
    let res = update_config(
        &mut app,
        &pp_addr,
        &ExecuteMsg::UpdateConfig {
            fee: Some(10),
            fee_reallocation: Some(10),
            fee_address: Some(Addr::unchecked("new_address")),
            game_denom: Some("new_denom".to_string()),
            game_cw721_addrs: vec![Addr::unchecked("test")],
            game_duration: Some(GAME_DURATION * 2),
            game_extend: Some(600 * 3),
            min_pot_initial_allocation: Some(Uint128::new(1_000_000u128)),
            decay_factor: Some(Uint128::new(50u128)),
        },
    )
    .unwrap();
    assert!(!res.events.is_empty());

    // Query states

    let new_config: GameConfigResponse = app
        .wrap()
        .query_wasm_smart(pp_addr.clone(), &QueryMsg::GameConfig {})
        .unwrap();

    let expected_new_config = GameConfigResponse {
        config: GameConfig {
            fee: 10,
            fee_reallocation: 10,
            fee_address: Addr::unchecked("new_address"),
            game_denom: "new_denom".to_string(),
            game_cw721_addrs: vec![Addr::unchecked("test")],
            game_duration: GAME_DURATION * 2,
            game_extend: 600 * 3,
            min_pot_initial_allocation: Uint128::new(1_000_000u128),
            decay_factor: Uint128::new(50u128),
        },
    };
    assert_eq!(new_config, expected_new_config);
}
