<template>
  <div class="row">
    <div class="offset-sm-3 col-sm-6 offset-md-4 col-md-4">
      <div class="card text-center">
        <div class="card-body">
          <h5 class="card-title">Timer</h5>
          <p v-if="timeLeftHuman" class="card-text text-success">Time Remaining: {{ timeLeftHuman }}</p>
          <div v-if="timeLeftHuman" class="progress">
            <div class="progress-bar" role="progressbar" :style="{ width: progressPercentage + '%' }" aria-valuenow="25"
                 aria-valuemin="0" aria-valuemax="100">{{ progressPercentage }}%
            </div>
          </div>
          <p v-else class="card-text text-danger">The game has ended. Please trigger the end game process.</p>

          <div class="mb-3">
            <!-- Button is only shown when timeLeftHuman is falsy, meaning the game has ended -->
            <ButtonComponent v-if="!timeLeftHuman" :isDisabled="isBusy || !userAddress" text="End Game"
                             @click="onEndGame"/>
            <LoadingComponent v-if="isBusy"/>
          </div>

          <p>Game started @ {{ new Date(gameState.start_time * 1000).toLocaleString() }}, and will end @
            {{ new Date(gameState.end_time * 1000).toLocaleString() }}.</p>

          <p>Reallocation fee pool: {{ reallocationFeePool / 1000000 }} $OSMO</p>
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
import mxGame from "@/mixin/game";

export default {
  name: "TimerComponent",
  components: {LoadingComponent, ButtonComponent},

  mixins: [mxChain, mxToast, mxGame],

  computed: {
    ...mapGetters(['gameState', 'reallocationFeePool', 'userAddress']),

    progressPercentage() {
      if (!this.gameState) return 0;
      const totalTime = this.gameState.end_time * 1000 - this.gameState.start_time * 1000;
      const timeElapsed = this.currentTime - this.gameState.start_time * 1000;
      return Math.min(100, (timeElapsed / totalTime) * 100).toFixed(2);
    }
  },

  data() {
    return {
      isBusy: false,
    };
  },

  methods: {
    ...mapActions(['fetchPlayerAllocations']),

    async onEndGame() {
      this.isBusy = true
      try {
        const tx = await this.endGame()
        this.toast.success(`Tx successful. ${tx.transactionHash}`)
        await this.fetchInterval()
        // TODO: check if the following can be wrapped or generalized
        await this.fetchPlayerAllocations()
      } catch (e) {
        this.toast.error(`${e.message}`)
      }
      this.isBusy = false
    }
  }
}
</script>
