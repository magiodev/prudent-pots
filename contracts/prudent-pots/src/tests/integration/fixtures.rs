use std::str::FromStr;

use cosmwasm_std::testing::mock_info;
use cosmwasm_std::{coin, coins, Addr, BlockInfo, Coin, Decimal, Empty, Uint128};
use cw_multi_test::{App, AppBuilder, BankKeeper, Contract, ContractWrapper, Executor};

use crate::msg::{ExecuteMsg, GameConfigResponse, InstantiateMsg, QueryMsg, UpdateGameConfig};
use crate::state::{GameConfig, Raffle};
use crate::tests::integration::helpers::{game_end, mint_nfts, update_config};

pub const DENOM_GAME: &str = "udenom";
pub const GAME_DURATION: u64 = 3600;
pub const GAME_EXTEND: u64 = 600;

pub const ADMIN_ADDRESS: &str = "admin_address";
pub const ADMIN_BALANCE: u128 = 1_000_000_000_000u128;
pub const MINTER_ADDRESS: &str = "merlin";

const CONTRACT_NAME: &str = "Magic Power";
const SYMBOL: &str = "MGK";

fn pp_contract() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        crate::contract::execute,
        crate::contract::instantiate,
        crate::contract::query,
    )
    .with_reply(crate::contract::reply);
    Box::new(contract)
}

fn cw721_contract() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        cw721_base::entry::execute,
        cw721_base::entry::instantiate,
        cw721_base::entry::query,
    );
    Box::new(contract)
}

fn instantiate_cw721(app: &mut App, code_id: u64, msg: cw721_base::InstantiateMsg) -> Addr {
    app.instantiate_contract(
        code_id,
        Addr::unchecked(ADMIN_ADDRESS),
        &msg,
        &[],
        "cw721",
        Some(ADMIN_ADDRESS.to_string()),
    )
    .unwrap()
}

fn instantiate_pp(app: &mut App, code_id: u64, msg: InstantiateMsg, funds: Vec<Coin>) -> Addr {
    let res = app
        .instantiate_contract(
            code_id,
            Addr::unchecked(ADMIN_ADDRESS),
            &msg,
            &funds,
            "prudent-pots",
            Some(ADMIN_ADDRESS.to_string()),
        )
        .unwrap();
    res
}

