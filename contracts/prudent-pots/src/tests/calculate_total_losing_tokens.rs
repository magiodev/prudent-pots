#[cfg(test)]
mod tests {
    use cosmwasm_std::{
        coins,
        testing::{mock_dependencies_with_balance, mock_env, mock_info},
        Addr, Uint128,
    };

    use crate::{helpers::calculate_total_losing_tokens, tests::helpers::setup_game};

    #[test]
    fn total_losing_tokens_single_winner() {
        // Setup
        let mut deps = mock_dependencies_with_balance(&coins(50, "token"));
        let env = mock_env();
        let info = mock_info(Addr::unchecked("sender").as_str(), &coins(50, "token"));
        setup_game(
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

        // Let's say pot 3 is the winner
        let total_losing_tokens = calculate_total_losing_tokens(&mut deps.storage, &[3]).unwrap();
        assert_eq!(
            total_losing_tokens,
            Uint128::new(10 + 20 + 40 + 50),
            "Should sum tokens from all pots except pot 3"
        );
    }

    #[test]
    fn total_losing_tokens_multiple_winners() {
        // Setup
        let mut deps = mock_dependencies_with_balance(&coins(50, "token"));
        let env = mock_env();
        let info = mock_info(Addr::unchecked("sender").as_str(), &coins(50, "token"));
        setup_game(
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

        // Let's say pots 2 and 4 are winners
        let total_losing_tokens =
            calculate_total_losing_tokens(&mut deps.storage, &[2, 4]).unwrap();
        assert_eq!(
            total_losing_tokens,
            Uint128::new(10 + 30 + 50),
            "Should sum tokens from pots 1, 3, and 5"
        );
    }

    #[test]
    fn total_losing_tokens_no_winners() {
        // Setup
        let mut deps = mock_dependencies_with_balance(&coins(50, "token"));
        let env = mock_env();
        let info = mock_info(Addr::unchecked("sender").as_str(), &coins(50, "token"));
        setup_game(
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
            Uint128::new(10 + 20 + 30 + 40 + 50),
            "Should sum all pots' tokens"
        );
    }

    #[test]
    fn total_losing_tokens_all_winners() {
        // Setup
        let mut deps = mock_dependencies_with_balance(&coins(50, "token"));
        let env = mock_env();
        let info = mock_info(Addr::unchecked("sender").as_str(), &coins(50, "token"));
        setup_game(
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
        let mut deps = mock_dependencies_with_balance(&coins(50, "token"));
        let env = mock_env();
        let info = mock_info(Addr::unchecked("sender").as_str(), &coins(50, "token"));
        setup_game(
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

        // Let's say pot 2 is the winner
        let total_losing_tokens = calculate_total_losing_tokens(&mut deps.storage, &[2]).unwrap();
        assert_eq!(
            total_losing_tokens,
            Uint128::new(10 + 10 + 10 + 10 + 10 + 10 + 20 + 15 + 25 - 10 - 20), // Exclude pot 2's initial tokens and allocations
            "Should sum tokens from all pots except pot 2"
        );
    }
}
