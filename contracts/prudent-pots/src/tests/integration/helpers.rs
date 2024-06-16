use cosmwasm_std::{Addr, Empty, MessageInfo};
use cw_multi_test::{error::AnyError, App, AppResponse, Executor};

use crate::{msg::ExecuteMsg, tests::integration::fixtures::MINTER_ADDRESS};

use super::fixtures::ADMIN_ADDRESS;

// PP

pub fn update_config(
    app: &mut App,
    pp_addr: &Addr,
    update_config_msg: &ExecuteMsg,
) -> Result<AppResponse, AnyError> {
    app.execute_contract(
        Addr::unchecked(ADMIN_ADDRESS),
        pp_addr.clone(),
        update_config_msg,
        &[],
    )
}

pub fn allocate_tokens(
    app: &mut App,
    pp_addr: &Addr,
    info: &MessageInfo,
    pot_id: u8,
) -> Result<AppResponse, AnyError> {
    app.execute_contract(
        info.sender.clone(),
        pp_addr.clone(),
        &ExecuteMsg::AllocateTokens { pot_id },
        &info.funds,
    )
}

pub fn reallocate_tokens(
    app: &mut App,
    pp_addr: &Addr,
    info: &MessageInfo,
    from_pot_id: u8,
    to_pot_id: u8,
) -> Result<AppResponse, AnyError> {
    app.execute_contract(
        info.sender.clone(),
        pp_addr.clone(),
        &ExecuteMsg::ReallocateTokens {
            from_pot_id,
            to_pot_id,
        },
        &info.funds,
    )
}

pub fn game_end(
    app: &mut App,
    pp_addr: &Addr,
    info: &MessageInfo,
    raffle_cw721_token_id: Option<String>,
    raffle_cw721_token_addr: Option<String>,
    next_game_start: Option<u64>,
) -> Result<AppResponse, AnyError> {
    app.execute_contract(
        info.sender.clone(),
        pp_addr.clone(),
        &ExecuteMsg::GameEnd {
            raffle_cw721_token_id,
            raffle_cw721_token_addr,
            next_game_start,
        },
        &info.funds,
    )
}

// CW721

pub fn mint_nfts(app: &mut App, cw721_addr: &Addr, start_id: u64, count: u64, to_addr: Addr) {
    for i in start_id..count + 1 {
        let _res = app
            .execute_contract(
                Addr::unchecked(MINTER_ADDRESS),
                cw721_addr.clone(),
                &cw721_base::msg::ExecuteMsg::<Option<Empty>, Empty>::Mint {
                    token_id: i.to_string(),
                    owner: to_addr.to_string(),
                    token_uri: None,
                    extension: None,
                },
                &vec![],
            )
            .unwrap();
    }
}
