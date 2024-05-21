<template>
  <div class="instructions container">
    <div class="row py-5">
      <div class="col">
        <h3>Initial Setup</h3>
        <p>The game board features a row of 5 pots, each with unique winning rules. Initially, the contract balance is
          evenly distributed among these pots, sourced either from the developer's funds for game instantiation or
          remaining funds from previous games' losing pots.</p>

        <h3>Game Duration</h3>
        <p>Each round lasts {{ gameDuration }}, with potential extensions if players make allocation or
          reallocation actions within the last {{ gameConfig.game_extend }} seconds of the game, resetting the timer.
          Following the distribution of winnings, the next game starts immediately.</p>

        <h3>Pot Rules</h3>
        <ul>
          <li><strong>Pot 1 (Lowest Pot):</strong> Wins with the lowest token count.</li>
          <li><strong>Pot 2 (Even Pot):</strong> Wins with an even token count.</li>
          <li><strong>Pot 3 (Median Pot):</strong> Wins by holding the median token count.</li>
          <li><strong>Pot 4 (Odd Pot):</strong> Wins with an odd token count.</li>
          <li><strong>Pot 5 (Highest Pot):</strong> Wins with the highest token count.</li>
        </ul>

        <h3>Token Allocation and Reallocation</h3>
        <p>Players can allocate tokens to any empty pot without a fee. Reallocating tokens incurs a
          {{ gameConfig.fee_reallocation }}% fee, and can only be done to pots that currently have no tokens from the
          player.</p>

        <h3>Dynamic Bid Constraints and Reallocation Limits</h3>
        <p>Minimum and maximum bet limits are dynamically set based on the average token count across pots, promoting
          strategic engagement and fairness. These constraints apply only to allocations.</p>

        <h3>Winning Pot Determination</h3>
        <p>The winning pot is determined by its specific rules, and players in this pot receive their proportional
          share
          of the total tokens, post the deduction of a {{ gameConfig.fee }}% winning fee, along with redistributed
          tokens from the less successful pots.</p>

        <p>Explore detailed gameplay examples and strategic insights at the <a
          href="https://github.com/magiodev/prudent-pots" target="_blank">GitHub repository</a>.</p>
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
  border-left: 1px solid $pp-color-4;
  border-right: 1px solid $pp-color-4;
}
</style>