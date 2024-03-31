<template>
  <div class="container-fluid">
    <div class="row bg-accent">
      <div class="col pb-5">
        <TimerComponent class="mb-3" v-if="gameState"/>

        <PotsComponent/>
      </div>
    </div>

    <div class="row">
      <div class="col py-5">
        <BidComponent class="mb-5"/>

        <ConfigComponent v-if="gameConfig"/>
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

export default {
  name: 'HomeView',

  components: {ConfigComponent, TimerComponent, PotsComponent, BidComponent},

  computed: {
    ...mapGetters(['gameConfig', 'gameState'])
  }
};
</script>
