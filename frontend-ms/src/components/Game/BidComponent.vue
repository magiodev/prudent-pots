<template>
  <div class="bet-component">
    <div class="row">
      <!-- Bid: Only during game -->
      <div class="col-lg-5 text-center text-white mb-5" v-if="timeLeftSeconds">
        <h2 class="mb-3">Place Your Bet</h2>
        <div class="pp-card position-relative">
          <!-- Hover animation -->
          <div class="mascotte position-absolute d-none d-lg-block pp-card bg-pp-color-3 p-0">
            <img class="img-fluid" alt="Sticker" :src="currentImage"/>
          </div>

          <div class="selected-pot mb-3">
            <p v-if="utils.selectedPot">
              Selected pot: {{ getPotName(utils.selectedPot) }}
            </p>
            <p v-else>Select a pot to place a bet.</p>
          </div>
          <form @submit.prevent="onAllocateTokens" class="bet-form">
            <div class="d-flex justify-content-center mb-3">
              <input
                type="number"
                class="form-control w-50"
                v-model.number="bidAmountDenom"
                :min="displayAmount(minBid)"
                :max="displayAmount(maxBid)"
                step="0.000001"
                :disabled="!utils.selectedPot || isBusy"
                required
              />
            </div>
            <div class="d-flex justify-content-center gap-3 mb-3">
              <ButtonComponent text="Min" @click.prevent="setMinBid" :isSmall="true" :isDisabled="!userAddress"/>
              <ButtonComponent text="Avg" @click.prevent="setAverageBid" :isSmall="true" :isDisabled="!userAddress"/>
              <ButtonComponent text="Max" @click.prevent="setMaxBid" :isSmall="true" :isDisabled="!userAddress"/>
            </div>
            <ButtonComponent v-if="userAddress" :isDisabled="!utils.selectedPot || isBusy" :isBusy="isBusy"
                             text="Place Bid"
                             class="mb-2"/>
            <div v-else>
              <p>Connect your wallet to make a bet.</p>
              <WalletComponent/>
            </div>
            <!-- User Balance -->
            <div v-if="userAddress" class="small">
              Balance: {{ userBalance || '...' }}
              <CoinComponent/>
            </div>
            <div v-if="userAddress && cw721Contract" class="small">
              <span v-if="userCw721Balance.length">You hodl {{ userCw721Balance.length }} MS so you're eligible for a discount of {{ displayAmount(maxBid / 2 - minBid)
                }} <CoinComponent/> on the min bet amount.</span>
              <span v-else>You own {{ userCw721Balance.length }} MS so you're not eligible for a discount on the min. bet amount.</span>
            </div>
          </form>
        </div>
      </div>

      <!-- Stats Always Bottom -->
      <div class="col-lg-7 text-white" :class="!timeLeftSeconds ? 'offset-lg-1 col-lg-10' : ''">
        <h2 class="text-center">Round #{{ gameState.round_count }}</h2>

        <p v-if="!timeLeftSeconds" class="text-center mb-3 text-pp-color-4">
          The round has finished, and the game will restart soon.
          <br/>When this occurs, the prizes will be distributed.
        </p>

        <div class="pp-card">
          <PlayersAllocations/>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import {mapActions, mapGetters} from "vuex";
import mxChain from "../../../../frontend-common/mixin/chain";
import mxToast from "../../../../frontend-common/mixin/toast";
import mxPot from "../../../../frontend-common/mixin/pot";
import ButtonComponent from "@/components/Common/ButtonComponent.vue";
import mxGame from "../../../../frontend-common/mixin/game";
import CoinComponent from "@/components/Common/CoinComponent.vue";
import imageWait from "@/assets/stickers/wait.gif";
import imageCook from "@/assets/stickers/cook.gif";
import imageBurn from "@/assets/stickers/burn.gif";
import imageFrightened from "@/assets/stickers/frightened.gif";
import PlayersAllocations from "@/components/Game/PlayersAllocations.vue";
import WalletComponent from "@/components/Common/WalletComponent.vue";

export default {
  name: "BidComponent",

  components: {WalletComponent, PlayersAllocations, CoinComponent, ButtonComponent},

  mixins: [mxChain, mxToast, mxPot, mxGame],

  computed: {
    ...mapGetters(['minBid', 'maxBid', 'gameConfig', 'utils', 'userAddress', 'userBalance', "userCw721Balance"]),

    bidAmountDenom: {
      get() {
        return this.displayAmount(this.bidAmount); // Convert from udenom to DENOM for display
      },
      set(value) {
        this.bidAmount = Math.ceil(value * 1000000); // Convert back to udenom for internal usage
      }
    },
  },
  data() {
    return {
      isBusy: false,
      bidAmount: 0,
      cw721Contract: process.env.VUE_APP_CONTRACT_CW721,
      currentImage: null,
      imageWait,
      imageCook,
      imageBurn,
      imageFrightened
    };
  },
  created() {
    this.currentImage = this.imageWait; // Set default image
    this.bidAmountDenom = this.displayAmount(this.minBid); // Set the initial bet amount in DENOM
  },
  methods: {
    ...mapActions(['fetchPlayerData']),
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
      this.isBusy = true;
      this.currentImage = this.imageCook; // Revert image
      try {
        const tx = await this.allocateTokens(this.utils.selectedPot, this.bidAmount);
        this.toast.success(`Tx successful. ${tx.transactionHash}`);
        this.currentImage = this.imageBurn; // Set 'Burn' image
        setTimeout(() => {
          this.currentImage = this.imageWait; // Revert image
        }, 3000);
        await this.fetchInterval(); // Fetch game interval
        this.setMinBid() // Automatically set the new minBid amount after a tx success
        await this.fetchPlayerData(); // Fetch player data
      } catch (e) {
        this.toast.error(`${this.cleanErrorMessage(e.message)}`);
        this.currentImage = this.imageFrightened; // Set 'Frightened' image
        setTimeout(() => {
          this.currentImage = this.imageWait; // Revert image
        }, 3000);
      }
      this.isBusy = false;
    }
  }
};
</script>

<style lang="scss" scoped>
.mascotte {
  left: 0;
  @media(max-width: 1199px) {
    left: -2em;
  }
  top: 50%;
  transform: translate(-50%, -50%);

  img {
    width: 120px;
  }
}

.bet-form {
  input.form-control {
    border-radius: 0;
    text-align: center;
    height: 48px;
  }
}
</style>
