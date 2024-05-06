<template>
  <div class="badge-component d-flex">
    <div class="badge-left">
      <img :src="imageLeft" class="img-fluid"/>
    </div>
    <div class="badge-center d-flex flex-row align-items-center">
      <span class="me-1">{{ displayAmount(amount) }}</span>
      <CoinComponent/>
    </div>
    <div class="badge-right">
      <img :src="imageRight" class="img-fluid"/>
    </div>
  </div>
</template>

<script>
import CoinComponent from "@/components/Common/CoinComponent.vue";
import imageLeft from "@/assets/badge/badge-left.png"
import imageRight from "@/assets/badge/badge-right.png"
import mxChain from "@/mixin/chain";

export default {
  name: "BadgeComponent",

  mixins: [mxChain],

  components: {CoinComponent},

  props: {
    amount: {
      required: true,
    }
  },

  data() {
    return {
      imageLeft,
      imageRight
    }
  }
}
</script>

<style lang="scss" scoped>
@import '@/assets/style';

.badge-component {
  img {
    height: 48px;
  }

  .badge-center {
    background: url("@/assets/badge/badge-center.png");
    background-size: 100% 100%;
  }

}
</style>
