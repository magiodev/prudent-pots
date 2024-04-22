<template>
  <button class="btn btn-wallet" @click.prevent="onClickConnect" v-if="!userSigner">
    <img :src="imageWalletIcon" alt="Wallet Icon" class="wallet-icon"/>
    <span>Connect Wallet</span>
  </button>
  <button class="btn btn-wallet" v-else>
    <img :src="imageWalletIcon" alt="Wallet Icon" class="wallet-icon"/>
    <span>{{ userAddress ? userAddress.substring(0, 10) : 'Error' }}...</span>
  </button>
</template>

<script>
import {mapGetters} from "vuex";
import imageWalletIcon from "@/assets/wallet-icon.png"
import mxGame from "@/mixin/game";

export default {
  name: "WalletComponent",

  mixins: [mxGame],

  computed: {
    ...mapGetters(["userSigner", "userAddress"])
  },

  data() {
    return {
      imageWalletIcon
    }
  },

  methods: {
    async onClickConnect() {
      await this.fetchOnce()
    }
  }
}
</script>

<style lang="scss" scoped>
.btn-wallet {
  position: relative;
  background: url('@/assets/wallet-bg.png') no-repeat center center;
  background-size: contain;
  border: 0;
  outline: none;
  padding-left: 4em;

  .wallet-icon {
    position: relative;
    height: 65px;
    left: -5.5em;
    transform: rotate(-25deg);
  }

  span {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    color: white;
  }
}
</style>