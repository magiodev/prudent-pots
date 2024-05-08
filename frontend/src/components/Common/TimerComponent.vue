<template>
  <div class="timer-component offset-md-1 col-md-10 offset-lg-2 col-lg-8 offset-xl-3 col-xl-6 position-relative">
    <div class="timer-card text-center py-5">
      <template v-if="timeLeftSeconds">
        <h4 class="text-pp-purple-2">Game Ends: {{ new Date(gameState.end_time * 1000).toLocaleString() }}</h4>
        <div>
          <img class="skull me-2 d-inline" :src="imageSkull" alt="Skull"/>
          <h2 class="text-pp-purple-2 d-inline">{{ timeLeftHuman }}</h2>
        </div>
      </template>
      <h2 class="text-pp-color-4">Time's up!</h2>
    </div>
  </div>
</template>

<script>
import {mapGetters} from "vuex";
import mxGame from "@/mixin/game";
import imageSkull from "@/assets/skull.png"

export default {
  name: "TimerComponent",

  mixins: [mxGame],

  computed: {
    ...mapGetters(['gameState'])
  },

  data() {
    return {
      imageSkull
    }
  }
}
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
