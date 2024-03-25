<template>
  <div class="container-fluid">
    <div class="row">
      <div class="col">
        <pre>{{gameConfig}}</pre>
        <pre>{{gameState}}</pre>
        <pre>{{winningPots}}</pre>
        <PotsComponent />
        <BidComponent />
      </div>
    </div>
  </div>
</template>

<script>
import PotsComponent from "@/components/Game/PotsComponent.vue";
import BidComponent from "@/components/Game/BidComponent.vue";
import {mapGetters} from "vuex";

export default {
  name: 'HomeView',
  components: { PotsComponent, BidComponent },
  computed: {
    ...mapGetters(['gameConfig', 'gameState', 'winningPots'])
  }
};
</script>
