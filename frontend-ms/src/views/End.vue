<template>
  <div class="end container">
    <div class="row py-5">
      <div class="offset-md-4 col-md-4 text-center">
        <EndComponent/>
      </div>
    </div>
  </div>
</template>

<script>
import EndComponent from "@/components/Game/EndComponent.vue";

export default {
  name: 'EndView',
  components: {EndComponent}
};
</script>

<style lang="scss" scoped>
@import "@/assets/style.scss";

.end {
  border-left: 1px solid $pp-color-5;
  border-right: 1px solid $pp-color-5;
}
</style>