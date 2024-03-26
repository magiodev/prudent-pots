<template>
  <div class="row">
    <div class="offset-md-4 col-md-4">
      <div class="card text-center">
        <div class="card-body">
          <h5 class="card-title">Timer</h5>
          <p v-if="timeLeft" class="card-text text-success">Time Remaining: {{ timeLeft }}</p>
          <p v-else class="card-text text-danger">The game has ended. Please trigger the end game process.</p>

          <!-- Button is only shown when timeLeft is falsy, meaning the game has ended -->
          <button v-if="!timeLeft" class="btn btn-primary mb-3" @click="endGame">End Game</button>

          <p>Game started @ {{ new Date(gameState.start_time * 1000).toLocaleString() }}, and will end @
            {{ new Date(gameState.end_time * 1000).toLocaleString() }}.</p>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import {mapGetters} from "vuex";
import mxChain from "@/mixin/chain";

export default {
  name: "TimerComponent",

  mixins: [mxChain],

  computed: {
    ...mapGetters(['gameState']),

    timeLeft() {
      if (!this.gameState) return null
      const endTime = this.gameState.end_time * 1000;
      const timeDiff = endTime - this.currentTime;
      if (timeDiff <= 0) {
        return null;
      }
      const hours = Math.floor((timeDiff / (1000 * 60 * 60)) % 24);
      const minutes = Math.floor((timeDiff / (1000 * 60)) % 60);
      const seconds = Math.floor((timeDiff / 1000) % 60);
      return `${hours}h ${minutes}m ${seconds}s`;
    }
  },

  data() {
    return {
      intervalId: null,
      currentTime: new Date().getTime(),
    };
  },

  // TODO: Implement fetchGameState inside this interval so we update it each second only when 15 seconds are left.

  mounted() {
    this.intervalId = setInterval(this.updateCurrentTime, 1000);
  },

  unmounted() {
    clearInterval(this.intervalId);
  },

  methods: {
    updateCurrentTime() {
      this.currentTime = new Date().getTime();
    }
  }
}
</script>