<template>
  <div id="app" class="d-flex flex-column">
    <NavbarComponent/>
    <div class="main-section flex-grow-1">
      <LoadingComponent v-if="isBusy"/>
      <router-view v-else/>
    </div>
    <FooterComponent/>
  </div>
</template>

<script>
import {mapActions, mapGetters} from "vuex";
import NavbarComponent from "@/components/Layout/NavbarComponent.vue";
import LoadingComponent from "@/components/Layout/LoadingComponent.vue";
import FooterComponent from "@/components/Layout/FooterComponent.vue";

export default {
  name: "App",

  components: {FooterComponent, LoadingComponent, NavbarComponent},

  computed: {
    ...mapGetters(['userAddress'])
  },

  data() {
    return {
      isBusy: true,
    };
  },

  async created() {
    // User
    await this.initUser();
    if (this.userAddress) await this.fetchUserAllocations()
    this.isBusy = false;

    // Game
    await this.fetchGameConfig()
    await this.fetchGameState()
    await this.fetchPots()
    await this.fetchWinningPots()
    await this.fetchBidRange()
    await this.fetchReallocationFeePool()
  },

  // TODO: We should be add intervals to fetch gameState, pots, winningPots and reallocationFeePool. Basically everything less gameConfig and initUser. This should be on a short interval to allow UI updates. lets set a different time foreach one so i can fine tune it later

  methods: {
    ...mapActions(['initUser', 'fetchGameConfig', 'fetchGameState', 'fetchPots', 'fetchWinningPots', 'fetchBidRange', 'fetchReallocationFeePool', 'fetchUserAllocations'])
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

#app {
  background-color: #001B79;
  color: white;
}
</style>