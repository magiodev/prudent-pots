<template>
  <div class="bid-component container">
    <div class="row">
      <div class="offset-md-4 col-md-4 text-center">
        <h3>Place Your Bids</h3>
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
          <div class="fee-calculation mb-3">
            <p>Allocation Fee: {{ calculateAllocationFee(bidAmount) }} $OSMO</p>
          </div>
          <button type="submit" class="btn btn-primary">Submit Bid</button>
        </form>
      </div>
    </div>
  </div>
</template>

<script>
import {mapActions, mapGetters} from "vuex";
import mxChain from "@/mixin/chain";
import mxToast from "@/mixin/toast";

export default {
  name: "BidComponent",

  mixins: [mxChain, mxToast],

  computed: {
    ...mapGetters(['minBid', 'maxBid', 'gameConfig'])
  },

  data() {
    return {
      bidAmount: 0,
      selectedPot: 2 // TODO: Create Vuex item
    };
  },

  created() {
    this.bidAmount = 0 // TODO: Set it as minBid
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
      try {
        await this.allocateTokens(this.selectedPot, this.bidAmount)
        this.toast.success("Tx successful")
        // Fetch new game information after ending the previous match
        // TODO: Create wrapper as fetchGameInit()
        await this.fetchGameState()
        await this.fetchPots()
        await this.fetchBidRange()
        await this.fetchWinningPots()
        await this.fetchReallocationFeePool()
      } catch (e) {
        this.toast.success("Tx error")
      }
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
