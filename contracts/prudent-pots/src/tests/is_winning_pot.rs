#[cfg(test)]
mod tests {
    use cosmwasm_std::{
        coins,
        testing::{mock_dependencies_with_balance, mock_env, mock_info},
        Addr, Uint128,
    };

    use crate::{helpers::is_winning_pot, tests::instantiate::tests::setup_game};

    #[test]
    fn identify_all_winning_pots_for_median() {
        // Setup
        let mut deps = mock_dependencies_with_balance(&coins(50, "token"));
        let env = mock_env();
        let info = mock_info(Addr::unchecked("sender").as_str(), &coins(50, "token"));
        setup_game(
            deps.as_mut(),
            &env,
            info,
            Some(vec![
                (1, Addr::unchecked("player1"), Uint128::new(17)), // Median
                (2, Addr::unchecked("player1"), Uint128::new(17)), // Highest
                (3, Addr::unchecked("player1"), Uint128::new(21)), // Even
                (4, Addr::unchecked("player1"), Uint128::new(15)), // Lowest
                (5, Addr::unchecked("player1"), Uint128::new(0)),  // Prime
            ]),
        );

        // Pot 1 has 27 tokens and should be the median in this setup
        let result = is_winning_pot(&mut deps.storage, 1).unwrap();
        assert_eq!(
            result, true,
            "Pot 1 should be winning as it has the median token count when tie-breaking by pot ID"
        );

        // Ensure that other pots are not falsely reported as winners
        let result = is_winning_pot(&mut deps.storage, 2).unwrap();
        assert_eq!(result, false, "Pot 2 should not be winning.");
        let result = is_winning_pot(&mut deps.storage, 3).unwrap();
        assert_eq!(result, false, "Pot 3 should not be winning.");
        let result = is_winning_pot(&mut deps.storage, 4).unwrap();
        assert_eq!(result, false, "Pot 4 should not be winning.");
        let result = is_winning_pot(&mut deps.storage, 5).unwrap();
        assert_eq!(result, false, "Pot 5 should not be winning.");
    }

    #[test]
    fn is_winning_pot_highest() {
        // Setup
        let mut deps = mock_dependencies_with_balance(&coins(50, "token"));
        let env = mock_env();
        let info = mock_info(Addr::unchecked("sender").as_str(), &coins(50, "token"));
        setup_game(
            deps.as_mut(),
            &env,
            info,
            Some(vec![
                (1, Addr::unchecked("player1"), Uint128::new(0)), // Median
                (2, Addr::unchecked("player1"), Uint128::new(50)), // Highest
                (3, Addr::unchecked("player1"), Uint128::new(21)), // Even
                (4, Addr::unchecked("player1"), Uint128::new(15)), // Lowest
                (5, Addr::unchecked("player1"), Uint128::new(0)), // Prime
            ]),
        );

        // Pot 2 has 60 tokens and should be the highest in this setup
        let result = is_winning_pot(&mut deps.storage, 2).unwrap();
        assert_eq!(
            result, true,
            "Pot 2 should be winning as it has the highest token count when"
        );

        // Ensure that other pots are not falsely reported as winners
        let result = is_winning_pot(&mut deps.storage, 1).unwrap();
        assert_eq!(result, false, "Pot 1 should not be winning.");
        let result = is_winning_pot(&mut deps.storage, 3).unwrap();
        assert_eq!(result, false, "Pot 3 should not be winning.");
        let result = is_winning_pot(&mut deps.storage, 4).unwrap();
        assert_eq!(result, false, "Pot 4 should not be winning.");
        let result = is_winning_pot(&mut deps.storage, 5).unwrap();
        assert_eq!(result, false, "Pot 5 should not be winning.");
    }

