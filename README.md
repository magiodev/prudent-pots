# Prudent Pot

## High-Level Description

Prudent Pot is a strategic, interactive game where players allocate tokens to various pots on a virtual game board. Each pot has unique rules determining the winning conditions. Players must strategically decide where to place their tokens, considering the actions of other players and the specific rules of each pot. The game emphasizes strategic thinking, forecasting, and adaptability.

## Game Rules and Concept

### Initial Setup

- The game board consists of a row of 5 pots.
- Each pot has a set of unique rules for winning.
- Initially, 10 tokens are placed in each pot to encourage participation.

### Gameplay

#### Token Allocation and Reallocation

- Players can allocate tokens to any pot.
- Each allocation is subject to a 5% fee, deducted from the tokens being placed.
- Players can reallocate tokens to a different pot, incurring a 10% reallocation fee.

#### Pot Rules

- **Pot 1 (Median Pot):** Wins if it holds the median number of tokens.
- **Pot 2 (Highest Pot):** Wins if it has the highest token count.
- **Pot 3 (Even Pot):** Wins if it holds an even number of tokens.
- **Pot 4 (Lowest Pot):** Wins if it has the lowest token count.
- **Pot 5 (Prime Pot):** Wins if its token count is a prime number.

#### Winning Pot Determination

- The game ends at a predetermined time or upon meeting a specific condition.
- The winning pot is determined by its unique rule set.

### Mathematical Formulation

#### Token Allocation and Pot Dynamics

- When a player allocates `x` tokens to pot `i`, the new total is `P_i = P_i + x * (1 - f_a/100)`, where `f_a` is the allocation fee percentage.
- If `y` tokens are reallocated from pot `i` to pot `j`, then `P_i = P_i - y` and `P_j = P_j + y * (1 - f_r/100)`, where `f_r` is the reallocation fee percentage.

#### Distribution at Game End

- The winning share for a player in pot `k` is `a/T_k * ΣP_i`, where `a` is the player's contribution to pot `k`, `T_k` is the total tokens in pot `k`, and `ΣP_i` is the sum of tokens in all pots.

#### Fees

- **Allocation Fee:** `x(1 - f_a/100)` tokens go to the pot, and `x(f_a/100)` are deducted as fees.
- **Reallocation Fee:** `y(1 - f_r/100)` tokens move to the new pot, and `y(f_r/100)` are deducted.

### Strategic Considerations

The game involves elements of game theory and statistical analysis, as players must anticipate others' actions and adapt their strategies accordingly.

- **Predictive Analysis**: Players might try to predict others' actions, especially near the game's end, to place their tokens in the winning pot.
- **Game Theory**: The game involves elements of game theory, where players must consider the best strategies not just based on the game's rules but also on other players' potential actions.
- **Statistical Analysis**: Over multiple games, patterns might emerge in player behaviors or winning pots, which can be analyzed statistically to inform future strategies.