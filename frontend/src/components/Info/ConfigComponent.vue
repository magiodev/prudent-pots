<template>
  <div class="row">
    <div class="offset-sm-3 col-sm-6 offset-md-4 col-md-4 text-center">
      <p>
        <a class="btn btn-link" data-bs-toggle="collapse" href="#collapseConfig" role="button" aria-expanded="false"
           aria-controls="collapseConfig">
          Game configuration
        </a>
      </p>
      <div class="collapse text-white" id="collapseConfig">
        <p v-if="gameConfig">In this game, a winning fee of {{ gameConfig.fee }}% is applied to the prize pot,
          collected at {{ gameConfig.fee_address }}. Each round lasts {{ gameDuration }}, with stakes in
          {{ gameConfig.game_denom }}. A {{ gameConfig.fee_reallocation }}% fee is charged for bets reallocation,
          with a minimum bid set at {{ gameConfig.min_bid }} {{ gameConfig.game_denom }}.</p>

        <p>Contract address: {{ contractAddress }}</p>
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
  },

  data() {
    return {
      contractAddress: process.env.VUE_APP_CONTRACT
    }
  }
}
</script>