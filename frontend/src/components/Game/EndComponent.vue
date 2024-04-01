<template>
  <div class="bid-component">
    <div class="row">
      <div class="offset-sm-3 col-sm-6 offset-md-4 col-md-4 text-center text-white">
        <p>Something here that explains what will happen</p>

        <ButtonComponent
          :isDisabled="isBusy || !userAddress"
          text="End Game"
          @click="onEndGame"/>
        <LoadingComponent v-if="isBusy"/>

        <div class="card mt-5">
          <div class="card-body">
            <h5>Winning Stats</h5>

            <ul class="winning-stats list-unstyled">
              <li>Reallocation fee pool: {{ reallocationFeePool / 1000000 }} <CoinComponent/></li>
              <li>Winning pots: {{winningPots.length ? winningPots : 'N/D'}}</li>
              <li>Total loosing tokens: TODO</li>
            </ul>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import ButtonComponent from "@/components/Common/ButtonComponent.vue";
import LoadingComponent from "@/components/Common/LoadingComponent.vue";
import {mapActions, mapGetters} from "vuex";
import mxChain from "@/mixin/chain";
import mxToast from "@/mixin/toast";
import mxGame from "@/mixin/game";
import CoinComponent from "@/components/Common/CoinComponent.vue";

export default {
  name: "EndComponent",
  components: {CoinComponent, LoadingComponent, ButtonComponent},

  mixins: [mxChain, mxToast, mxGame],

  computed: {
    ...mapGetters(['userAddress', 'winningPots', 'reallocationFeePool'])
  },

  data() {
    return {
      isBusy: false,
    };
  },

  methods: {
    ...mapActions(['fetchPlayerAllocations']),

    async onEndGame() {
      this.isBusy = true
      try {
        const tx = await this.endGame()
        this.toast.success(`Tx successful. ${tx.transactionHash}`)
        await this.fetchInterval()
        await this.fetchPlayerAllocations()
      } catch (e) {
        this.toast.error(`${e.message}`)
      }
      this.isBusy = false
    }
  }
}
</script>

<style lang="scss" scoped>

</style>