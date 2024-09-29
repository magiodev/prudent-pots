<template>
  <div class="timer-component offset-md-1 col-md-10 offset-lg-2 col-lg-8 offset-xl-3 col-xl-6 position-relative">
    <img :src="imageTimer" alt="Timer" class="img-fluid" />

    <div class="timer-card pt-5">
      <h3 class="counter text-pp-color-3 mb-0 mb-md-4">
        <span v-if="isCountingDownToStart">Next round in:<br /></span>
        <span v-else-if="timeLeftSeconds > 0">Game ends in:<br /></span>
        {{ timeLeftHuman }}
      </h3>

      <h5 v-if="timeLeftSeconds > 0" class="text-pp-color-3 text-end">
        Extends: {{ gameState.extend_count.toString() || "N/D" }}
      </h5>
    </div>

    <!-- Add the restart game button -->
  </div>
  <ButtonComponent
    v-if="timeLeftSeconds <= 0"
    :isDisabled="isBusy || !userAddress"
    :isBusy="isBusy"
    text="Restart Game"
    @click="onRestartGame"
  />
</template>

<script>
import { mapGetters } from "vuex";
import mxGame from "../../../../frontend-common/mixin/game";
import mxChain from "../../../../frontend-common/mixin/chain";
import mxToast from "../../../../frontend-common/mixin/toast";
import imageTimer from "@/assets/timer-item.gif";
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
      imageTimer,
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
.timer-component {
  .timer-card {
    position: absolute;
    top: 38%;
    left: 57%;
    transform: translate(-50%, -50%);
    width: 47.5%;

    h3,
    h5 {
      font-family: "Reddit Mono", monospace;
    }

    @media (max-width: 420px) {
      h3 {
        font-size: 1em;
      }
    }

    @media (min-width: 576px) {
      h2 {
        font-size: 2em;
        vertical-align: bottom;
      }
    }

    .skull {
      width: auto;
      height: 48px;
    }

    @media (max-width: 575px) {
      .text-end {
        font-size: 1em;
      }
    }
  }
}
</style>
