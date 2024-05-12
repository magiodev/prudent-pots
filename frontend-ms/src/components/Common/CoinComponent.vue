<template>
  <img :src="imageCoin" class="coin-component" alt="Coin"/>
</template>

<script>
import imageCoin from "@/assets/icons/icon-osmo.svg"

export default {
  name: "CoinComponent",

  data() {
    return {
      imageCoin
    }
  }
}
</script>

<style scoped lang="scss">
.coin-component {
  width: 1.25em;
  height: auto;
  vertical-align: sub;
}
</style>