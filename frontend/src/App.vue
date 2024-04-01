<template>
  <div id="app-body" class="d-flex flex-column">
    <NavbarComponent/>
    <div class="main-section flex-grow-1">
      <LoadingComponent v-if="isBusy"/>
      <router-view v-else/>
    </div>
    <FooterComponent/>
  </div>
</template>

<script>
import NavbarComponent from "@/components/Layout/NavbarComponent.vue";
import LoadingComponent from "@/components/Common/LoadingComponent.vue";
import FooterComponent from "@/components/Layout/FooterComponent.vue";
import mxGame from "@/mixin/game";
import {mapGetters} from "vuex";

export default {
  name: "App",

  mixins: [mxGame],

  components: {FooterComponent, LoadingComponent, NavbarComponent},

  computed: {
    ...mapGetters(['gameConfig'])
  },

  data() {
    return {
      isBusy: true,
    }
  },

  async created() {
    await this.fetchOnce();
    await this.fetchInterval()
    // we ensure that till this moment rest of UI is kept idle
    this.isBusy = false;

    // Set auto-fetch interval
    this.intervalId = setInterval(() => {
      const isGameEnd = this.timeLeftSeconds < Number(this.gameConfig.game_extend)
      this.fetchInterval(isGameEnd);
      console.log(`Auto-fetch!`)
    }, 5000);
  },

  unmounted() {
    if (this.intervalId) clearInterval(this.intervalId)
  }
};
</script>

<style lang="scss">
@import "vue-toastification/src/scss/_variables";
@import "vue-toastification/src/scss/_toastContainer";
@import "vue-toastification/src/scss/_toast";
@import "vue-toastification/src/scss/_closeButton";
@import "vue-toastification/src/scss/_progressBar";
@import "vue-toastification/src/scss/_icon";
@import "vue-toastification/src/scss/animations/_bounce";

@import "@/assets/style.scss";

@font-face {
  font-family: 'TrashHand';
  src: url('assets/fonts/TrashHand.ttf') format('truetype'); /* Adjust the path based on where you placed the font */
}

body {
  background: $color-background !important;
  color: white;

  * {
    font-family: 'TrashHand', sans-serif;
  }
}
</style>