#[cfg(test)]
mod tests {
    use cosmwasm_std::{
        coins,
        testing::{mock_dependencies, mock_dependencies_with_balance, mock_env},
        Addr, Uint128,
    };

    use crate::{
        execute::game_end,
        helpers::prepare_next_game,
        state::{GameConfig, GAME_CONFIG, GAME_STATE, POT_STATES, REALLOCATION_FEE_POOL},
        tests::helpers::{setup_game_end, setup_pots},
    };

    #[test]
    fn prepare_next_game_works() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let config = GameConfig {
            game_duration: 3600, // 1 hour in seconds
            game_extend: 600,
            fee_allocation: 2,   // Assuming a 2% allocation fee
            fee_reallocation: 5, // Assuming a 5% reallocation fee
            fee_allocation_address: Addr::unchecked("fee_address"), // An example fee address
            game_denom: "token".to_string(),
            min_bid: Uint128::new(1000000u128),
        };
        GAME_CONFIG.save(deps.as_mut().storage, &config).unwrap();

        // Simulate some initial state for pots and reallocation fee pool
        let initial_pots = vec![10, 10, 10, 10, 10]
            .into_iter()
            .map(Uint128::new)
            .collect();

        setup_pots(&mut deps.as_mut(), initial_pots);
        REALLOCATION_FEE_POOL
            .save(deps.as_mut().storage, &Uint128::new(100))
            .unwrap(); // example value

        // Invoke prepare_next_game
        prepare_next_game(deps.as_mut(), &env, &vec![]).unwrap();

        // Verify GAME_STATE
        let game_state = GAME_STATE.load(deps.as_mut().storage).unwrap();
        assert_eq!(game_state.start_time, env.block.time.seconds() + 1);
        assert_eq!(
            game_state.end_time,
            env.block.time.seconds() + 1 + config.game_duration
        );

        // TODO: Fix this
        // Verify pots have been reset and include the reallocation fee pool
        // let expected_token_per_pot = (deps
        //     .querier
        //     .query_all_balances(&env.contract.address)
        //     .unwrap()
        //     .iter()
        //     .sum::<Coin>()
        //     .amount
        //     + Uint128::new(100))
        // .u128()
        //     / 5;
        // for pot_id in 1..=5 {
        //     let pot_state = POT_STATES.load(deps.as_mut().storage, pot_id).unwrap();
        //     assert_eq!(pot_state, Uint128::new(expected_token_per_pot));
        // }

        // Verify reallocation fee pool reset
        let reallocation_fee_pool = REALLOCATION_FEE_POOL.load(deps.as_mut().storage).unwrap();
        assert_eq!(reallocation_fee_pool, Uint128::zero());
    }

    #[test]
    fn test_game_end_with_no_player_allocations() {
        let mut deps = mock_dependencies_with_balance(&coins(5000, "token"));
        let mut env = mock_env();
        setup_game_end(&mut deps.as_mut(), &mut env);

        game_end(deps.as_mut(), env.clone()).unwrap();

        let new_game_state = GAME_STATE.load(deps.as_ref().storage).unwrap();
        assert!(
            new_game_state.start_time > env.block.time.seconds(),
            "New game should start in the future"
        );

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
