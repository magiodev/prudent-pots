<template>
  <div class="report-component">
    <div class="row">
      <div class="offset-sm-3 col-sm-6 offset-md-4 col-md-4 text-center text-white">
        <div class="report-card pt-4 pb-5">
          <h5>Winning Stats</h5>

          <ul class="winning-stats list-unstyled">
            <li>Reallocation fee pool: {{ reallocationFeePool / 1000000 }}
              <CoinComponent/>
            </li>
            <li>Winning pots: {{ winningPots.length ? winningPots : 'N/D' }}</li>
            <!-- <li>Total loosing tokens: N</li>-->
          </ul>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import {mapGetters} from "vuex";
import mxChain from "@/mixin/chain";
import mxToast from "@/mixin/toast";
import mxGame from "@/mixin/game";
import CoinComponent from "@/components/Common/CoinComponent.vue";

export default {
  name: "ReportComponent",
  components: {CoinComponent},

  mixins: [mxChain, mxToast, mxGame],

  computed: {
    ...mapGetters(['winningPots', 'reallocationFeePool'])
  }
}
</script>

<style lang="scss" scoped>
.report-card {
  background: url("@/assets/tomb-bg.png");
  background-size: 100% 100%;
  background-repeat: no-repeat;
  background-position: center;
}
</style>