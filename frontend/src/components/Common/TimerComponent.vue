<template>
  <div class="timer-component offset-md-1 col-md-10 offset-lg-2 col-lg-8 offset-xl-3 col-xl-6 position-relative mb-5">
    <div class="timer-card text-center py-5">
      <h4 v-if="!isCountingDownToStart" class="text-pp-purple-2">
        Game Ends: {{ new Date(gameState.end_time * 1000).toLocaleString() }}
      </h4>
      <h4 v-else class="text-pp-purple-2">Game Starts Soon</h4>
      <div>
        <img class="skull me-2 d-inline" :src="imageSkull" alt="Skull" />
        <h2 class="text-pp-purple-2 d-inline">
          <span v-if="isCountingDownToStart">Next round in:<br /></span>
          <span v-else-if="timeLeftSeconds > 0">Game ends in:<br /></span>
          {{ timeLeftHuman }}
        </h2>
        <h4>Extend count: {{ gameState.extend_count }}</h4>
      </div>
    </div>
  </div>

  <!-- Add the restart game button -->
  <ButtonComponent
      v-if="timeLeftSeconds <= 0"
      :isDisabled="isBusy || !userAddress"
      :isBusy="isBusy"
      text="Restart Game"
      @click="onRestartGame"
      class="mt-3"
    />
</template>

<script>
import { mapGetters } from "vuex";
import mxGame from "../../../../frontend-common/mixin/game";
import mxChain from "../../../../frontend-common/mixin/chain";
import mxToast from "../../../../frontend-common/mixin/toast";
import imageSkull from "@/assets/skull.png";
import ButtonComponent from "@/components/Common/ButtonComponent.vue";

export default {
  name: "TimerComponent",

  components: {
    ButtonComponent,
  },

  mixins: [mxGame, mxChain, mxToast],

  computed: {
    ...mapGetters(["gameState", "userAddress"]),
  },

  data() {
    return {
      imageSkull,
      isBusy: false,
    };
  },

  methods: {
    async onRestartGame() {
      this.isBusy = true;
      try {
        const tx = await this.endGame();
        this.toast.success(`Game restarted successfully. Tx: ${tx.transactionHash}`);
        await this.fetchInterval(true);
      } catch (e) {
        this.toast.error(`Failed to restart game: ${this.cleanErrorMessage(e.message)}`);
      }
      this.isBusy = false;
    },
  },
};
</script>

<style lang="scss" scoped>
.timer-card {
  background: url("@/assets/timer-bg.png");
  background-size: 100% 100%;
  background-repeat: no-repeat;
  background-position: center;

  h2 {
    font-size: 2.5em;
    vertical-align: bottom;
  }

  h4 {
    font-size: 1.5em;
  }

  .skull {
    width: auto;
    height: 48px;
  }
}
</style>
