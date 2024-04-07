<template>
  <div class="pots-component position-relative z-1">
    <div class="row justify-content-center" v-if="pots.length">
      <PotItemComponent
        :pot="pot"
        v-for="pot in pots"
        :key="pot.id"
        @endReallocation="onReallocateTokens"
        @pot-clicked="onPotClick"
      />
    </div>
    <LoadingComponent v-else/>
  </div>
</template>

<script>
import {mapActions, mapGetters, mapMutations} from "vuex";
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

  mounted() {
    this.$nextTick(() => {
      document.addEventListener('click', this.handleClickOutside);
    });
  },

  unmounted() {
    // Remove the event listener when the component is destroyed
    document.removeEventListener('click', this.handleClickOutside);
  },

  methods: {
    ...mapActions(['fetchPlayerAllocations']),
    ...mapMutations(['setSelectedPot']),

    onPotClick(potId) {
      this.setSelectedPot(potId);
    },

    handleClickOutside(event) {
      // Check each pot item to see if the click was outside all of them
      for (const pot of this.pots) {
        const potItemElement = this.$refs[`potItem-${pot.pot_id}`];
        if (potItemElement && potItemElement.$el.contains(event.target)) {
          return; // Click was inside, no need to clear the selection
        }
      }
      // Click was outside all pot items, clear the selection
      this.setSelectedPot(null);
    },

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
