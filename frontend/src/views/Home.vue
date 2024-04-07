<template>
  <div class="container-fluid">
    <div class="row first-section">
      <div class="col">
        <TimerComponent class="mb-3 mb-md-5"/>
        <PotsComponent class="mb-md-5"/>
      </div>
    </div>

    <div class="row graphic-items position-relative bg-pp-purple-1">
      <div class="col py-3 py-md-5">
        <BidComponent v-if="timeLeftSeconds"/>
        <ReportComponent v-else/>
      </div>
    </div>

    <div class="row bg-pp-purple-1">
      <div class="col">
        <ConfigComponent/>
      </div>
    </div>
  </div>
</template>

<script>
import PotsComponent from "@/components/Game/PotsComponent.vue";
import BidComponent from "@/components/Game/BidComponent.vue";
import TimerComponent from "@/components/Info/TimerComponent.vue";
import ConfigComponent from "@/components/Info/ConfigComponent.vue";
import {mapGetters} from "vuex";
import mxGame from "@/mixin/game";
import ReportComponent from "@/components/Game/ReportComponent.vue";

export default {
  name: 'HomeView',

  components: {ReportComponent, ConfigComponent, TimerComponent, PotsComponent, BidComponent},

  mixins: [mxGame],

  computed: {
    ...mapGetters(['gameConfig', 'gameState'])
  }
};
</script>

<style lang="scss" scoped>
.first-section {
  @media(min-width: 768px) {
    margin-top: -100px;
  }
}
.graphic-items {
  &:before {
    content: url("@/assets/tree.png");
    position: absolute;
    top: -400px;
    left: -210px;
    transform: scale(.3);
    display: inline-block;
    @media (max-width: 767px) {
      left: -325px;
    }
  }
  &:after {
    content: url("@/assets/tombs.png");
    position: absolute;
    top: -210px;
    right: -200px;
    transform: scale(.3);
    display: inline-block;
    @media (max-width: 767px) {
      right: -325px;
    }
  }
}
</style>