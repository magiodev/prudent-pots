<template>
  <div class="pots-component position-relative">
    <div class="row justify-content-center">
      <PotItemComponent
          :pot="pot"
          v-for="pot in pots"
          :key="pot.id"
          @endReallocation="onReallocateTokens"
          @dragStateChange="onDragStateChange"
      />

      <div v-if="userAddress" class="mt-3 text-center">
        <p v-if="dragging">
          You are reallocating {{ displayAmount(dragInfo.originalAmount)}} <CoinComponent/> <u>from the {{ getPotName(dragInfo.fromPotId) }} Pot</u><br>
          A fee of {{ gameConfig.fee }}% will be applied, so <u>you will move {{ displayAmount(dragInfo.newAmount) }} <CoinComponent/></u>
        </p>
        <p v-else>
          Allocations can be dragged and dropped between pots<br>
          You have reallocated <u>{{ playerReallocations }} out of {{ gameConfig.reallocations_limit }}</u> times allowed
        </p>
      </div>
    </div>
  </div>
</template>

<script>
import {mapActions, mapGetters} from "vuex";
import PotItemComponent from "@/components/Game/PotItemComponent.vue";
import mxToast from "../../../../frontend-common/mixin/toast";
import mxChain from "../../../../frontend-common/mixin/chain";
import mxGame from "../../../../frontend-common/mixin/game";
import CoinComponent from "@/components/Common/CoinComponent.vue";
import mxPot from "../../../../frontend-common/mixin/pot";

export default {
  name: "PotsComponent",

  mixins: [mxChain, mxToast, mxGame, mxPot],

  components: {CoinComponent, PotItemComponent},

  data() {
    return {
      dragging: false,
      dragInfo: {
        fromPotId: null,
        originalAmount: 0,
        newAmount: 0,
      }
    }
  },

  computed: {
    ...mapGetters(['pots', "playerReallocations", "userAddress", "gameConfig"]),
  },

  methods: {
    ...mapActions(['fetchPlayerData']),

    async onReallocateTokens(newAllocation) {
      // prevent from to same pot id
      if (newAllocation.fromPotId === newAllocation.toPotId) return

      this.isBusy = true
      try {
        const tx = await this.reallocateTokens(newAllocation.fromPotId, newAllocation.toPotId)
        this.toast.success(`Tx successful. ${tx.transactionHash}`)
        // Fetch new game information after ending the previous match
        await this.fetchInterval()
        await this.fetchPlayerData()
      } catch (e) {
        this.toast.error(`${this.cleanErrorMessage(e.message)}`)
      }
      this.isBusy = false
    },

    onDragStateChange(dragState) {
      this.dragging = dragState.dragging;
      this.dragInfo = { ...this.dragInfo, ...dragState };
    }
  }
};
</script>

<style lang="scss" scoped>
@media (max-width: 991px) {
  #pot-id-1 {
    order: 1;
  }
  #pot-id-2 {
    order: 4;
  }
  #pot-id-3 {
    order: 2;
  }
  #pot-id-4 {
    order: 5;
  }
  #pot-id-5 {
    order: 3;
  }
}
</style>