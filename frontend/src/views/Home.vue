<template>

  <div class="container-fluid">
    <div class="row banner">
      <TimerComponent/>
    </div>

    <div class="row raffle">
      <div class="col">
        <RaffleComponent/>
      </div>
    </div>

    <div class="row game" v-if="!isCountingDownToStart">
      <div class="col">
        <StatsComponent/>
        <PotsComponent class="mt-lg-5 mt-3 mb-lg-5"/>
      </div>
    </div>

    <div class="row bet position-relative bg-pp-color-5" v-if="!isCountingDownToStart">
      <div class="col py-3 py-lg-5">
        <BidComponent/>
      </div>
    </div>
  </div>
</template>

<script>
import PotsComponent from "@/components/Game/PotsComponent.vue";
import BidComponent from "@/components/Game/BidComponent.vue";
import TimerComponent from "@/components/Common/TimerComponent.vue";
import {mapGetters} from "vuex";
import mxGame from "../../../frontend-common/mixin/game";
import StatsComponent from "@/components/Common/StatsComponent.vue";
import RaffleComponent from "@/components/Common/RaffleComponent.vue";

export default {
  name: 'HomeView',

  components: {RaffleComponent, StatsComponent, TimerComponent, PotsComponent, BidComponent},

  mixins: [mxGame],

  computed: {
    ...mapGetters(['gameConfig', 'gameState'])
  }
};
</script>

<style lang="scss" scoped>
@import "@/assets/style.scss";

.banner {
  position: relative;
  overflow: hidden; // Ensures the image doesn't show outside the banner
  background-size: auto 100%; // Adjust the width freely, but fit the height
  animation: slide 90s infinite alternate ease-in-out; // Adjust the timing as needed
}

@keyframes slide {
  from {
    background-position: 0% center;
  }
  to {
    background-position: 100% center;
  }
}

.raffle, .game {
  border-bottom: 1px solid $pp-color-4;
}
</style>