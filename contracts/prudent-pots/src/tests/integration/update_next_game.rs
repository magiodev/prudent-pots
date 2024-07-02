use crate::{
    msg::{GameStateResponse, QueryMsg, RaffleResponse},
    tests::integration::{
        fixtures::{default_with_balances, ADMIN_ADDRESS, DENOM_GAME},
        helpers::{game_end, mint_nfts, update_next_game},
    },
};
use cosmwasm_std::{coin, coins, testing::mock_info, Addr, Uint128};
use cw721::OwnerOfResponse;
use cw_multi_test::Executor;

use super::fixtures::{increase_app_time, GAME_DURATION};

#[test]
fn test_update_next_game_works() {
    let (mut app, pp_addr, cw721_addr) = default_with_balances(
        5,
        vec![coin(100_000_000u128, DENOM_GAME.to_string())],
        None,
        None,
    );

    // Game state extend_count after
    let game_state: GameStateResponse = app
        .wrap()
        .query_wasm_smart(&pp_addr, &QueryMsg::GameState {})
        .unwrap();
    assert_eq!(game_state.state.extend_count, 0);
    assert_eq!(game_state.state.round_count, 1);

    // Increase time by GAME_DURATION second to make the game expire
    increase_app_time(&mut app, GAME_DURATION);

    // Compute next game start time
    let next_game_start = app.block_info().time.plus_seconds(GAME_DURATION).seconds();

    // Game end and set NO raffle prizes
    let info = mock_info(ADMIN_ADDRESS, &vec![]);
    game_end(&mut app, &pp_addr, &info, None, None, Some(next_game_start)).unwrap();

    // Assert game_state.start_time is now in the future
    let game_state: GameStateResponse = app
        .wrap()
        .query_wasm_smart(&pp_addr, &QueryMsg::GameState {})
        .unwrap();
    assert_eq!(game_state.state.start_time, next_game_start);

    // Compute next game start time double in the future after game_end has been executed with a lower start_time
    let next_game_start = app
        .block_info()
        .time
        .plus_seconds(GAME_DURATION * 2)
        .seconds();

    // Update the next_game_start to be more in the future
    update_next_game(
        &mut app,
        &pp_addr,
        &mock_info(ADMIN_ADDRESS, &vec![]),
        None,
        None,
        Some(next_game_start), // Schedule it in the future
    )
    .unwrap();

    // Assert game_state.start_time is now in the future
    let game_state: GameStateResponse = app
        .wrap()
        .query_wasm_smart(&pp_addr, &QueryMsg::GameState {})
        .unwrap();
    let raffle_state: RaffleResponse = app
        .wrap()
        .query_wasm_smart(&pp_addr, &QueryMsg::Raffle {})
        .unwrap();
    assert_eq!(game_state.state.start_time, next_game_start);
    assert_eq!(raffle_state.raffle.cw721_token_id, None);
    assert_eq!(raffle_state.raffle.cw721_addr, None);

    // mint 10x nfts as minter to admin
    mint_nfts(&mut app, &cw721_addr, 1, 2, Addr::unchecked(ADMIN_ADDRESS));

    // ApproveAll as admin to operator pp contract
    app.execute_contract(
        Addr::unchecked(ADMIN_ADDRESS),
        cw721_addr.clone(),
        &cw721::Cw721ExecuteMsg::ApproveAll {
            operator: pp_addr.to_string(),
            expires: None,
        },
        &vec![],
    )
    .unwrap();

    // Update the NFT to be some, from no NFT. we should approve it first
    update_next_game(
        &mut app,
        &pp_addr,
        &mock_info(ADMIN_ADDRESS, &vec![]),
        Some("1".to_string()),
        Some(cw721_addr.to_string()),
        None, // Schedule it in the future
    )
    .unwrap();
    let game_state: GameStateResponse = app
        .wrap()
        .query_wasm_smart(&pp_addr, &QueryMsg::GameState {})
        .unwrap();
    let raffle_state: RaffleResponse = app
        .wrap()
        .query_wasm_smart(&pp_addr, &QueryMsg::Raffle {})
        .unwrap();
    assert_eq!(game_state.state.start_time, next_game_start);
    assert_eq!(raffle_state.raffle.cw721_token_id, Some("1".to_string()));
    assert_eq!(raffle_state.raffle.cw721_addr, Some(cw721_addr.to_string()));
    assert_eq!(raffle_state.raffle.denom_amount, Uint128::zero());

    // Assert nft ownership after
    let nft_owner: OwnerOfResponse = app
        .wrap()
        .query_wasm_smart(
            &cw721_addr,
            &cw721::Cw721QueryMsg::OwnerOf {
                token_id: "1".to_string(),
                include_expired: None,
            },
        )
        .unwrap();
    assert_eq!(nft_owner.owner, Addr::unchecked(&pp_addr));

    // Try update the NFT with another one, we should get an error as InvalidNft
    update_next_game(
        &mut app,
        &pp_addr,
        &mock_info(ADMIN_ADDRESS, &vec![]),
        Some("2".to_string()),
        Some(cw721_addr.to_string()),
        None, // Schedule it in the future
    )
    .unwrap_err(); // todo: assert type of error

    // Update the next game again sending some funds and check the Raffle state before after
    update_next_game(
        &mut app,
        &pp_addr,
        &mock_info(ADMIN_ADDRESS, &coins(10_000_000, DENOM_GAME)),
        None,
        None,
        None, // Schedule it in the future
    )
    .unwrap(); // todo: assert type of error

    let raffle_state: RaffleResponse = app
        .wrap()
        .query_wasm_smart(&pp_addr, &QueryMsg::Raffle {})
        .unwrap();
    assert_eq!(
        raffle_state.raffle.denom_amount,
        Uint128::new(10_000_000u128)
    );
}
