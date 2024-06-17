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

- **Pot 1 (Lowest Pot):** Wins if it has the lowest token count.
- **Pot 2 (Even Pot):** Wins if it holds an even number of tokens.
- **Pot 3 (Median Pot):** Wins if it holds the median number of tokens.
- **Pot 4 (Odd Pot):** Wins if its token count is a odd number.
- **Pot 5 (Highest Pot):** Wins if it has the highest token count.

### Token Allocation and Reallocation

- **Allocation Rules**: Players can allocate tokens to any pot without an allocation fee. Each player is allowed to allocate tokens to a pot only once per game, preventing multiple allocations to the same pot. This rule is designed to ensure that players must carefully consider their initial strategic decisions.
- **Reallocation Rules**: During reallocation, players are not bound by minimum or maximum bid constraints. However, a `GameConfig.fee_reallocation`% reallocation fee is still applied, contributing to the next game's pool. This fee encourages players to make thoughtful decisions when reallocating their tokens.

### Allocation Dynamic Bid Constraints and Limits

- **Minimum Bid**: The minimum amount a player can allocate is dynamically set based on the average token
  count across all pots. This prevents players from placing insignificantly small bets and ensures engagement with the
  game's strategic elements.
- **Maximum Bid**: The maximum bid a player can allocate is set to double the average token count across all pots, preventing overwhelmingly
  large bets that could unbalance the game.
- **Other Limits**: Additionally, players can both allocate or reallocate tokens only to empty pots.

### Winning Pot Determination

The winning pot is determined by its specific rules, and players in this pot receive their proportional share
of the total tokens, post the deduction of a winning fee, along with redistributed
tokens from the less successful pots.

### Redistribution of Losing Pots Tokens

- 50% of tokens in losing pots are rolled over to the next game's pool.
- The remaining 50% are now distributed proportionally to the winning pots based on the amount of tokens in each winning pot. This ensures that the distribution reflects the level of risk and investment players have put into each pot.