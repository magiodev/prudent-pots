<template>
  <button class="btn btn-wallet" @click.prevent="onClickConnect" v-if="!userSigner">
    <img :src="imageWalletIcon" alt="Wallet Icon" class="wallet-icon"/>
    <span>Connect Wallet</span>
  </button>
  <button class="btn btn-wallet" v-else disabled>
    <img :src="imageWalletIcon" alt="Wallet Icon" class="wallet-icon"/>
    <span>{{ userAddress ? userAddress.substring(0, 10) : 'Error' }}...</span>
  </button>
</template>

<script>
import {mapActions, mapGetters} from "vuex";
import imageWalletIcon from "@/assets/wallet-icon.png"

export default {
  name: "WalletComponent",

  computed: {
    ...mapGetters(["userSigner", "userAddress"])
  },

  data() {
    return {
      imageWalletIcon
    }
  },

  methods: {
    ...mapActions(["initUser"]),

    async onClickConnect() {
      await this.initUser()
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

  span {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    color: white;
  }
}

.wallet-icon {
  position: relative;
  height: 100px;
  left: -45px;
  transform: rotate(-15deg);
}
</style>