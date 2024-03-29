#[cfg(test)]
mod tests {
    use cosmwasm_std::{testing::mock_dependencies, Uint128};

    use crate::{helpers::is_winning_pot, tests::helpers::setup_pots};

    #[test]
    fn identify_all_winning_pots_for_median() {
        let mut deps = mock_dependencies();

        setup_pots(
            &mut deps.as_mut(),
            vec![
                Uint128::new(27), // Median
                Uint128::new(27), // Highest
                Uint128::new(31), // Even
                Uint128::new(25), // Lowest
                Uint128::new(10), // Prime
            ],
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
        let mut deps = mock_dependencies();

        setup_pots(
            &mut deps.as_mut(),
            vec![
                Uint128::new(10), // Median
                Uint128::new(60), // Highest
                Uint128::new(31), // Even
                Uint128::new(25), // Lowest
                Uint128::new(10), // Prime
            ],
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
        let mut deps = mock_dependencies();

        setup_pots(
            &mut deps.as_mut(),
            vec![
                Uint128::new(10), // Median
                Uint128::new(30), // Highest
                Uint128::new(60), // Even
                Uint128::new(25), // Lowest
                Uint128::new(10), // Prime
            ],
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
        let mut deps = mock_dependencies();

        setup_pots(
            &mut deps.as_mut(),
            vec![
                Uint128::new(32), // Median
                Uint128::new(30), // Highest
                Uint128::new(61), // Even
                Uint128::new(1),  // Lowest
                Uint128::new(10), // Prime
            ],
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
        let mut deps = mock_dependencies();

        setup_pots(
            &mut deps.as_mut(),
            vec![
                Uint128::new(32), // Median
                Uint128::new(30), // Highest
                Uint128::new(61), // Even
                Uint128::new(4),  // Lowest
                Uint128::new(3),  // Prime
            ],
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
