<template>
  <div class="timer-component offset-md-1 col-md-10 offset-lg-2 col-lg-8 offset-xl-3 col-xl-6 position-relative">
    <img :src="imageTimer" alt="Timer" class="img-fluid"/>

    <div class="timer-card pt-5">
      <h2 class="counter text-pp-color-3 mb-0 mb-md-4">{{ timeLeftSeconds ? timeLeftHuman : "Time's up!" }}</h2>

      <h5 class="text-pp-color-3 text-end">Extends: {{ gameState.extend_count.toString() || 'N/D' }}</h5>
    </div>
  </div>
</template>

<script>
import {mapGetters} from "vuex";
import mxGame from "../../../../frontend-common/mixin/game";
import imageTimer from "@/assets/timer-item.gif"

export default {
  name: "TimerComponent",

  mixins: [mxGame],

  computed: {
    ...mapGetters(['gameState'])
  },

  data() {
    return {
      imageTimer,
    }
  }
}
</script>

<style lang="scss" scoped>
.timer-component {

  .timer-card {
    position: absolute;
    top: 38%;
    left: 57%;
    transform: translate(-50%, -50%);
    width: 47.5%;

    h2 {
      font-family: "Reddit Mono", monospace;
    }

    @media(min-width: 576px) {
      h2 {
        font-size: 2em;
        vertical-align: bottom;
      }
    }

    .skull {
      width: auto;
      height: 48px;
    }

    @media(max-width: 575px) {
      .text-end {
        font-size: 1em;
      }
    }
  }
}
</style>
