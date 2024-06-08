<template>
  <div class="pot-item-component col-4 col-lg-2 text-center text-black mb-lg-0 mb-3" :id="'pot-id-'+pot.pot_id">
    <div class="pot-header"
         ref="popover"
         data-bs-toggle="popover"
         data-bs-placement="top"
         :data-bs-content="getPotDescription(pot.pot_id)"
         data-bs-trigger="hover"
    >
      <h4 class="d-inline me-0 me-md-2" :class="isPotWinning ? 'text-pp-winner' : 'text-pp-loser'">
        {{ getPotName(pot.pot_id) }}
      </h4>
      <PotItemIconComponent :isWinning="isPotWinning" :potId="pot.pot_id" class="d-none d-md-inline-block"/>
    </div>

    <div class="pot-item position-relative mb-3" @click="onPotClick(pot.pot_id)">
      <img class="pot-highlight-image w-100 position-absolute"
           :class="utils.selectedPot === pot.pot_id ? 'd-block highlight' : ''" :src="imagePotHighlight"
           alt="Pot Item"/>
      <img class="pot-image w-100 position-relative" :src="imagePot" alt="Pot Item"/>
    </div>

    <div class="pot-content mb-3">
      <span class="pot-tokens py-2 px-3">{{ displayAmount(pot.amount) }} <CoinComponent/></span>
    </div>

    <div class="allocations card p-1" :data-pot-id="pot.pot_id">
      <draggable
          v-model="allPotsAllocations"
          group="allocations"
          @start="onDragStart"
          @end="onDragEnd"
          item-key="key"
          class="draggable-container"
      >
        <template #item="{ element }">
          <div :class="['draggable-item bg-primary', drag && 'dragged']" v-if="Number(element.amount)">
            <div class="draggable-item-text">
              {{
                !drag
                    ? displayAmount(element.amount)
                    : displayAmount(element.amount * (1 - gameConfig.fee_reallocation / 100))
              }}
              <CoinComponent class="d-inline"/>
            </div>
          </div>
        </template>
        <template #footer v-if="!allPotsAllocations.length">
          <span class="text-secondary drag-here">{{ !playerAllocations.length ? 'No bets' : 'Drag here' }}</span>
        </template>
      </draggable>
    </div>
  </div>
</template>


<script>
import {mapGetters, mapMutations} from "vuex";
import mxPot from "../../../../frontend-common/mixin/pot";
import draggable from "vuedraggable";
import imagePot from "@/assets/pot.gif"
import imagePotHighlight from "@/assets/pot-highlight.png"
import CoinComponent from "@/components/Common/CoinComponent.vue";
import PotItemIconComponent from "@/components/Game/PotItemIconComponent.vue";
import {Popover} from "bootstrap";
import mxChain from "../../../../frontend-common/mixin/chain";

export default {
  name: "PotItemComponent",

  components: {PotItemIconComponent, CoinComponent, draggable},

  mixins: [mxPot, mxChain],

  props: {
    pot: {
      type: Object,
      required: true
    }
  },

  computed: {
    ...mapGetters(['winningPots', 'playerAllocations', 'utils', 'gameConfig']),

    isPotWinning() {
      return this.winningPots.includes(this.pot.pot_id);
    },

    allocations() {
      return this.playerAllocations.find(a => a.pot_id === this.pot.pot_id)?.amount || 0;
    },

    allPotsAllocations() {
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

  mounted() {
    // Use this.$refs to access the DOM element
    const popoverElement = this.$refs.popover;
    if (popoverElement) {
      new Popover(popoverElement);
    }
  },

  methods: {
    ...mapMutations(['setSelectedPot']),

    onPotClick(potId) {
      this.setSelectedPot(potId);
    },

    onDragStart() {
      this.drag = true
      const fromPotId = this.pot.pot_id;

      const originalAmount = this.allPotsAllocations[0]?.amount || 0;
      const newAmount = originalAmount * (1 - this.gameConfig.fee_reallocation / 100);
      this.$emit('dragStateChange', {dragging: true, fromPotId, originalAmount, newAmount});
    },

    onDragEnd(event) {
      this.drag = false

      const fromPotId = this.pot.pot_id;

      // Retrieve the pot_id from the new container after dragging ends
      const toPotElement = event.to.closest('.allocations');
      const toPotId = toPotElement ? Number(toPotElement.dataset.potId) : null;
      if (!toPotId) throw new Error("Something went wrong.")

      const originalAmount = this.allPotsAllocations[0]?.amount || 0;
      const newAmount = originalAmount * (1 - this.gameConfig.fee_reallocation / 100);
      this.$emit('dragStateChange', {dragging: false, fromPotId, originalAmount, newAmount}); // just for ui feedback to user
      this.$emit('endReallocation', {fromPotId, toPotId}); // to cast tx
    },
  },
};
</script>

<style lang="scss" scoped>
@import "@/assets/style";

.pot-highlight-image.highlight {
  opacity: 1;
  transform: scale(1.15);
}

.pot-item:hover {
  .pot-highlight-image:not(.highlight) {
    opacity: 1;
    transform: scale(1.1);
  }
}

.pot-highlight-image {
  //position: absolute;
  left: 0;
  top: 0;
  opacity: 0;
  transition: .15s;
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
