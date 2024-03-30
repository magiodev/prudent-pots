#[cfg(test)]
mod tests {
    use cosmwasm_std::{
        coins,
        testing::{mock_dependencies_with_balance, mock_env, mock_info},
        Addr, BankMsg, CosmosMsg, Uint128,
    };

    use crate::{
        helpers::get_distribute_bank_msgs, state::REALLOCATION_FEE_POOL, tests::helpers::setup_game,
    };

    /// Test `get_distribute_bank_msgs_single_winner` to ensure proper distribution of tokens
    /// in a scenario where there's a single winner with player allocations.
    ///
    /// Initial Setup:
    /// - Five pots with initial allocations. Total tokens in the contract: 4921.
    ///   - Pot 1: 200 tokens (looser)
    ///   - Pot 2: 3140 tokens (winner with player1's allocation of 2940 tokens, 200 initial)
    ///   - Pot 3: 1181 tokens (looser with player1's allocation of 982 tokens, 200 initial)
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
    /// This test invokes `get_distribute_bank_msgs` with Pot 2 as the winner and asserts:
    ///   - That the BankMsg::Send message is generated correctly for Player1.
    ///   - That Player1 receives the correct amount of tokens.
    ///   - That the remaining tokens for the next game are correctly calculated and stored.
    #[test]
    fn get_distribute_bank_msgs_single_winner() {
        // Setup
        let mut deps = mock_dependencies_with_balance(&coins(1000, "token"));
        let env = mock_env();
        let info = mock_info(Addr::unchecked("sender").as_str(), &coins(1000, "token"));
        setup_game(
            deps.as_mut(),
            &env,
            info,
            Some(vec![
                (2, Addr::unchecked("player1"), Uint128::new(2940)), // Player 1 allocates to pot 2
                (3, Addr::unchecked("player1"), Uint128::new(982)),  // Player 1 allocates to pot 3
            ]),
        );

        // Pot 1 - 200 tokens (looser, no alloc)
        // Pot 2 - 3140 tokens (winner with player1's allocation)
        // Pot 3 - 1181 tokens (looser, with alloc)
        // Pot 4 - 200 tokens (looser, no alloc)
        // Pot 5 - 200 tokens (looser, no alloc)

        // Invoke get_distribute_bank_msgs assuming pot 2 is the winner
        let winning_pots = vec![2];
        let total_losing_tokens = Uint128::new(200 + 1181 + 200 + 200); // Total losing tokens excluding the winning pot
        let messages =
            get_distribute_bank_msgs(deps.as_mut().storage, &winning_pots, total_losing_tokens)
                .unwrap();

        // Assertions
        assert_eq!(
            messages.len(),
            1,
            "There should be one message for player1's reward"
        );

        if let CosmosMsg::Bank(BankMsg::Send { to_address, amount }) = &messages[0] {
            assert_eq!(
                to_address,
                &Addr::unchecked("player1").to_string(),
                "Reward should be sent to player1"
            );
            // the following fails as assertion `left == right` failed: Player1 should receive all tokens from the winning pot
            //   left: Uint128(460)
            //   right: Uint128(3140)
            assert_eq!(
                amount[0].amount,
                Uint128::new(3140),
                "Player1 should receive all tokens from the winning pot"
            );
        } else {
            panic!("Expected BankMsg::Send message");
        }

        // Assert remaining tokens for next game
        // TODO: As in this test case we didnt reallocate any allocated tokens from a pot to another, this should be 0. 890 it should be the amount left in the contract balance, that will be used for the next game.
        let remaining_for_next_game = REALLOCATION_FEE_POOL.load(deps.as_ref().storage).unwrap();
        assert_eq!(
            remaining_for_next_game,
            Uint128::new(890),
            "Remaining tokens for next game should be 890"
        );
    }
}
