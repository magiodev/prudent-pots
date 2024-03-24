# Prudent Pot

## High-Level Description

Prudent Pot is a strategic, interactive game where players allocate tokens to various pots on a virtual game board. Each pot has unique rules determining the winning conditions. Players must strategically decide where to place their tokens, considering the actions of other players and the specific rules of each pot. The game emphasizes strategic thinking, forecasting, and adaptability.

## Game Rules and Concept

### Initial Setup

- The game begins with a predefined number of pots, each with a set of unique rules for winning.
- An initial amount of tokens is placed in one or more pots to start the game and encourage participation.

Game board is a row of 5 pots, each pot stands independently with its unique set of rules for winning, and players allocate their tokens to these pots based on their strategic decisions.

---

### Gameplay

#### Token Allocation and Reallocation

- Players can allocate tokens to any pot of their choice.
- Each allocation is subject to a fee, deducted from the tokens being placed.
- Players have the option to reallocate their tokens to a different pot, incurring a reallocation fee.

- **Initial Allocation**: When a player allocates \( x \) tokens to a pot, those tokens are added to the pot's total. If the player allocates tokens to pot \( i \), then \( P_i = P_i + x \).
- **Reallocation**: If a player decides to move \( y \) tokens from pot \( i \) to pot \( j \), then \( P_i = P_i - y \) and \( P_j = P_j + y \).

#### Winning Pot Determination

- The game ends at a predetermined time or upon meeting a specific condition.
- The winning pot is determined based on its unique set of rules, such as having a median number of tokens, an even number, or a unique number closest to a target.

Assume there are \( N \) pots with varying rules for winning. Let's consider a few example rules:

- **Median Pot**: The pot with the median amount of tokens wins. If there are \( P_i \) tokens in pot \( i \), and pots are sorted in ascending order by the number of tokens, the median pot is the one where \( i = \lfloor N/2 \rfloor \) if \( N \) is odd, or an average of \( i = N/2 \) and \( i = N/2 + 1 \) pots if \( N \) is even.
- **Even Pot**: The pot with an even number of tokens wins. If multiple pots have an even number of tokens, the one with the largest amount wins.
- **Unique Number Pot**: The pot with a unique number of tokens (no other pot has the same number of tokens) and closest to a specified target number wins.

---

### Mathematical Formulation

#### Token Allocation and Pot Dynamics

Let \( P_i \) represent the number of tokens in pot \( i \).

When a player allocates \( x \) tokens to pot \( i \), the new total is \( P_i = P_i + x \cdot (1 - \frac{f_a}{100}) \), where \( f_a \) is the allocation fee percentage.

If \( y \) tokens are reallocated from pot \( i \) to pot \( j \), then \( P_i = P_i - y \) and \( P_j = P_j + y \cdot (1 - \frac{f_r}{100}) \), where \( f_r \) is the reallocation fee percentage.

#### Winning Pot Determination

- **Median Pot**: The pot with the median number of tokens is determined by sorting the pots in ascending order of their token counts. The median pot is identified based on the total number of pots.
- **Even Pot**: The pot with the highest even number of tokens is considered the winning pot.
- **Unique Number Pot**: The pot whose token count is unique and closest to a predefined target wins.

#### Distribution at Game End

The winning share for a player who contributed \( a \) tokens to the winning pot \( k \) is calculated as \( \frac{a}{T_k} \times \sum_{i=1}^{N} P_i \), where \( T_k \) is the total tokens in pot \( k \) and \( \sum_{i=1}^{N} P_i \) is the sum of tokens in all pots. 

Let's say pot \( k \) wins. If a player contributed \( a \) tokens out of a total of \( T_k \) tokens in pot \( k \), their winning share (excluding any fees from last-minute entries) is \( \frac{a}{T_k} \times \sum_{i=1}^{N} P_i \), where \( \sum_{i=1}^{N} P_i \) is the sum of tokens in all pots.

#### Fees

- **Allocation Fee**: If there's an allocation fee of \( f_a \% \), and a player allocates \( x \) tokens, \( x(1 - f_a/100) \) tokens go to the pot, and \( x(f_a/100) \) are deducted as fees.
- **Reallocation Fee**: For a reallocation fee of \( f_r \% \), when \( y \) tokens are reallocated, \( y(1 - f_r/100) \) tokens move to the new pot, and \( y(f_r/100) \) are deducted.

---

### Strategic Considerations

The game involves elements of game theory and statistical analysis, as players must anticipate others' actions and adapt their strategies accordingly.

- **Predictive Analysis**: Players might try to predict others' actions, especially near the game's end, to place their tokens in the winning pot.
- **Game Theory**: The game involves elements of game theory, where players must consider the best strategies not just based on the game's rules but also on other players' potential actions.
- **Statistical Analysis**: Over multiple games, patterns might emerge in player behaviors or winning pots, which can be analyzed statistically to inform future strategies.