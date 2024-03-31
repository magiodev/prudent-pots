<template>
  <div class="bid-component">
    <div class="row">
      <div class="offset-sm-3 col-sm-6 offset-md-4 col-md-4 text-center">
        <div class="card">
          <div class="card-body">
            <h3>Place Your Bid</h3>

            <div class="selected-pot">
              <p v-if="utils.selectedPot">Selected pot: {{getPotName(utils.selectedPot)}}</p>
              <p v-else>Select a pot to place a bid.</p>
            </div>

            <form @submit.prevent="onAllocateTokens">
              <div class="input-group mb-3">
                <!-- TODO show this divided by 1000000 but leave value as original -->
                <input
                  type="number"
                  class="form-control"
                  v-model.number="bidAmount"
                  :min="minBid"
                  :max="maxBid"
                  placeholder="Token Amount"
                  :disabled="!utils.selectedPot || isBusy"
                  required
                />
                <button class="btn btn-outline-secondary" type="button" @click="setMinBid">Min</button>
                <button class="btn btn-outline-secondary" type="button" @click="setAverageBid">Avg</button>
                <button class="btn btn-outline-secondary" type="button" @click="setMaxBid">Max</button>
              </div>

              <ButtonComponent :isDisabled="!utils.selectedPot || isBusy" text="Place Bid"/>
              <LoadingComponent v-if="isBusy"/>
            </form>

            <div class="fee-calculation mt-3">
              <p class="mb-0">Allocation Fee: {{ calculateAllocationFee(bidAmount) / 1000000 }} $OSMO</p>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import {mapActions, mapGetters} from "vuex";
import mxChain from "@/mixin/chain";
import mxToast from "@/mixin/toast";
import mxPot from "@/mixin/pot";
import ButtonComponent from "@/components/Common/ButtonComponent.vue";
import LoadingComponent from "@/components/Common/LoadingComponent.vue";

export default {
  name: "BidComponent",
  components: {LoadingComponent, ButtonComponent},

  mixins: [mxChain, mxToast, mxPot],

  computed: {
    ...mapGetters(['minBid', 'maxBid', 'gameConfig', 'utils'])
  },

  data() {
    return {
      isBusy: false,
      bidAmount: 0
    };
  },

  created() {
    this.bidAmount = Number(this.minBid)
  },

  methods: {
    ...mapActions(['fetchGameState', 'fetchPots', 'fetchBidRange', 'fetchWinningPots', 'fetchReallocationFeePool', 'fetchUserAllocations']),

    setMinBid() {
      this.bidAmount = this.minBid;
    },

    setAverageBid() {
      this.bidAmount = (this.minBid + this.maxBid) / 2;
    },

    setMaxBid() {
      this.bidAmount = this.maxBid;
    },

    calculateAllocationFee(amount) {
      // Ensure gameConfig is available and fee is a number
      if (this.gameConfig && !isNaN(this.gameConfig.fee)) {
        return (amount * this.gameConfig.fee) / 100; // TODO div by 1000000 and Math.ceil
      }
      return 0;
    },

    async onAllocateTokens() {
      this.isBusy = true
      try {
        await this.allocateTokens(this.utils.selectedPot, this.bidAmount)
        this.toast.success("Tx successful")
        // Fetch new game information after ending the previous match
        // TODO: Create wrapper as fetchGameInit()
        await this.fetchGameState()
        await this.fetchPots()
        await this.fetchUserAllocations()
        await this.fetchBidRange()
        this.setMinBid()
        await this.fetchWinningPots()
        await this.fetchReallocationFeePool()
      } catch (e) {
        this.toast.error(`${e.message}`)
      }
      this.isBusy = false
    }
  }
};
</script>

<style scoped>
.bid-component {
  margin-top: 2rem;
}

.input-group {
  margin-bottom: 1rem;
}

.fee-calculation {
  font-size: 0.9rem;
}
</style>
