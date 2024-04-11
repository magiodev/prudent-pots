<template>
  <div class="end-component py-3">
    <h2 class="text-pp-purple-2">Time's up!</h2>
    <h4 class="text-pp-purple-2">Please trigger the end game process.</h4>

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
import mxChain from "@/mixin/chain";
import mxToast from "@/mixin/toast";
import mxGame from "@/mixin/game";

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
    };
  },

  methods: {
    ...mapActions(['fetchPlayerData']),

    async onEndGame() {
      this.isBusy = true
      try {
        const tx = await this.endGame()
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

<style lang="scss" scoped>

</style>