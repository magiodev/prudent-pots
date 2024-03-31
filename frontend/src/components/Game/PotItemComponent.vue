<template>
  <div class="pot-item-component col-4 col-md-2 text-center text-black">
    <!-- TODO: Highlight the pot red or green based on if its currently winning or not. -->
    <div class="pot-header">
      <h5 class="d-inline" :class="isPotWinning ? 'text-success' : 'text-danger'">{{ getPotName(pot.pot_id) }}</h5>

      <PopoverComponent :text="getPotDescription(pot.pot_id)"/>
    </div>

    <div class="pot-item position-relative" @click="onPotClick(pot.pot_id)">
      <img class="pot-image w-100 position-relative" :Src="imagePot"/>

      <div class="pot-content">
        <!-- TODO cut decimals to 6 only if more-->
        <span class="pot-tokens p-1">{{ Number(pot.amount / 1000000) }} $OSMO</span>
      </div>
    </div>

    <div class="allocations card mt-3 p-1">
      <h6>Your bet:</h6>
      <span class="card" :class="allocations ? 'bg-primary' : 'bg-secondary'">{{ allocations / 1000000 }} $OSMO</span>
    </div>
  </div>
</template>

<script>
import {mapGetters, mapMutations} from "vuex";
import mxPot from "@/mixin/pot";
import PopoverComponent from "@/components/Common/PopoverComponent.vue";
import imagePot from "@/assets/pot.png"
import imagePotInfo from "@/assets/pot-info.png"

export default {
  name: "PotItemComponent",
  components: {PopoverComponent},

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

  data() {
    return {
      imagePot,
      imagePotInfo
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
.pot-header {
  padding: 1rem;
  margin-bottom: 1rem;
  border-radius: 0.5rem;

  position: relative;
  background: url('@/assets/wallet-bg.png') no-repeat center center;
  background-size: contain;
  border: 0;
  outline: none;

  div {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    color: white;
  }
}

.pot-content {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
}

.pot-tokens {
  font-size: 1.2rem;
  font-weight: bold;
  margin-bottom: 1rem;
  border-radius: 0.5rem;
  background-color: #2743b2;
  white-space: nowrap;
  color: white;
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
