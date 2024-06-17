<template>
  <div class="timer-component offset-md-1 col-md-10 offset-lg-2 col-lg-8 offset-xl-3 col-xl-6 position-relative">
    <div class="timer-card text-center py-5">
      <h4 v-if="!isCountingDownToStart" class="text-pp-purple-2">Game Ends:
        {{ new Date(gameState.end_time * 1000).toLocaleString() }}</h4>
      <h4 v-else class="text-pp-purple-2">Game Starts Soon</h4>
      <div>
        <img class="skull me-2 d-inline" :src="imageSkull" alt="Skull"/>
        <h2 class="text-pp-purple-2 d-inline">
          <span v-if="isCountingDownToStart">Next round in:<br></span>
          {{ timeLeftHuman }}
        </h2>
        <h4>Extend count: {{ gameState.extend_count }}</h4>
      </div>
    </div>
  </div>
</template>


<script>
import {mapGetters} from "vuex";
import mxGame from "../../../../frontend-common/mixin/game";
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
