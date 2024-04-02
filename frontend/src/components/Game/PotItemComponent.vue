<template>
  <div class="pot-item-component col-4 col-md-2 text-center text-black mb-md-0 mb-3">
    <div class="pot-header">
      <h2 class="d-inline" :class="isPotWinning ? 'text-success' : 'text-danger'">{{ getPotName(pot.pot_id) }}</h2>
      <PopoverComponent :text="getPotDescription(pot.pot_id)"/>
    </div>

    <div class="pot-item position-relative" @click="onPotClick(pot.pot_id)">
      <img class="pot-highlight-image w-100 position-absolute"
           :class="utils.selectedPot === pot.pot_id ? 'd-block' : ''" :src="imagePotHighlight" alt="Pot Item"/>
      <img class="pot-image w-100 position-relative" :src="imagePot" alt="Pot Item"/>
      <div class="pot-content">
        <span class="pot-tokens py-1 px-2">{{ Number(pot.amount / 1000000) }} <CoinComponent/></span>
      </div>
    </div>

    <div class="allocations card mt-3 p-1" :data-pot-id="pot.pot_id">
      <draggable
        v-model="allocationsList"
        group="allocations"
        @start="onDragStart"
        @end="onDragEnd"
        item-key="key"
        class="draggable-container"
      >
        <template #item="{ element }">
          <div class="draggable-item bg-primary" v-if="Number(element.amount)">
            <div class="draggable-item-text">
              {{ element.amount / 1000000 }}
            <CoinComponent class="d-inline"/>
            </div>
          </div>
        </template>
        <template #footer v-if="!allocationsList.length">
          <span class="text-secondary drag-here">{{ !playerAllocations.length ? 'No bets' : 'Drag here' }}</span>
        </template>
      </draggable>
    </div>
  </div>
</template>


<script>
import {mapGetters, mapMutations} from "vuex";
import mxPot from "@/mixin/pot";
import PopoverComponent from "@/components/Common/PopoverComponent.vue";
import draggable from "vuedraggable";
import imagePot from "@/assets/pot.gif"
import imagePotHighlight from "@/assets/pot-highlight.png"
import CoinComponent from "@/components/Common/CoinComponent.vue";

export default {
  name: "PotItemComponent",
  components: {CoinComponent, PopoverComponent, draggable},
  mixins: [mxPot],

  props: {
    pot: {
      type: Object,
      required: true
    }
  },

  computed: {
    ...mapGetters(['winningPots', 'playerAllocations', 'utils']),

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
      imagePotHighlight
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
.pot-item:hover {
  .pot-highlight-image {
    display: block;
  }
}

.pot-highlight-image {
  //position: absolute;
  left: 0;
  top: 0;
  display: none;
}

.pot-content {
  position: absolute;
  top: 60%;
  left: 50%;
  transform: translate(-50%, -50%);
}

.pot-tokens {
  font-weight: bold;
  font-size: calc(8px + 1vw); /* Adjust 12px and 1vw as needed */

  border-radius: .75em;
  background-color: white;
  white-space: nowrap;
}

.pot-footer > button:hover {
  background-color: #45a049;
}

.draggable-container {
  min-height: 2em;

  .draggable-item {
    min-height: 2em;
    .draggable-item-text {
      white-space: nowrap;
      position: absolute;
      top: 50%;
      left: 50%;
      transform: translate(-50%, -50%);
    }
  }

  .drag-here {
    white-space: nowrap;
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
  }
}
</style>
