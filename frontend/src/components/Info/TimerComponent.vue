<template>
  <div class="row">
    <div class="offset-sm-3 col-sm-6 offset-md-4 col-md-4">
      <div class="card text-center">
        <div class="card-body">
          <template v-if="timeLeftSeconds">
            <p class="text-success">Time Remaining: {{ timeLeftHuman }}</p>
            <div class="progress">
              <div class="progress-bar" role="progressbar" :style="{ width: progressPercentage + '%' }"
                   aria-valuenow="25"
                   aria-valuemin="0" aria-valuemax="100">{{ progressPercentage }}%
              </div>
            </div>
          </template>
          <template v-else>
            <p class="text-danger">The game has ended. Please trigger the end game process.</p>
          </template>

          <ul class="general-stats list-unstyled">
            <li>Game started @ {{ new Date(gameState.start_time * 1000).toLocaleString() }}</li>
            <li>Game end: {{ new Date(gameState.end_time * 1000).toLocaleString() }}</li>
            <li>Reallocation fee pool: {{ reallocationFeePool / 1000000 }} $OSMO</li>
          </ul>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import {mapGetters} from "vuex";
import mxGame from "@/mixin/game";

export default {
  name: "TimerComponent",

  mixins: [mxGame],

  computed: {
    ...mapGetters(['gameState', 'reallocationFeePool']),

    progressPercentage() {
      if (!this.gameState) return 0;
      const totalTime = this.gameState.end_time * 1000 - this.gameState.start_time * 1000;
      const timeElapsed = this.currentTime - this.gameState.start_time * 1000;
      return Math.min(100, (timeElapsed / totalTime) * 100).toFixed(2);
    }
  }
}
</script>
