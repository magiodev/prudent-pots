#[cfg(test)]
mod tests {
    use cosmwasm_std::{
        coins,
        testing::{mock_dependencies_with_balance, mock_env, mock_info},
        Addr, Uint128,
    };

    use crate::{
        execute::game_end,
        state::{GAME_STATE, POT_STATES, REALLOCATION_FEE_POOL},
        tests::helpers::setup_game,
    };

    #[test]
    fn test_game_end_with_no_player_allocations() {
        // Setup
        let mut deps = mock_dependencies_with_balance(&coins(50, "token"));
        let env = mock_env();
        let info = mock_info(Addr::unchecked("sender").as_str(), &coins(50, "token"));
        setup_game(
            deps.as_mut(),
            &env,
            info,
            Some(vec![]), // Some(vec![(2, Addr::unchecked("player1"), Uint128::new(2940))])
        );

        // Test case

        // TODO: Mock +3600 after setup_game

        game_end(deps.as_mut(), env).unwrap();

        let new_game_state = GAME_STATE.load(deps.as_ref().storage).unwrap();
        // assert!(
        //     new_game_state.start_time > env.block.time.seconds(),
        //     "New game should start in the future"
        // );

        for pot_id in 1..=5 {
            let pot_state = POT_STATES.load(deps.as_ref().storage, pot_id).unwrap();
            assert!(
                pot_state.amount.gt(&Uint128::zero()),
                "Pot {} should have initial tokens for the next game",
                pot_id
            );
        }

        let reallocation_fee_pool = REALLOCATION_FEE_POOL.load(deps.as_ref().storage).unwrap();
        assert_eq!(
            reallocation_fee_pool,
            Uint128::zero(),
            "Reallocation fee pool should be reset to zero for the next game"
        );
    }
}
