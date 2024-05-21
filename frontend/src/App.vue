<template>
  <div id="app-body" class="d-flex flex-column">
    <LoadingComponent v-if="isBusy"/>
    <template v-else>
      <NavbarComponent/>
      <div class="main-section flex-grow-1">
        <router-view/>
      </div>
      <SidebarComponent/>
      <FooterComponent/>
    </template>
  </div>
</template>

<script>
import NavbarComponent from "@/components/Layout/NavbarComponent.vue";
import LoadingComponent from "@/components/Common/LoadingComponent.vue";
import SidebarComponent from "@/components/Layout/SidebarComponent.vue";
import FooterComponent from "@/components/Layout/FooterComponent.vue";
import mxGame from "../../frontend-common/mixin/game";
import {mapGetters} from "vuex";
import mxChain from "../../frontend-common/mixin/chain";

export default {
  name: "App",

  mixins: [mxGame, mxChain],

  components: {SidebarComponent, FooterComponent, LoadingComponent, NavbarComponent},

  computed: {
    ...mapGetters(['gameConfig'])
  },

  data() {
    return {
      isBusy: true,
      intervalTimeout: Number(process.env.VUE_APP_INTERVAL_TIMEOUT)
    }
  },

  async created() {
    await this.suggestChain()
    await this.fetchOnce();
    await this.fetchInterval()
    // we ensure that till this moment rest of UI is kept idle
    this.isBusy = false;

    // Set auto-fetch interval
    this.intervalId = setInterval(() => {
      const isGameEnd = this.timeLeftSeconds < Number(this.gameConfig.game_extend)
      this.fetchInterval(isGameEnd);
    }, this.intervalTimeout);
  },

  unmounted() {
    if (this.intervalId) clearInterval(this.intervalId)
  }
};
</script>

<style lang="scss">
@import "@/assets/style.scss";
</style>