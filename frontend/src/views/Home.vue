<template>
  <div class="container-fluid">
    <div class="row bg-accent">
      <div class="col pb-5">
        <TimerComponent class="mb-3"/>
        <ConfigComponent/>

        <PotsComponent/>
      </div>
    </div>

    <div class="row">
      <div class="col py-5">
        <BidComponent v-if="timeLeftSeconds"/>
        <EndComponent v-else/>
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
import EndComponent from "@/components/Game/EndComponent.vue";
import mxGame from "@/mixin/game";

export default {
  name: 'HomeView',

  components: {EndComponent, ConfigComponent, TimerComponent, PotsComponent, BidComponent},

  mixins: [mxGame],

  computed: {
    ...mapGetters(['gameConfig', 'gameState'])
  }
};
</script>
