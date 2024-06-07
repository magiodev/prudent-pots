<template>
  <div class="raffle-component row text-center" v-if="raffle?.nft?.id || Number(raffle.denom_amount)">
    <div class="title col-12 pt-3">
      <h3 class="mb-0">Raffle Prizes!</h3>
    </div>

    <div class="col-4 offset-2 col-md-3 offset-md-3 raffle-item py-2">
      <div class="pp-card p-2 h-100">
        <h5 v-if="raffle?.nft?.id" class="text-pp-color-4">MS #{{ raffle.nft.id }}</h5>
        <img v-if="raffle?.nft?.imageUrl" class="nft-image mb-0" :src="raffle.nft.imageUrl" alt=""/>
      </div>
    </div>

    <div class="col-4 col-md-3 raffle-item py-2">
      <div class="pp-card p-2 h-100">
        <h5 v-if="raffle.denom_amount" class="text-pp-color-4">$OSMO</h5>
        <span class="text-pp-color-4">{{ displayAmount(raffle.denom_amount, 2) }}</span> <CoinComponent/>
      </div>
    </div>
  </div>
</template>

<script>
import {mapGetters} from "vuex";
import mxChain from "../../../../frontend-common/mixin/chain";
import CoinComponent from "@/components/Common/CoinComponent.vue";

export default {
  name: "RaffleComponent",
  components: {CoinComponent},

  mixins: [mxChain],

  computed: {
    ...mapGetters(['raffle'])
  }
}
</script>

<style lang="scss" scoped>
@import "@/assets/style";
.raffle-component {

  .raffle-item {
    .nft-image {
      border-radius: 1em;
      border: 1px solid $pp-color-4;
      width: 25%;
      min-width: 72px;
    }
  }
}
</style>