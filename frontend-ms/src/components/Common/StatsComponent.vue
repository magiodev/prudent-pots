<template>
  <div class="stats-component row text-center">
    <div class="col stats-item px-0 small">
      <div class="total-tokens py-3">
        Total: {{ totalTokens }}
        <CoinComponent/>
      </div>
    </div>

    <div class="col stats-item px-0 small">
      <div class="winning-tokens py-3">
        Winning: {{ winningTokens }}
        <CoinComponent/>
      </div>
    </div>
    <div class="col stats-item px-0 small">
      <div class="losing-tokens py-3">
        Losing: {{ losingTokens }}
        <CoinComponent/>
      </div>
    </div>

    <div class="col stats-item px-0 small">
      <div class="reallocation-fee-pool py-3">
        R.F. Pool: {{ displayAmount(reallocationFeePool) }}
        <CoinComponent/>
      </div>
    </div>
  </div>
</template>

<script>
import {mapGetters} from "vuex";
import CoinComponent from "@/components/Common/CoinComponent.vue";
import mxChain from "@/mixin/chain";

export default {
  name: "StatsComponent",

  components: {CoinComponent},

  mixins: [mxChain],

  computed: {
    ...mapGetters(['pots', 'winningPots', 'reallocationFeePool']),

    totalTokens() {
      // Sums up the amount from all pots, converting string to number and adjusting for decimals
      return this.pots.reduce((total, pot) => (Number(total) + Number(pot.amount) / 1000000).toFixed(6), 0);
    },

    winningTokens() {
      // Sums up the amount from pots that are in the winningPots array, converting string to number and adjusting for decimals
      return this.pots
        .filter(pot => this.winningPots.includes(pot.pot_id))
        .reduce((total, pot) => (Number(total) + Number(pot.amount) / 1000000).toFixed(6), 0);
    },

    losingTokens() {
      // Sums up the amount from pots that are not in the winningPots array, converting string to number and adjusting for decimals
      return this.pots
        .filter(pot => !this.winningPots.includes(pot.pot_id))
        .reduce((total, pot) => (Number(total) + Number(pot.amount) / 1000000).toFixed(6), 0);
    }
  }
}
</script>

<style lang="scss" scoped>
@import '@/assets/style';

.stats-component {
  .stats-item {
    border-bottom: 1px solid $pp-color-5;
    border-right: 1px solid $pp-color-5;

    &:last-child {
      border-right: none;
    }

    width: auto;
    white-space: nowrap;
  }
}
</style>