pub fn default_with_balances(
    num_users: u8,
    initial_balance: Vec<Coin>,
    raffle: Option<Raffle>,
    next_game_start_offset: Option<u64>,
) -> (App, Addr, Addr) {
    // Create a vector to hold the balances setup
    let mut balances = vec![(
        Addr::unchecked(ADMIN_ADDRESS),
        vec![coin(ADMIN_BALANCE, DENOM_GAME)],
    )];

    // Generate user addresses and their initial balances
    for i in 1..=num_users {
        let user_address = Addr::unchecked(format!("user{}", i));
        balances.push((user_address, initial_balance.clone()));
    }

    // Create a new bank keeper and setup the app with initial balances
    let bank = BankKeeper::new();

    let mut app = AppBuilder::new()
        .with_bank(bank)
        .build(|router, _api, storage| {
            // Initialize balances for each account
            for (account, amount) in balances {
                router.bank.init_balance(storage, &account, amount).unwrap();
            }
        });

    // check admin balance
    let balance = app.wrap().query_balance(ADMIN_ADDRESS, DENOM_GAME).unwrap();
    assert_eq!(balance, coin(ADMIN_BALANCE, DENOM_GAME));

    // instantiate cw721 contract
    let cw721_id = app.store_code(cw721_contract());
    let cw721_msg = cw721_base::InstantiateMsg {
        name: CONTRACT_NAME.to_string(),
        symbol: SYMBOL.to_string(),
        minter: String::from(MINTER_ADDRESS),
    };
    let cw721_addr = instantiate_cw721(&mut app, cw721_id, cw721_msg);

    // Instantiate Prudent Pots contract with 5 tokens (1 per pot). Conditionally based on Raffle option field.
    let pp_id = app.store_code(pp_contract());
    let pp_addr: Addr;
    let mut pp_msg: InstantiateMsg;
    match raffle {
        Some(raffle) => {
            // Mint Nfts as minter to admin address
            pp_msg = InstantiateMsg {
                config: GameConfig {
                    fee: 5,
                    fee_reallocation: 5,
                    fee_address: Addr::unchecked("treasury_addr"),
                    game_denom: DENOM_GAME.to_string(),
                    game_cw721_addrs: vec![Addr::unchecked(&cw721_addr)],
                    game_duration: 1, // we hardcode 1 here in order to let the game expire inmediately, so we execute the raffle init wflow (this could be avoided by instantiating a predicatable contract)
                    game_duration_epoch: GAME_EXTEND, // we hardcode 1 here in order to let the game expire inmediately, so we execute the raffle init wflow (this could be avoided by instantiating a predicatable contract)
                    game_extend: GAME_EXTEND,
                    game_end_threshold: GAME_EXTEND,
                    min_pot_initial_allocation: Uint128::new(1_000_000u128),
                    decay_factor: Decimal::from_str("0.05").unwrap(),
                    reallocations_limit: 10,
                },
                next_game_start: None,
            };
            pp_addr = instantiate_pp(
                &mut app,
                pp_id,
                pp_msg.clone(),
                coins(5_000_000, DENOM_GAME),
            );

            // mint 10x nfts as minter to admin
            mint_nfts(&mut app, &cw721_addr, 1, 10, Addr::unchecked(ADMIN_ADDRESS));

            // ApproveAll as admin to operator pp
            let _ = app.execute_contract(
                Addr::unchecked(ADMIN_ADDRESS),
                cw721_addr.clone(),
                &cw721::Cw721ExecuteMsg::ApproveAll {
                    operator: pp_addr.to_string(),
                    expires: None,
                },
                &vec![],
            );

            // Update config to extend game duration
            update_config(
                &mut app,
                &pp_addr,
                &ExecuteMsg::UpdateConfig {
                    config: Box::new(UpdateGameConfig {
                        fee: None,
                        fee_reallocation: None,
                        fee_address: None,
                        game_denom: None,
                        game_cw721_addrs: vec![Addr::unchecked(&cw721_addr)], // set the same to avoid updating
                        game_duration: Some(GAME_DURATION),
                        game_duration_epoch: None,
                        game_extend: None,
                        game_end_threshold: None,
                        min_pot_initial_allocation: None,
                        decay_factor: None,
                        reallocations_limit: None,
                    }),
                },
            )
            .unwrap();
            pp_msg.config.game_duration = GAME_DURATION; // this is to make the below assert pass

            // Increase time to expire game
            increase_app_time(&mut app, 2);

            let next_game_start_time = match next_game_start_offset {
                Some(offset) => Some(app.block_info().time.plus_seconds(offset).seconds()),
                None => None,
            };

            // Game end to start the first real round (this would break the counter and so start from 1)
            // sending 100 extra $DENOM as raffle prize + 1 NFT
            game_end(
                &mut app,
                &pp_addr,
                &mock_info(
                    ADMIN_ADDRESS,
                    &coins(raffle.denom_amount.into(), DENOM_GAME), // raffle denom
                ),
                raffle.cw721_token_id,        // raffle nft prize
                Some(cw721_addr.to_string()), // overriding the None passed from outside as contract wasnt instantiated yet
                next_game_start_time,
            )
            .unwrap();
        }
        None => {
            pp_msg = InstantiateMsg {
                config: GameConfig {
                    fee: 5,
                    fee_reallocation: 5,
                    fee_address: Addr::unchecked("treasury_addr"),
                    game_denom: DENOM_GAME.to_string(),
                    game_cw721_addrs: vec![Addr::unchecked(&cw721_addr)],
                    game_duration: GAME_DURATION,
                    game_duration_epoch: GAME_EXTEND,
                    game_extend: GAME_EXTEND,
                    game_end_threshold: GAME_EXTEND,
                    min_pot_initial_allocation: Uint128::new(1_000_000u128),
                    decay_factor: Decimal::from_str("0.05").unwrap(),
                    reallocations_limit: 10,
                },
                next_game_start: match next_game_start_offset {
                    Some(offset) => Some(app.block_info().time.plus_seconds(offset).seconds()),
                    None => None,
                },
            };
            pp_addr = instantiate_pp(
                &mut app,
                pp_id,
                pp_msg.clone(),
                coins(5_000_000, DENOM_GAME),
            );
        }
    };

    // If your CustomApp has a different interface for queries, adjust this section accordingly
    let query_msg = QueryMsg::GameConfig {};
    let config: GameConfigResponse = app
        .wrap()
        .query_wasm_smart(pp_addr.clone(), &query_msg)
        .unwrap();

    // Validation and assertions remain the same
    let expected_config = GameConfigResponse {
        config: pp_msg.config,
    };
    assert_eq!(config, expected_config);

    (app, pp_addr, cw721_addr)
}

// UTILITIES

pub fn increase_app_time(app: &mut App, seconds: u64) {
    let block_info = app.block_info();
    app.set_block(BlockInfo {
        height: block_info.height + 1,
        time: block_info.time.plus_seconds(seconds),
        chain_id: block_info.chain_id,
    });
}
