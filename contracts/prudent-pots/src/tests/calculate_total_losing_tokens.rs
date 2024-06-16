#[cfg(test)]
mod tests {
    use cosmwasm_std::{
        coins,
        testing::{mock_dependencies_with_balance, mock_env, mock_info},
        Addr, Uint128,
    };

    use crate::{
        helpers::game_end::calculate_total_losing_tokens,
        tests::instantiate::tests::setup_game_works,
    };

    #[test]
    fn total_losing_tokens_single_winner() {
        // Setup
        let mut deps = mock_dependencies_with_balance(&coins(1000, "token"));
        let env = mock_env();
        let info = mock_info(Addr::unchecked("sender").as_str(), &coins(1000, "token"));
        setup_game_works(
            deps.as_mut(),
            &env,
            info,
            Some(vec![
                (1, Addr::unchecked("player1"), Uint128::new(10)), // l
                (2, Addr::unchecked("player1"), Uint128::new(5)),  // l
                (3, Addr::unchecked("player1"), Uint128::new(20)), // w
                (4, Addr::unchecked("player1"), Uint128::new(40)), // l
                (5, Addr::unchecked("player1"), Uint128::new(30)), // l
            ]),
        );

        // Let's say pot 3 is the winner
        let total_losing_tokens = calculate_total_losing_tokens(&mut deps.storage, &[3]).unwrap();
        assert_eq!(
            total_losing_tokens,
            Uint128::new(10 + 5 + 40 + 30 + 1000 - 200), // plus initial allocations, less winner init alloc
            "Should sum tokens from all pots except pot 3"
        );
    }

    #[test]
    fn total_losing_tokens_multiple_winners() {
        // Setup
        let mut deps = mock_dependencies_with_balance(&coins(1000, "token"));
        let env = mock_env();
        let info = mock_info(Addr::unchecked("sender").as_str(), &coins(1000, "token"));
        setup_game_works(
            deps.as_mut(),
            &env,
            info,
            Some(vec![
                // pot 1 it would be winner, but it has NO player allocations so its loser. // l
                (2, Addr::unchecked("player1"), Uint128::new(10)),
                (3, Addr::unchecked("player1"), Uint128::new(20)), // w
                (4, Addr::unchecked("player1"), Uint128::new(30)),
                (5, Addr::unchecked("player1"), Uint128::new(40)), // w
            ]),
        );

        // Let's say pots 2 and 4 are winners
        let total_losing_tokens =
            calculate_total_losing_tokens(&mut deps.storage, &[3, 5]).unwrap();
        assert_eq!(
            total_losing_tokens,
            Uint128::new(10 + 30 + 1000 - 200 - 200),
            "Should sum tokens from pots 1, 2, and 4"
        );
    }

    #[test]
    fn total_losing_tokens_no_winners() {
        // Setup
        let mut deps = mock_dependencies_with_balance(&coins(1000, "token"));
        let env = mock_env();
        let info = mock_info(Addr::unchecked("sender").as_str(), &coins(1000, "token"));
        setup_game_works(
            deps.as_mut(),
            &env,
            info,
            Some(vec![
                (1, Addr::unchecked("player1"), Uint128::new(0)),
                (2, Addr::unchecked("player1"), Uint128::new(10)),
                (3, Addr::unchecked("player1"), Uint128::new(20)),
                (4, Addr::unchecked("player1"), Uint128::new(30)),
                (5, Addr::unchecked("player1"), Uint128::new(40)),
            ]),
        );

        // No winners
        let total_losing_tokens = calculate_total_losing_tokens(&mut deps.storage, &[]).unwrap();
        assert_eq!(
            total_losing_tokens,
            Uint128::new(10 + 20 + 30 + 40 + 1000),
            "Should sum all pots' tokens"
        );
    }

    #[test]
    fn total_losing_tokens_all_winners() {
        // Setup
        let mut deps = mock_dependencies_with_balance(&coins(1000, "token"));
        let env = mock_env();
        let info = mock_info(Addr::unchecked("sender").as_str(), &coins(1000, "token"));
        setup_game_works(
            deps.as_mut(),
            &env,
            info,
            Some(vec![
                (1, Addr::unchecked("player1"), Uint128::new(10)), // w
                (2, Addr::unchecked("player1"), Uint128::new(20)), // w
                (3, Addr::unchecked("player1"), Uint128::new(30)), // w
                (4, Addr::unchecked("player1"), Uint128::new(40)), // w
                (5, Addr::unchecked("player1"), Uint128::new(50)), // w
            ]),
        );

        // All pots are winners
        let total_losing_tokens =
            calculate_total_losing_tokens(&mut deps.storage, &[1, 2, 3, 4, 5]).unwrap();
        assert_eq!(
            total_losing_tokens,
            Uint128::zero(),
            "Should not sum any tokens as all pots are winners"
        );
    }

    #[test]
    fn total_losing_tokens_with_player_allocations() {
        // Setup
        let mut deps = mock_dependencies_with_balance(&coins(1000, "token"));
        let env = mock_env();
        let info = mock_info(Addr::unchecked("sender").as_str(), &coins(1000, "token"));
        setup_game_works(
            deps.as_mut(),
            &env,
            info,
            Some(vec![
                (1, Addr::unchecked("player1"), Uint128::new(10)),
                (2, Addr::unchecked("player1"), Uint128::new(20)),
                (1, Addr::unchecked("player1"), Uint128::new(15)),
                (3, Addr::unchecked("player1"), Uint128::new(25)),
            ]),
        );

        // pot 1: 200 + 25 = 225 // l
        // pot 2: 200 + 20 = 220 // w
        // pot 3: 200 + 25 = 225 // l
        // pot 4: 200 // l
        // pot 5: 200 // l

        // Let's say pot 2 is the winner
        let total_losing_tokens = calculate_total_losing_tokens(&mut deps.storage, &[2]).unwrap();
        assert_eq!(
            total_losing_tokens,
            Uint128::new(10 + 20 + 15 + 25 + 1000 - 200 - 20), // Exclude pot 2's initial tokens and allocations
            "Should sum tokens from all pots except pot 2"
        );
    }
}
