<template>
  <!-- Button trigger modal -->
  <div class="readme-component d-flex justify-content-end">
    <ButtonComponent text="README" data-bs-toggle="modal" data-bs-target="#exampleModal" :is-small="true"/>
  </div>

  <!-- Modal -->
  <div class="readme-modal modal fade" id="exampleModal" tabindex="-1" aria-labelledby="exampleModalLabel" aria-hidden="true">
    <div class="modal-dialog">
      <div class="modal-body p-5" v-if="gameConfig">
        <p>
          Each round lasts {{ gameDuration }}, with stakes in {{ gameConfig.game_denom }}. The minimum bid is set at
          {{ gameConfig.min_bid }} {{ gameConfig.game_denom }}.
          A {{ gameConfig.fee_reallocation }}% fee is charged for bet reallocation, which is reserved for the next
          game, and a winning fee of {{ gameConfig.fee }}% is applied to the prize pot.</p>
      </div>
      <LoadingComponent v-else/>
    </div>
  </div>
</template>

<script>
import {mapGetters} from "vuex";
import LoadingComponent from "@/App.vue";
import ButtonComponent from "@/components/Common/ButtonComponent.vue";

export default {
  name: "ReadmeComponent",
  components: {ButtonComponent, LoadingComponent},

  computed: {
    ...mapGetters(['gameConfig']),

    gameDuration() {
      const minutes = Math.floor(this.gameConfig.game_duration / 60);
      const seconds = this.gameConfig.game_duration % 60;
      return `${minutes} minutes and ${seconds} seconds`;
    }
  }
}
</script>

<style lang="scss" scoped>
.readme-modal {
  .modal-body {
    background: url("@/assets/tomb-bg.png");
    background-size: 100% 100%;
    background-repeat: no-repeat;
    background-position: center;
  }
}
</style>