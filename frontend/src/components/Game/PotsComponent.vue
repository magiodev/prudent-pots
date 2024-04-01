<template>
  <div class="pots-component position-relative z-1">
    <div class="row justify-content-center" v-if="pots.length">
      <PotItemComponent :pot="pot" v-for="pot in pots" :key="pot.id" @endReallocation="onReallocateTokens"/>
    </div>
    <LoadingComponent v-else/>
  </div>
</template>

<script>
import {mapActions, mapGetters} from "vuex";
import PotItemComponent from "@/components/Game/PotItemComponent.vue";
import LoadingComponent from "@/components/Common/LoadingComponent.vue";
import mxToast from "@/mixin/toast";
import mxChain from "@/mixin/chain";
import mxGame from "@/mixin/game";

export default {
  name: "PotsComponent",

  mixins: [mxChain, mxToast, mxGame],

  components: {LoadingComponent, PotItemComponent},

  computed: {
    ...mapGetters(['pots']),
  },

  methods: {
    ...mapActions(['fetchPlayerAllocations']),

    async onReallocateTokens(newAllocation) {
      // prevent from to same pot id
      if (newAllocation.fromPotId === newAllocation.toPotId) return

      this.isBusy = true
      try {
        const tx = await this.reallocateTokens(newAllocation.fromPotId, newAllocation.toPotId)
        this.toast.success(`Tx successful. ${tx.transactionHash}`)
        // Fetch new game information after ending the previous match
        await this.fetchInterval()
        await this.fetchPlayerAllocations()
      } catch (e) {
        this.toast.error(`${e.message}`)
      }
      this.isBusy = false
    }
  }
};
</script>
