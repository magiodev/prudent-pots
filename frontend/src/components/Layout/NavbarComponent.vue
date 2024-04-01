<template>
  <nav class="navbar navbar-light bg-pp-purple-3">
    <div class="container-fluid py-1">
      <a class="navbar-brand">
        <img class="logo" :src="imageLogo" alt="Prudent Pots"/>
      </a>

      <div class="controls">
        <WalletComponent/>
      </div>
    </div>
  </nav>
</template>

<script>
import imageLogo from "@/assets/logo.png"
import WalletComponent from "@/components/Common/WalletComponent.vue";

export default {
  name: "NavbarComponent",
  components: {WalletComponent},

  data() {
    return {
      imageLogo
    }
  }
}
</script>

<style scoped lang="scss">
.logo {
  height: 100px;
}
</style>