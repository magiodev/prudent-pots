<template>
  <div class="row">
    <div class="offset-md-4 col-md-4">
      <div class="card text-center">
        <div class="card-body">
          <h5 class="card-title">Timer</h5>
          <p v-if="timeLeft" class="card-text text-success">Time Remaining: {{ timeLeft }}</p>
          <div v-if="timeLeft" class="progress">
            <div class="progress-bar" role="progressbar" :style="{ width: progressPercentage + '%' }" aria-valuenow="25" aria-valuemin="0" aria-valuemax="100">{{ progressPercentage }}%</div>
          </div>
          <p v-else class="card-text text-danger">The game has ended. Please trigger the end game process.</p>

          <div class="mb-3">
            <!-- Button is only shown when timeLeft is falsy, meaning the game has ended -->
            <ButtonComponent v-if="!timeLeft" :isDisabled="isBusy" text="End Game" @click="onEndGame"/>
            <LoadingComponent v-if="isBusy"/>
          </div>

          <p>Game started @ {{ new Date(gameState.start_time * 1000).toLocaleString() }}, and will end @
            {{ new Date(gameState.end_time * 1000).toLocaleString() }}.</p>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import {mapActions, mapGetters} from "vuex";
import mxChain from "@/mixin/chain";
import mxToast from "@/mixin/toast";
import ButtonComponent from "@/components/Common/ButtonComponent.vue";
import LoadingComponent from "@/components/Common/LoadingComponent.vue";

export default {
  name: "TimerComponent",
  components: {LoadingComponent, ButtonComponent},

  mixins: [mxChain, mxToast],

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
    },

    progressPercentage() {
      if (!this.gameState) return 0;
      const totalTime = this.gameState.end_time * 1000 - this.gameState.start_time * 1000;
      const timeElapsed = this.currentTime - this.gameState.start_time * 1000;
      return Math.min(100, (timeElapsed / totalTime) * 100).toFixed(2);
    }
  },

  data() {
    return {
      intervalId: null,
      currentTime: new Date().getTime(),
      isBusy: false,
    };
  },

  mounted() {
    this.intervalId = setInterval(this.updateCurrentTime, 1000);
  },

  unmounted() {
    clearInterval(this.intervalId);
  },

  methods: {
    ...mapActions(['fetchGameState', 'fetchPots', 'fetchBidRange', 'fetchWinningPots', 'fetchReallocationFeePool', 'fetchUserAllocations']),

    updateCurrentTime() {
      this.currentTime = new Date().getTime();
    },

    async onEndGame() {
      this.isBusy = true
      try {
        await this.endGame()
        this.toast.success("Tx successful")
        await this.fetchGameState()
        await this.fetchPots()
        await this.fetchReallocationFeePool()

        await this.fetchUserAllocations()
        await this.fetchBidRange()
        await this.fetchWinningPots()
      } catch (e) {
        this.toast.error(`${e.message}`)
      }
      this.isBusy = false
    }
  }
}
</script>
