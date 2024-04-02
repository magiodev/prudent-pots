# Prudent Pots

Prudent Pot is a strategic, interactive game where players allocate tokens to various pots on a virtual game board. Each
pot has unique rules determining the winning conditions. Players must strategically decide where to place their tokens,
considering the actions of other players and the specific rules of each pot. The game emphasizes strategic thinking,
forecasting, and adaptability.

<img src="./frontend/src/assets/logo.png">

## Game Rules and Concept

### Initial Setup

The game board consists of a row of 5 pots. Each pot has a set of unique rules for winning. Initially, the contract
balance is split equally among the five pots. This could be funds provided by the developer for game instantiation or
remaining funds from losing pots in the previous game.

### Game Duration

Each game of Prudent Pot lasts for a duration specified by `GameConfig.game_duration`. The game's duration can extend if
an allocation or reallocation happens within the last `GameConfig.game_extend` seconds, resetting the timer. After the
distribution of winnings at the end of a game, the next game starts immediately.

### Pot Rules

- **Pot 1 (Highest Pot):** Wins if it has the highest token count.
- **Pot 2 (Median Pot):** Wins if it holds the median number of tokens.
- **Pot 3 (Lowest Pot):** Wins if it has the lowest token count.
- **Pot 4 (Even Pot):** Wins if it holds an even number of tokens.
- **Pot 5 (Odd Pot):** Wins if its token count is a odd number.

### Token Allocation and Reallocation

- **Allocation**: Players can allocate tokens to any pot without an allocation fee. However, a winning fee is deducted
  from the total winnings before distribution.
- **Reallocation**: Players can reallocate tokens to a different pot, incurring a `GameConfig.fee_reallocation`%
  reallocation fee that contributes
  to the next game's pool. This fee encourages players to make thoughtful decisions when reallocating.

### Dynamic Bid Constraints and Reallocation Limits

- **Minimum Bid**: The minimum amount a player can allocate or reallocate is dynamically set based on the average token
  count across all pots. This prevents players from placing insignificantly small bets and ensures engagement with the
  game's strategic elements.
- **Maximum Bid**: The maximum bid is set to double the average token count across all pots, preventing overwhelmingly
  large bets that could unbalance the game.
- **Reallocation Limits**: During reallocation, players must adhere to the same minimum and maximum bid constraints,
  ensuring consistency and fairness in strategic decisions throughout the game.

These rules are designed to promote strategic depth, prevent exploitation, and ensure a balanced and engaging game
experience for all players.

### Winning Pot Determination

- The game ends at a predetermined time or upon meeting a specific condition.
- The winning pot is determined by its unique rule set.
- Players in winning pots receive a proportional share of the total tokens, after deducting the winning fee, including
  redistributed tokens from losing pots.

For detailed gameplay examples and strategic insights, see [Example Scenarios](./ExampleScenarios.md).

### Mathematical Formulation

#### Token Allocation and Pot Dynamics

- When a player allocates `x` tokens to pot `i`, the new total is `P_i = P_i + x`.
- If `y` tokens are reallocated from pot `i` to pot `j`, then `P_i = P_i - y` and `P_j = P_j + y * (1 - f_r/100)`,
  where `f_r` is the reallocation fee percentage.

#### Redistribution of Losing Pots Tokens

- 50% of tokens in losing pots are rolled over to the next game's pool.
- The remaining 50% are distributed to the winning pots, where players receive a share based on their contribution,
  after the winning fee is deducted.

### Strategic Considerations

The game involves elements of game theory and statistical analysis, as players must anticipate others' actions and adapt
their strategies accordingly.

- **Predictive Analysis**: Players might try to predict others' actions, especially near the game's end, to place their
  tokens in the winning pot.
- **Game Theory**: The game involves elements of game theory, where players must consider the best strategies not just
  based on the game's rules but also on other players' potential actions.
- **Statistical Analysis**: Over multiple games, patterns might emerge in player behaviors or winning pots, which can be
  analyzed statistically to inform future strategies.