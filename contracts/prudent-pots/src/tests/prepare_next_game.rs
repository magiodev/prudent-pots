#[cfg(test)]
mod tests {
    use cosmwasm_std::{
        coins,
        testing::{mock_dependencies_with_balance, mock_env, mock_info},
        Addr, Uint128,
    };

    use crate::{
        helpers::game_end::prepare_next_game,
        state::{GAME_STATE, POT_STATES, REALLOCATION_FEE_POOL},
        tests::instantiate::tests::setup_game_no_raffle_works,
    };

    #[test]
    fn prepare_next_game_no_raffle_works() {
        // Setup
        let mut deps = mock_dependencies_with_balance(&coins(1300, "token")); // 1000 initial + 200 player alloc on pot 5 + 100 reallocation fee pool
        let env = mock_env();
        // Instantiating with 100 tokens, 20 each pot
        let info = mock_info(Addr::unchecked("sender").as_str(), &coins(1000, "token"));
        setup_game_no_raffle_works(
            deps.as_mut(),
            &env,
            info,
            Some(vec![
                (5, Addr::unchecked("player1"), Uint128::new(100u128)), // allocating 100 tokens on pot 5, which will be a looser cause is not Odd
            ]),
        );

        // Override reallocation fee pool, so we will distribute 20 tokens each pot from previous game
        REALLOCATION_FEE_POOL
            .save(deps.as_mut().storage, &Uint128::new(100))
            .unwrap();

        // Test case

        prepare_next_game(deps.as_mut(), &env, Uint128::zero(), None, None).unwrap();

        // Verify new GAME_STATE after running prepare next game
        let game_state = GAME_STATE.load(deps.as_mut().storage).unwrap();
        assert_eq!(game_state.start_time, env.block.time.seconds() + 1);
        assert_eq!(game_state.end_time, env.block.time.seconds() + 1 + 3600);

        // @deprecated as now this is a game_end responsibility: Verify reallocation fee pool reset
        // let reallocation_fee_pool = REALLOCATION_FEE_POOL.load(deps.as_mut().storage).unwrap();
        // assert_eq!(reallocation_fee_pool, Uint128::zero());

        // TODO: PlayerAllocations reset assertion

        // TODO: total_outgoing_tokens assertion variant!

        // Verify pots have been allocated with initial funds, plus user's allocation and include the reallocation fee pool
        for pot_id in 1..=5 {
            let pot_state = POT_STATES.load(deps.as_mut().storage, pot_id).unwrap();
            assert_eq!(pot_state.amount, Uint128::new(260u128)); // 1300 / 5 = 260
        }
    }

    // TODO: prepare_next_game_raffle_works
}
