<template>
  <div class="pots-component">
    <h3 class="text-center">Pots</h3>

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
    ...mapActions(['fetchUserAllocations']),

    async onReallocateTokens(newAllocation) {
      // Handle the reallocation logic here
      // This might involve calling a Vuex action or directly an API method
      console.log('Reallocation happened', newAllocation);

      this.isBusy = true
      try {
        await this.reallocateTokens(newAllocation.fromPotId, newAllocation.toPotId) // TODO
        this.toast.success("Tx successful")
        // Fetch new game information after ending the previous match
        await this.fetchInterval()
        await this.fetchUserAllocations()
      } catch (e) {
        this.toast.error(`${e.message}`)
      }
      this.isBusy = false
    }
  }
};
</script>
