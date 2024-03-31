<template>
  <div class="pots-component">
    <h3 class="text-center">Pots</h3>
    <div v-if="pots.length">
      <div class="container-fluid">
        <div class="row offset-md-1 gap-3">
          <PotItemComponent :pot="pot" v-for="pot in pots" :key="pot.id"/>
        </div>
      </div>
    </div>
    <LoadingComponent v-else/>
  </div>
</template>

<script>
import { mapGetters } from "vuex";
import PotItemComponent from "@/components/Game/PotItemComponent.vue";
import LoadingComponent from "@/components/Common/LoadingComponent.vue";

export default {
  name: "PotsComponent",
  components: {LoadingComponent, PotItemComponent},
  computed: {
    ...mapGetters(['pots'])
  }
};
</script>
