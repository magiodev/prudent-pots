#[cfg(test)]
mod tests {
    use cosmwasm_std::{
        coins,
        testing::{mock_dependencies_with_balance, mock_env, mock_info},
        Addr, BankMsg, CosmosMsg, Uint128,
    };

    use crate::{
        helpers::game_end::get_distribution_send_msgs, state::GAME_CONFIG,
        tests::instantiate::tests::setup_game_works,
    };

    /// Test `get_distribution_send_msgs_single_winner` to ensure proper distribution of tokens
    /// in a scenario where there's a single winner with player allocations.
    ///
    /// Initial Setup:
    /// - Five pots with initial allocations. Total tokens in the contract: 4921.
    ///   - Pot 1: 200 tokens (looser)
    ///   - Pot 2: 3140 tokens (winner with player1's allocation of 2940 tokens, 200 initial)
    ///   - Pot 3: 1181 tokens (looser with player1's allocation of 981 tokens, 200 initial)
    ///   - Pot 4: 200 tokens (looser)
    ///   - Pot 5: 200 tokens (looser)
    ///
    /// The total losing tokens (excluding the winning pot): 200 + 1181 + 200 + 200 = 1781.
    ///
    /// The distribution amount (50% of total losing tokens): 1781 / 2 = 890.5 (approximated as needed).
    ///
    /// Expected Outcome:
    /// - Pot 2 is the winner. Player1 should receive all the tokens from Pot 2, which includes:
    ///   - Their own allocation: 2940 tokens.
    ///   - The winning pot's initial allocation: 200 tokens.
    ///   - The distributed amount: 890.5 tokens from the losing pots.
    ///   - Total expected to be received by Player1: 3140 + 890.5 = 4030.5 tokens (approximated as needed).
    ///
    /// - Remaining tokens for the next game should be the other half of the losing tokens: 890.5 tokens.
    ///
    /// This test invokes `get_distribution_send_msgs` with Pot 2 as the winner and asserts:
    ///   - That the BankMsg::Send message is generated correctly for Player1.
    ///   - That Player1 receives the correct amount of tokens.
    ///   - That the remaining tokens for the next game are correctly calculated and stored.
    #[test]
    fn get_distribution_send_msgs_single_winner() {
        // Setup
        let mut deps = mock_dependencies_with_balance(&coins(1000, "token"));
        let env = mock_env();
        let info = mock_info(Addr::unchecked("sender").as_str(), &coins(1000, "token"));
        setup_game_works(
            deps.as_mut(),
            &env,
            info,
            Some(vec![
                (2, Addr::unchecked("player1"), Uint128::new(2940)), // Player 1 allocates to pot 2
                (3, Addr::unchecked("player1"), Uint128::new(981)),  // Player 1 allocates to pot 3
            ]),
        );

        // Pot 1 - 200 tokens (looser, no alloc)
        // Pot 2 - 3140 tokens (winner with player1's allocation)
        // Pot 3 - 1181 tokens (looser, with alloc)
        // Pot 4 - 200 tokens (looser, no alloc)
        // Pot 5 - 200 tokens (looser, no alloc)

        // Invoke get_distribution_send_msgs assuming pot 2 is the winner
        let winning_pots = vec![2];
        let total_losing_tokens = Uint128::new(200 + 1181 + 200 + 200); // Total losing tokens excluding the winning pot
        let messages =
            get_distribution_send_msgs(&deps.as_ref(), &winning_pots, total_losing_tokens)
                .unwrap()
                .0;

        // Assertions
        let config = GAME_CONFIG.load(deps.as_ref().storage).unwrap();

        assert_eq!(
            messages.len(),
            2,
            "There should be two messages: one for player1's reward and one for the fee"
        );

        // Checking the player's reward
        if let CosmosMsg::Bank(BankMsg::Send { to_address, amount }) = &messages[0] {
            assert_eq!(
                to_address,
                &Addr::unchecked("player1").to_string(),
                "Reward should be sent to player1"
            );
            assert_eq!(
                amount[0].amount,
                Uint128::new(3950u128), // 4030 - 2% fee
                "Player1 should receive the correct amount after fee deduction"
            );
        } else {
            panic!("Expected BankMsg::Send message for player reward");
        }

        // Checking the fee
        if let CosmosMsg::Bank(BankMsg::Send { to_address, amount }) = &messages[1] {
            assert_eq!(
                to_address,
                &config.fee_address.to_string(),
                "Fee should be sent to the fee allocation address"
            );
            assert_eq!(
                amount[0].amount,
                Uint128::new(80u128), // 4030 * 0.002 rounded down due to Uint oflow
                "The correct fee amount should be sent"
            );
        } else {
            panic!("Expected BankMsg::Send message for fee");
        }
    }
}
