<template>
  <div class="bid-component">
    <div class="row">
      <div class="offset-md-3 col-md-6 text-center">
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
                  required
                />
                <button class="btn btn-outline-secondary" type="button" @click="setMinBid">Min</button>
                <button class="btn btn-outline-secondary" type="button" @click="setAverageBid">Avg</button>
                <button class="btn btn-outline-secondary" type="button" @click="setMaxBid">Max</button>
              </div>

              <button type="submit" class="btn btn-primary mb-2" :disabled="!utils.selectedPot || isBusy">Submit Bid</button>
            </form>

            <div class="fee-calculation">
              <p>Allocation Fee: {{ calculateAllocationFee(bidAmount) }} $OSMO</p>
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

export default {
  name: "BidComponent",

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
    ...mapActions(['fetchGameState', 'fetchPots', 'fetchBidRange', 'fetchWinningPots', 'fetchReallocationFeePool']),

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
      // Ensure gameConfig is available and fee_allocation is a number
      if (this.gameConfig && !isNaN(this.gameConfig.fee_allocation)) {
        return (amount * this.gameConfig.fee_allocation) / 100; // TODO div by 1000000 and Math.ceil
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
        await this.fetchBidRange()
        this.setMinBid()
        await this.fetchWinningPots()
        await this.fetchReallocationFeePool()
      } catch (e) {
        this.toast.success("Tx error")
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
