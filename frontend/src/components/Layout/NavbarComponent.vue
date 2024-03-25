<template>
  <nav class="navbar navbar-light">
    <div class="container-fluid py-1">
      <a class="navbar-brand">
        <img class="logo" src="favicon.ico" alt="Prudent Pots"/>
        Prudent Pots
      </a>

      <div class="controls d-flex">
        <button class="btn btn-prudent" @click.prevent="onClickConnect" v-if="!userSigner">
          <b-icon-wallet></b-icon-wallet>
          Connect Wallet
        </button>
        <button class="btn btn-prudent" v-else disabled>
          <b-icon-wallet></b-icon-wallet>
          {{ userAddress ? userAddress.substring(0, 10) : 'Error' }}...
        </button>
      </div>
    </div>
  </nav>
</template>

<script>
import {mapActions, mapGetters} from "vuex";
import mxToast from "@/mixin/toast";

export default {
  name: "NavbarComponent",

  mixins: [mxToast],

  computed: {
    ...mapGetters(["userSigner", "userAddress"])
  },

  methods: {
    ...mapActions(["initUser"]),

    async onClickConnect() {
      await this.initUser()
    }
  }
}
</script>

<style scoped lang="scss">
.logo {
  height: 32px;
}
</style>