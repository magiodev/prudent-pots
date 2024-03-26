<template>
  <div class="container-fluid">
    <div class="row">
      <div class="col">
        <TimerComponent class="mb-3" v-if="gameState"/>

        <PotsComponent/>
        <BidComponent/>

        <hr/>

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
