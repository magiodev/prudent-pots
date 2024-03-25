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
import {mapActions} from "vuex";
import NavbarComponent from "@/components/Layout/NavbarComponent.vue";
import LoadingComponent from "@/components/Layout/LoadingComponent.vue";
import FooterComponent from "@/components/Layout/FooterComponent.vue";

export default {
  name: "App",
  components: {FooterComponent, LoadingComponent, NavbarComponent},
  data() {
    return {
      isBusy: true,
    };
  },
  async created() {
    // User
    await this.initUser();
    this.isBusy = false;

    // Game
    await this.fetchGameConfig()
    await this.fetchGameState()
    await this.fetchPots()
    await this.fetchWinningPots()
    await this.fetchBidRange()
    await this.fetchReallocationFeePool()
  },
  methods: {
    ...mapActions(['initUser', 'fetchGameConfig', 'fetchGameState', 'fetchPots', 'fetchWinningPots', 'fetchBidRange', 'fetchReallocationFeePool'])
  }
};
</script>

<style lang="scss">
#app {
  background-color: #001B79;
  color: white;
}
</style>