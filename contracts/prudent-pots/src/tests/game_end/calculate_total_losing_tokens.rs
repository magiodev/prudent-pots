#[cfg(test)]
mod tests {
    use cosmwasm_std::{testing::mock_dependencies, Addr, Uint128};

    use crate::{
        helpers::calculate_total_losing_tokens,
        tests::helpers::{setup_pots, setup_pots_and_allocations},
    };

    #[test]
    fn total_losing_tokens_single_winner() {
        let mut deps = mock_dependencies();
        setup_pots(
            &mut deps.as_mut(),
            vec![
                Uint128::new(10),
                Uint128::new(20),
                Uint128::new(30),
                Uint128::new(40),
                Uint128::new(50),
            ],
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
        let mut deps = mock_dependencies();
        setup_pots(
            &mut deps.as_mut(),
            vec![
                Uint128::new(10),
                Uint128::new(20),
                Uint128::new(30),
                Uint128::new(40),
                Uint128::new(50),
            ],
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
        let mut deps = mock_dependencies();
        setup_pots(
            &mut deps.as_mut(),
            vec![
                Uint128::new(10),
                Uint128::new(20),
                Uint128::new(30),
                Uint128::new(40),
                Uint128::new(50),
            ],
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
        let mut deps = mock_dependencies();
        setup_pots(
            &mut deps.as_mut(),
            vec![
                Uint128::new(10),
                Uint128::new(20),
                Uint128::new(30),
                Uint128::new(40),
                Uint128::new(50),
            ],
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
        let mut deps = mock_dependencies();

        let player1 = Addr::unchecked("player1");
        let player2 = Addr::unchecked("player2");

        // Setup pots and player allocations
        setup_pots_and_allocations(
            &mut deps.as_mut(),
            vec![
                Uint128::new(10),
                Uint128::new(10),
                Uint128::new(10),
                Uint128::new(10),
                Uint128::new(10),
            ],
            vec![
                (1, player1.clone(), Uint128::new(10)), // Player 1 allocates 10 tokens to pot 1
                (2, player1.clone(), Uint128::new(20)), // Player 1 allocates 20 tokens to pot 2
                (1, player2.clone(), Uint128::new(15)), // Player 2 allocates 15 tokens to pot 1
                (3, player2.clone(), Uint128::new(25)), // Player 2 allocates 25 tokens to pot 3
            ],
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
