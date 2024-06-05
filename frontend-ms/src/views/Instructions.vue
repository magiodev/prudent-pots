<template>
  <div class="instructions container">
    <div class="row py-5">
      <div class="col-12">
        <h2>Mad Pots Game Rules</h2>
        <p>Welcome to Mad Pots, the ultimate experiment zone where mad scientists meet to test their wits and
          strategies! Here, brilliant minds compete by betting on unique pots with simple yet intriguing winning rules.
          Are you ready to dive into the world of Mad Pots and claim your share of the prize?</p>

        <hr/>

        <h3>Setup and Strategy</h3>
        <p>In Mad Pots, players face off against each other by betting on 5 different pots. The goal is to guess which
          pot will win. By doing so, you can win a portion of the tokens in the winning pots and have a chance at the
          raffle prizes. Each round lasts at least 24 hours, with potential extensions based on player activity. All
          pots start with an equal balance of tokens. Get ready to place your bets and strategize to come out on
          top!</p>

        <hr/>

        <h3>Game Duration and Restart Mechanics</h3>
        <p>Each round lasts for 24 hours. Here's the twist: if any player makes a move within the last 60 minutes of the
          round, the timer will reset to 60 minutes, regardless of when the move was made within that period. This means
          the game can continuously extend if players keep making moves during the final hour, effectively prolonging
          the round indefinitely as long as moves are made. Pure madness!</p>
        <p>After the winners are determined, there will be a short break to set up raffle prizes and check the pots. The
          Mad Pots admin will then restart the next game. If the admin hasn't restarted the game within the first hour,
          anyone can initiate the restart after the 60 minutes have passed. Take over the control of the Lab!</p>

        <hr/>

        <h3>Pot Rules</h3>
        <ul>
          <li><strong>Pot 1 (Lowest Pot):</strong> This pot wins when it has the least amount of tokens among all other
            pots.
          </li>
          <li><strong>Pot 2 (Even Pot):</strong> This pot wins with an even number of tokens on the latest decimal digit
            (e.g. 7.123456 is considered even).
          </li>
          <li><strong>Pot 3 (Median Pot):</strong> This pot wins by having the median token count, defined as any number
            between the 2nd and 4th values when sorted.
          </li>
          <li><strong>Pot 4 (Odd Pot):</strong> This pot wins with an odd number of tokens on the latest decimal digit
            (e.g. 10.654321 is considered odd).
          </li>
          <li><strong>Pot 5 (Highest Pot):</strong> This pot wins when it has the most tokens among all other pots.</li>
        </ul>

        <hr/>

        <h3>Token Allocation and Reallocation</h3>
        <h4>Allocate Tokens:</h4>
        <ul>
          <li>You can put tokens in any empty pot for free. No sneaky fees!</li>
          <li>You can't place more than one bet in the same pot.</li>
          <li>Allocations (placing new bets) have limits based on the average token count.</li>
        </ul>
        <h4>Reallocate Tokens:</h4>
        <ul>
          <li>If you want to move your tokens around, it'll cost you a {{ gameConfig.fee_reallocation }}% fee.</li>
          <li>You can only move tokens to pots where you currently don't have any.</li>
          <li>Reallocations can be done only 10 times in the same round.</li>
        </ul>

        <hr/>

        <h3>Dynamic Bid Constraints</h3>
        <p>To keep things fair and strategic, minimum and maximum bet limits are set based on the average number of
          tokens across all pots. However, these limits only apply to new allocations, not reallocations.</p>
        <p>A general rule that always applies is that a single pot cannot have more tokens allocated to it than the
          combined total of the other pots. This prevents mad scientists from collectively targeting specific pots, such
          as the highest or the even/odd pots.</p>
        <p>No Cartels allowed in the experiment zone! So, plan your strategy wisely,
          or risk falling into the chaos of the Mad Pots laboratory!</p>

        <hr/>

        <h3>Winning Pot Determination and Prize Distribution</h3>
        <ul>
          <li><strong>Determining the Winner:</strong> Each pot has its own rule to decide the winner. The pot that
            meets its rule wins.
          </li>
          <li><strong>Prize Distribution:</strong> Winners in these pots receive their share of the total tokens. This
            includes the initial tokens allocated to the pot and half of the losing pots' tokens, after a
            {{ gameConfig.fee }}% winning fee is deducted.
          </li>
          <li><strong>Redistribution of Losing Pots:</strong> The tokens from the losing pots are redistributed among
            the winning pots based on the number of tokens in each winning pot.
          </li>
          <li><strong>Dividing the Tokens:</strong> The redistributed tokens are then divided proportionally among the
            bettors of each winning pot.
          </li>
        </ul>

        <hr/>

        <h3>Raffle Prize</h3>
        <p>The player who bets the most tokens and wins also gets an extra surprise: they win the raffle! The raffle
          prize could include a Mad Scientists NFT and/or $OSMO tokens. Each game extension slightly reduces the tokens
          raffle prize, with the reduced amount being refunded to the treasury. So, keep an eye on the clock!</p>

        <hr/>

        <h3>Fees and Future Games</h3>
        <p>All collected fees and half of the losing pots' token amounts will be used to fund the next games, ensuring
          the madness continues!</p>

        <hr/>

        <h3>Disclaimer</h3>
        <p>Remember that none of this is financial advice. The game can be much more complex due to reallocations, real
          bid constraints, and other factors not covered in this simplified example. Always play responsibly and
          understand the rules fully before participating. By entering Mad Pots, you accept that the outcomes are based
          on the rules of the game and that winning is never guaranteed. Enjoy the madness, but keep your wits about
          you!</p>
        <p>EVERYTHING IS AN EXPERIMENT!</p>

        <p>For more detailed gameplay examples and strategic tips, head over to our Telegram group.</p>
        <p>Happy potting, and may the craziest scientist win!</p>

        <hr/>

        <div class="accordion" id="accordionExample">
          <div class="accordion-item">
            <h2 class="accordion-header">
              <button class="accordion-button collapsed" type="button" data-bs-toggle="collapse"
                      data-bs-target="#collapseThree" aria-expanded="false" aria-controls="collapseThree">
                Example and Clarifications
              </button>
            </h2>
            <div id="collapseThree" class="accordion-collapse collapse" data-bs-parent="#accordionExample">
              <div class="accordion-body">
                ### Initial Setup


                * The round starts with **1.0 tokens in each pot**, for a total of **5.0 tokens**.
                * This round is also funded with **1 MS NFT** and **100 $OSMO** as the raffle prize.


                ### Player Bets


                * **Alice** bets **2.0 tokens** on Pot 1 (Lowest Pot).
                * **Bob** bets **10.654321 tokens** on Pot 2 (Even Pot).
                * **Carol** bets **7.0 tokens** on Pot 3 (Median Pot).
                * **Dave** bets **5.123456 tokens** on Pot 4 (Odd Pot).
                * **Eve** bets **15.0 tokens** on Pot 5 (Highest Pot).


                #### Winners Criteria Application


                #### Considering the 1.0 initial token allocated per pot:


                * **Pot 1 (Lowest Pot)**: Ends up with **3.0 tokens**, the least among all pots, so it wins.
                * **Pot 2 (Even Pot)**: Ends up with **11.654321 tokens**. Since it's an odd number, it loses.
                * **Pot 3 (Median Pot)**: Ends up with **8.0 tokens**. The token counts across all pots are 3.000000,
                6.123456, 8.000000, 11.654321, and 16.0. The median is 8.000000, so it wins.
                * **Pot 4 (Odd Pot)**: Ends up with **7.123456 tokens**. Since it's an even number, it loses.
                * **Pot 5 (Highest Pot)**: Ends up with **16.0 tokens**, the most among all pots, so it wins.


                #### **Conclusion and Prize Distribution:**


                * **Winners**: Alice, Carol, and Eve bet on winning pots.
                * **Losers**: Bob and Dave lost their bets.
                * **Losing Pots Tokens**: Pot 2 and Pot 4 contain a total of **17.777777 tokens** (10.654321 +
                7.123456).
                * **Half** of the losing pots' tokens (**8.888888 tokens**) will be split among the winning pots.
                * The other half (**8.888888 tokens**) will be reserved for the next round, starting with **1.777777
                tokens per pot**.


                ### Distribution of Winning Pot Tokens


                #### **Winning Shares calculation**

                Considering the total tokens among winning pots: 3 + 8 + 16 = 27


                * **Alice**:
                * Share: 3 / 27 * 100 ≈ 10.0%
                * Additional tokens: 1.0 + 8.888888 * 0.1 = 1.888888
                * Bet: 2.0, Win: 1.888888, Receive: 3.888888
                * **Carol**:
                * Share: 8 / 27 * 100 ≈ 30.0%
                * Additional tokens: 1.0 + 8.888888 * 0.3 = 3.666666
                * Bet: 7.0, Win: 3.666666, Receive: 10.666666
                * **Eve**:
                * Share: 16 / 27 * 100 ≈ 60.0%
                * Additional tokens: 1.0 + 8.888888 * 0.6 = 6.333332
                * Bet: 15.0, Win: 6.333332, Receive: 21.333332


                #### **Winning Fee Deduction**


                * A **5% winning fee** will be deducted from the total amounts of winning pots.
                * Alice will receive: 3.888888 * 0.95 = 3.694443 (fee 0.194445)
                * Carol will receive: 10.666666 * 0.95 = 10.133332 (fee 0.533334)
                * Eve will receive: 21.333332 * 0.95 = 20.266665 (fee 1.066667)


                #### **Raffle Prize Assignation**


                * **Eve** was the player with the most tokens placed. She wins both the **MS NFT** and the additional
                **100 $OSMO** prize!


                ### Game Extension Example

                Imagine the game is about to end in 7 minutes. Someone makes a move, and boom—the game timer resets to
                60 minutes. Now, the new end time is 1 hour from the time of the last move. And the situation changes
                drastically. Will you make a move? LFG!
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import {mapGetters} from "vuex";
import mxGame from "../../../frontend-common/mixin/game";

export default {
  name: 'InstructionsView',

  mixins: [mxGame],

  computed: {
    ...mapGetters(['gameConfig']),

    gameDuration() {
      const totalSeconds = this.gameConfig.game_duration;

      const days = Math.floor(totalSeconds / (60 * 60 * 24));
      const hours = Math.floor((totalSeconds / (60 * 60)) % 24);
      const minutes = Math.floor((totalSeconds / 60) % 60);
      const seconds = totalSeconds % 60;

      let durationString = "";
      if (days > 0) {
        durationString += `${days} day${days > 1 ? "s" : ""}, `;
      }
      if (hours > 0 || days > 0) {
        durationString += `${hours} hour${hours > 1 ? "s" : ""}, `;
      }
      durationString += `${minutes} minute${minutes > 1 ? "s" : ""} and ${seconds} second${seconds > 1 ? "s" : ""}`;

      return durationString;
    }
  }
};
</script>

<style lang="scss" scoped>
@import "@/assets/style.scss";

.instructions {
  border-left: 1px solid $pp-color-5;
  border-right: 1px solid $pp-color-5;
}
</style>