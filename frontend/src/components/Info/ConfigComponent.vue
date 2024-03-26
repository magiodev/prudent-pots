<template>
  <div class="row">
    <div class="offset-md-3 col-md-6">
      <div class="accordion" id="gameInfoAccordion" v-if="gameConfig">
        <div class="accordion-item">
          <h2 class="accordion-header" id="gameConfigHeading">
            <button class="accordion-button" type="button" data-bs-toggle="collapse" data-bs-target="#gameConfig"
                    aria-expanded="true" aria-controls="gameConfig">
              Game Configuration
            </button>
          </h2>
          <div id="gameConfig" class="accordion-collapse collapse show" aria-labelledby="gameConfigHeading"
               data-bs-parent="#gameInfoAccordion">
            <div class="accordion-body">
              <ul>
                <li>Fee Allocation: {{ gameConfig.fee_allocation }}%</li>
                <li>Fee Reallocation: {{ gameConfig.fee_reallocation }}%</li>
                <li>Allocation Address: {{ gameConfig.fee_allocation_address }}</li>
                <li>Game Duration: {{ gameDuration }}</li>
                <li>Game Denomination: {{ gameConfig.game_denom }}</li>
                <li>Minimum Bid: {{ gameConfig.min_bid }}</li>
              </ul>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import {mapGetters} from "vuex";

export default {
  name: "ConfigComponent",

  computed: {
    ...mapGetters(['gameConfig']),

    gameDuration() {
      const minutes = Math.floor(this.gameConfig.game_duration / 60);
      const seconds = this.gameConfig.game_duration % 60;
      return `${minutes} minutes and ${seconds} seconds`;
    }
  }
}
</script>