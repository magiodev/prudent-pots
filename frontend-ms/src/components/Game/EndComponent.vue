<template>
  <div class="end-component py-3">
    <h2>Time's up!</h2>
    <h4>Please trigger the end game process.</h4>

    <div class="end-inputs mb-3">
      <!-- Input for denomAmount -->
      <label for="denomAmount" class="form-label">Enter Denomination Amount:</label>
      <input id="denomAmount" type="number" class="form-control" v-model="denomAmount">

      <!-- Input for raffleCw721TokenIds as a comma-separated string -->
      <label for="tokenContract" class="form-label">Token Contract:</label>
      <input id="tokenContract" class="form-control" v-model="tokenContract" placeholder="e.g., addy1xyz">

      <!-- Input for raffleCw721TokenIds as a comma-separated string -->
      <label for="tokenId" class="form-label">Token ID:</label>
      <input id="tokenId" class="form-control" v-model="tokenId" placeholder="e.g., 1234">

      <!-- Input for Next Game Start as a future unit timestamp -->
      <label for="nextGameStart" class="form-label">Next Game Start:</label>
      <input id="nextGameStart" class="form-control" v-model="nextGameStart" placeholder="e.g., unit timestamp">
    </div>

    <ButtonComponent
      :isDisabled="isBusy || !userAddress"
      :isBusy="isBusy"
      text="End Game"
      @click="onEndGame"/>
  </div>
</template>

<script>
import ButtonComponent from "@/components/Common/ButtonComponent.vue";
import {mapActions, mapGetters} from "vuex";
import mxChain from "../../../../frontend-common/mixin/chain";
import mxToast from "../../../../frontend-common/mixin/toast";
import mxGame from "../../../../frontend-common/mixin/game";

export default {
  name: "EndComponent",
  components: {ButtonComponent},

  mixins: [mxChain, mxToast, mxGame],

  computed: {
    ...mapGetters(['userAddress', 'winningPots', 'reallocationFeePool'])
  },

  data() {
    return {
      isBusy: false,
      tokenContract: null,
      tokenId: null,
      denomAmount: 0
    };
  },

  methods: {
    ...mapActions(['fetchPlayerData']),

    async onEndGame() {
      this.isBusy = true
      try {
        const tx = await this.endGame(this.tokenContract, this.tokenId, this.denomAmount)
        this.toast.success(`Tx successful. ${tx.transactionHash}`)
        await this.fetchInterval()
        await this.fetchPlayerData()
      } catch (e) {
        this.toast.error(`${this.cleanErrorMessage(e.message)}`)
      }
      this.isBusy = false
    }
  }
}
</script>