    #[test]
    fn is_winning_pot_even() {
        // Setup
        let mut deps = mock_dependencies_with_balance(&coins(50, "token"));
        let env = mock_env();
        let info = mock_info(Addr::unchecked("sender").as_str(), &coins(50, "token"));
        setup_game(
            deps.as_mut(),
            &env,
            info,
            Some(vec![
                (1, Addr::unchecked("player1"), Uint128::new(0)), // Median
                (2, Addr::unchecked("player1"), Uint128::new(20)), // Highest
                (3, Addr::unchecked("player1"), Uint128::new(50)), // Even
                (4, Addr::unchecked("player1"), Uint128::new(15)), // Lowest
                (5, Addr::unchecked("player1"), Uint128::new(0)), // Prime
            ]),
        );

        // Pot 3 has 60 tokens and should be the even in this setup
        let result = is_winning_pot(&mut deps.storage, 3).unwrap();
        assert_eq!(
            result, true,
            "Pot 3 should be winning as it has the even token count when"
        );

        // Ensure that other pots are not falsely reported as winners
        let result = is_winning_pot(&mut deps.storage, 1).unwrap();
        assert_eq!(result, false, "Pot 1 should not be winning.");
        let result = is_winning_pot(&mut deps.storage, 2).unwrap();
        assert_eq!(result, false, "Pot 2 should not be winning.");
        let result = is_winning_pot(&mut deps.storage, 4).unwrap();
        assert_eq!(result, false, "Pot 4 should not be winning.");
        let result = is_winning_pot(&mut deps.storage, 5).unwrap();
        assert_eq!(result, false, "Pot 5 should not be winning.");
    }

    #[test]
    fn is_winning_pot_lowest() {
        // Setup
        let mut deps = mock_dependencies_with_balance(&coins(50, "token"));
        let env = mock_env();
        let info = mock_info(Addr::unchecked("sender").as_str(), &coins(50, "token"));
        setup_game(
            deps.as_mut(),
            &env,
            info,
            Some(vec![
                (1, Addr::unchecked("player1"), Uint128::new(2)), // Median
                (2, Addr::unchecked("player1"), Uint128::new(20)), // Highest
                (3, Addr::unchecked("player1"), Uint128::new(51)), // Even
                (4, Addr::unchecked("player1"), Uint128::new(1)), // Lowest
                (5, Addr::unchecked("player1"), Uint128::new(10)), // Prime
            ]),
        );

        // Pot 4 has 1 tokens and should be the lowest in this setup
        let result = is_winning_pot(&mut deps.storage, 4).unwrap();
        assert_eq!(
            result, true,
            "Pot 4 should be winning as it has the lowest token count when"
        );

        // Ensure that other pots are not falsely reported as winners
        let result = is_winning_pot(&mut deps.storage, 1).unwrap();
        assert_eq!(result, false, "Pot 1 should not be winning.");
        let result = is_winning_pot(&mut deps.storage, 2).unwrap();
        assert_eq!(result, false, "2 should not be winning.");
        let result = is_winning_pot(&mut deps.storage, 3).unwrap();
        assert_eq!(result, false, "Pot 3 should not be winning.");
        let result = is_winning_pot(&mut deps.storage, 5).unwrap();
        assert_eq!(result, false, "Pot 5 should not be winning.");
    }

    #[test]
    fn is_winning_pot_prime() {
        // Setup
        let mut deps = mock_dependencies_with_balance(&coins(50, "token"));
        let env = mock_env();
        let info = mock_info(Addr::unchecked("sender").as_str(), &coins(50, "token"));
        setup_game(
            deps.as_mut(),
            &env,
            info,
            Some(vec![
                (1, Addr::unchecked("player1"), Uint128::new(22)), // Median
                (2, Addr::unchecked("player1"), Uint128::new(20)), // Highest
                (3, Addr::unchecked("player1"), Uint128::new(51)), // Even
                (4, Addr::unchecked("player1"), Uint128::new(14)), // Lowest
                (5, Addr::unchecked("player1"), Uint128::new(13)), // Prime
            ]),
        );

        // Pot 5 has 3 tokens and should be the prime in this setup
        let result = is_winning_pot(&mut deps.storage, 5).unwrap();
        assert_eq!(
            result, true,
            "Pot 5 should be winning as it has the prime token count when"
        );

        // Ensure that other pots are not falsely reported as winners
        let result = is_winning_pot(&mut deps.storage, 1).unwrap();
        assert_eq!(result, false, "Pot 1 should not be winning.");
        let result = is_winning_pot(&mut deps.storage, 2).unwrap();
        assert_eq!(result, false, "Pot 2 should not be winning.");
        let result = is_winning_pot(&mut deps.storage, 3).unwrap();
        assert_eq!(result, false, "Pot 3 should not be winning.");
        let result = is_winning_pot(&mut deps.storage, 4).unwrap();
        assert_eq!(result, false, "Pot 4 should not be winning.");
    }
}
