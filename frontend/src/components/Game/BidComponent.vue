<template>
  <div class="bid-component">
    <div class="row">
      <div class="offset-sm-3 col-sm-6 offset-md-4 col-md-4 text-center text-white">
        <div class="bid-header">
          <h2>Place Your Bid</h2>
        </div>

        <div class="selected-pot my-3">
          <p v-if="utils.selectedPot">
            Selected pot: {{ getPotName(utils.selectedPot) }}
          </p>
          <p v-else>Select a pot to place a bid.</p>
        </div>

        <form @submit.prevent="onAllocateTokens" class="bid-form">
          <div class="d-flex justify-content-center">
            <input
              type="number"
              class="form-control mb-3 w-50"
              v-model.number="bidAmountOSMO"
              :min="minBid / 1000000"
              :max="maxBid / 1000000"
              step="0.000001"
              :disabled="!utils.selectedPot || isBusy"
              required
            />
          </div>

          <div class="d-flex justify-content-center gap-3 mb-3">
            <ButtonComponent text="Min" @click.prevent="setMinBid" :isSmall="true"/>
            <ButtonComponent text="Avg" @click.prevent="setAverageBid" :isSmall="true"/>
            <ButtonComponent text="Max" @click.prevent="setMaxBid" :isSmall="true"/>
          </div>

          <ButtonComponent :isDisabled="!utils.selectedPot || isBusy || !userAddress" :isBusy="isBusy" text="Place Bid"/>
        </form>
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
import mxGame from "@/mixin/game";

export default {
  name: "BidComponent",
  components: {ButtonComponent},

  mixins: [mxChain, mxToast, mxPot, mxGame],

  computed: {
    ...mapGetters(['minBid', 'maxBid', 'gameConfig', 'utils', 'userAddress']),

    bidAmountOSMO: {
      get() {
        return this.bidAmount / 1000000; // Convert from uOSMO to OSMO for display
      },
      set(value) {
        this.bidAmount = value * 1000000; // Convert back to uOSMO for internal usage
      }
    },
  },

  data() {
    return {
      isBusy: false,
      bidAmount: 0
    };
  },

  created() {
    this.bidAmountOSMO = this.minBid / 1000000; // Set the initial bid amount in OSMO
  },

  methods: {
    ...mapActions(['fetchPlayerAllocations']),

    setMinBid() {
      this.bidAmount = this.minBid;
    },

    setAverageBid() {
      this.bidAmount = (this.minBid + this.maxBid) / 2;
    },

    setMaxBid() {
      this.bidAmount = this.maxBid;
    },

    async onAllocateTokens() {
      this.isBusy = true
      try {
        const tx = await this.allocateTokens(this.utils.selectedPot, this.bidAmount)
        this.toast.success(`Tx successful. ${tx.transactionHash}`)
        // Fetch new game information after ending the previous match
        await this.fetchInterval()
        this.setMinBid()
        await this.fetchPlayerAllocations()
      } catch (e) {
        this.toast.error(`${e.message}`)
      }
      this.isBusy = false
    }
  }
};
</script>

<style lang="scss" scoped>
.bid-header {
  position: relative;
  background: url('@/assets/wallet-bg.png') no-repeat center center;
  background-size: contain;
  border: 0;
  outline: none;
  padding: 20px 0 10px;

  div {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    color: white;
  }
}
.bid-form {
  input.form-control {
    border-radius: 0;
    text-align: center;
    height: 48px;
  }
}
</style>
