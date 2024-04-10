<template>
  <div class="row">
    <div class="offset-sm-3 col-sm-6 offset-lg-4 col-lg-4">
      <div class="timer-card text-center py-5">
        <template v-if="timeLeftSeconds">
          <h4 class="text-pp-purple-2">Game Ends: {{ new Date(gameState.end_time * 1000).toLocaleString() }}</h4>
          <div>
            <img class="skull me-2 d-inline" :src="imageSkull" alt="Skull"/>
            <h2 class="text-pp-purple-2 d-inline">{{ timeLeftHuman }}</h2>
          </div>
        </template>
        <EndComponent v-else />
      </div>
      <p class="text-center text-pp-purple-2">Reallocation fee pool: {{ reallocationFeePool / 1000000 }} <CoinComponent/></p>
    </div>
  </div>
</template>

<script>
import {mapGetters} from "vuex";
import mxGame from "@/mixin/game";
import imageSkull from "@/assets/skull.png"
import EndComponent from "@/components/Game/EndComponent.vue";
import CoinComponent from "@/components/Common/CoinComponent.vue";

export default {
  name: "TimerComponent",
  components: {CoinComponent, EndComponent},

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
