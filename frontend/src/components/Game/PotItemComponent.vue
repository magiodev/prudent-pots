<template>
  <div class="pot-item-component col-4 col-md-2 text-center text-black">
    <div class="pot-header">
      <h5 class="d-inline" :class="isPotWinning ? 'text-success' : 'text-danger'">{{ getPotName(pot.pot_id) }}</h5>
      <PopoverComponent :text="getPotDescription(pot.pot_id)"/>
    </div>

    <div class="pot-item position-relative" @click="onPotClick(pot.pot_id)">
      <img class="pot-image w-100 position-relative" :src="imagePot" alt="Pot Item"/>
      <div class="pot-content">
        <span class="pot-tokens p-1">{{ Number(pot.amount / 1000000) }} $OSMO</span>
      </div>
    </div>

    <div class="allocations card mt-3 p-1" :data-pot-id="pot.pot_id">
      <draggable v-model="allocationsList" group="allocations" @start="onDragStart" @end="onDragEnd" item-key="key">
        <template #item="{ element }">
          <span class="card bg-primary" v-if="Number(element.amount)">
            {{ element.amount / 1000000 }} $OSMO
          </span>
        </template>
      </draggable>
      <p v-if="!allocationsList.length">Drag here</p>
    </div>
  </div>
</template>


<script>
import {mapGetters, mapMutations} from "vuex";
import mxPot from "@/mixin/pot";
import PopoverComponent from "@/components/Common/PopoverComponent.vue";
import draggable from "vuedraggable";
import imagePot from "@/assets/pot.png"

export default {
  name: "PotItemComponent",
  components: {PopoverComponent, draggable},
  mixins: [mxPot],

  props: {
    pot: {
      type: Object,
      required: true
    }
  },

  computed: {
    ...mapGetters(['winningPots', 'playerAllocations']),

    isPotWinning() {
      return this.winningPots.includes(this.pot.pot_id);
    },

    allocations() {
      return this.playerAllocations.find(a => a.pot_id === this.pot.pot_id)?.amount || 0;
    },

    allocationsList() {
      // Including only the allocation for this specific pot
      const allocationForThisPot = this.playerAllocations.find(a => a.pot_id === this.pot.pot_id);
      return allocationForThisPot
        ? [{
          key: `allocation-${this.pot.pot_id}`,
          amount: allocationForThisPot.amount,
        }]
        : [];
    }

  },

  data() {
    return {
      drag: false,
      imagePot,
    }
  },

  methods: {
    ...mapMutations(['setSelectedPot']),

    onPotClick(potId) {
      this.setSelectedPot(potId);
    },

    onDragStart() {
      this.drag = true
    },

    onDragEnd(event) {
      const fromPotId = this.pot.pot_id;

      // Retrieve the pot_id from the new container after dragging ends
      const toPotElement = event.to.closest('.allocations');
      const toPotId = toPotElement ? Number(toPotElement.dataset.potId) : null;
      if (!toPotId) throw new Error("Something went wrong.")
      this.drag = false

      this.$emit('endReallocation', {fromPotId, toPotId});
    },
  },
};
</script>

<style lang="scss" scoped>
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
