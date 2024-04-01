<template>
  <div class="row">
    <div class="offset-sm-3 col-sm-6 offset-md-4 col-md-4">
      <div class="timer-card text-center py-5">
        <template v-if="timeLeftSeconds">
          <h4 class="text-pp-purple-2">Game Ends: {{ new Date(gameState.end_time * 1000).toLocaleString() }}</h4>
          <div>
            <img class="skull me-2 d-inline" :src="imageSkull" alt="Skull"/>
            <h2 class="text-pp-purple-2 d-inline">{{ timeLeftHuman }}</h2>
          </div>
        </template>
        <template v-else>
          <h2 class="text-pp-purple-2">Time's up!</h2>
          <h4 class="text-pp-purple-2">Please trigger the end game process.</h4>
        </template>
      </div>
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
    ...mapGetters(['gameState', 'reallocationFeePool'])
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

  .skull {
    width: auto;
    height: 32px;
  }
}
</style>
