# Prudent Pot

## High-Level Description

Prudent Pot is a strategic, interactive game where players allocate tokens to various pots on a virtual game board. Each pot has unique rules determining the winning conditions. Players must strategically decide where to place their tokens, considering the actions of other players and the specific rules of each pot. The game emphasizes strategic thinking, forecasting, and adaptability.

## Game Rules and Concept

### Initial Setup

- The game board consists of a row of 5 pots.
- Each pot has a set of unique rules for winning.
- Initially, 10 tokens are placed in each pot to encourage participation.

### Gameplay

## Game Duration

- Each game of Prudent Pot lasts for a fixed duration of 1 hour.
- After the distribution of winnings at the end of a game, the next game starts immediately.

#### Token Allocation and Reallocation

- Players can allocate tokens to any pot.
- An allocation fee of 2% is deducted from the tokens being placed, supporting operational costs and game development.
- Players can reallocate tokens to a different pot, incurring a 5% reallocation fee that contributes to the next game's pool.

#### Pot Rules

- **Pot 1 (Median Pot):** Wins if it holds the median number of tokens.
- **Pot 2 (Highest Pot):** Wins if it has the highest token count.
- **Pot 3 (Even Pot):** Wins if it holds an even number of tokens.
- **Pot 4 (Lowest Pot):** Wins if it has the lowest token count.
- **Pot 5 (Prime Pot):** Wins if its token count is a prime number.

#### Winning Pot Determination

- The game ends at a predetermined time or upon meeting a specific condition.
- The winning pot is determined by its unique rule set.

For detailed gameplay examples and strategic insights, see [Example Scenarios](./ExampleScenarios.md).

### Mathematical Formulation

#### Token Allocation and Pot Dynamics

- When a player allocates `x` tokens to pot `i`, the new total is `P_i = P_i + x * (1 - f_a/100)`, where `f_a` is the allocation fee percentage.
- If `y` tokens are reallocated from pot `i` to pot `j`, then `P_i = P_i - y` and `P_j = P_j + y * (1 - f_r/100)`, where `f_r` is the reallocation fee percentage.

#### Redistribution of Losing Pots Tokens

- 50% of tokens in losing pots are rolled over to the next game's pool.
- The remaining 50% are distributed to the winning pots, where players receive a share based on their contribution.

#### Distribution at Game End

- Players in winning pots receive a proportional share of the total tokens, including redistributed tokens from losing pots.

### Strategic Considerations

The game involves elements of game theory and statistical analysis, as players must anticipate others' actions and adapt their strategies accordingly.

- **Predictive Analysis**: Players might try to predict others' actions, especially near the game's end, to place their tokens in the winning pot.
- **Game Theory**: The game involves elements of game theory, where players must consider the best strategies not just based on the game's rules but also on other players' potential actions.
- **Statistical Analysis**: Over multiple games, patterns might emerge in player behaviors or winning pots, which can be analyzed statistically to inform future strategies.

