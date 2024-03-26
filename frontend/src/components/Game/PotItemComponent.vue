<template>
  <div class="pot-item-component col-md-2 text-center text-black" :class="isPotWinning ? 'bg-success' : 'bg-danger'">
    <!-- TODO: Highlight the pot red or green based on if its currently winning or not. -->
    <div class="pot-header">
      <h5>{{ getPotName(pot.pot_id) }}</h5>
    </div>

    <div class="pot-content">
      <p>{{ getPotDescription(pot.pot_id) }}</p>
      <!-- TODO cut decimals to 6 only if more-->
      <span class="pot-tokens p-1">{{ Number(pot.amount / 1000000) }} $OSMO</span>
    </div>

    <div class="pot-footer mt-3">
      <button @click="onPotClick(pot.pot_id)">Select</button>
    </div>

    <hr/>

    <div class="allocations card">
      <h6>Your allocation:</h6>
      <span>{{ allocations }} $OSMO</span>
    </div>
  </div>
</template>

<script>
import {mapGetters, mapMutations} from "vuex";
import mxPot from "@/mixin/pot";

export default {
  name: "PotItemComponent",

  mixins: [mxPot],

  props: {
    pot: {
      type: Object,
      required: true
    }
  },

  computed: {
    ...mapGetters(['winningPots', 'userAllocations']),

    isPotWinning() {
      return !!this.winningPots.find(pot => pot === Number(this.pot.pot_id))
    },

    allocations() {
      return this.userAllocations.find(a => a.pot_id === Number(this.pot.pot_id))?.amount || 0
    }
  },

  methods: {
    ...mapMutations(['setSelectedPot']),

    // TODO: Implement this.electedPot Vuex store item
    onPotClick(potId) {
      // Do something with userSigner
      this.setSelectedPot(potId)
    }
  }
};
</script>

<style scoped>
.pot-item-component {
  border: 1px solid #ddd;
  padding: 1rem;
  margin-bottom: 1rem;
  border-radius: 0.5rem;
  background-color: #f9f9f9;
}

.pot-tokens {
  font-size: 1.2rem;
  font-weight: bold;
  margin-bottom: 1rem;
  border-radius: 0.5rem;
  background-color: #2743b2;
}

.pot-footer > button {
  background-color: #4caf50;
  padding: 0.5rem 1rem;
  border: none;
  border-radius: 0.3rem;
  cursor: pointer;
  transition: background-color 0.3s;
}

.pot-footer > button:hover {
  background-color: #45a049;
}
</style>
