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
        <p v-if="gameConfig" class="mb-3">
          Each round lasts {{ gameDuration }}, with stakes in {{ gameConfig.game_denom }}. The minimum bid is set at {{ gameConfig.min_bid }} {{ gameConfig.game_denom }}.
          A {{ gameConfig.fee_reallocation }}% fee is charged for bet reallocation, which is reserved for the next game, and a winning fee of {{ gameConfig.fee }}% is applied to the prize pot.</p>
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