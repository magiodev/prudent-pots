<template>
  <button class="button-component" :class="isSmall ? 'small' : ''" :disabled="isDisabled">
    {{ text }}
    <LoadingComponent v-if="isBusy" class="d-inline"/>
  </button>
</template>

<script>
import LoadingComponent from "@/components/Common/LoadingComponent.vue";

export default {
  name: "ButtonComponent",
  components: {LoadingComponent},
  props: {
    text: {
      type: String,
      required: true,
    },
    isDisabled: {
      type: Boolean,
      default: false,
    },
    isBusy: {
      type: Boolean,
      default: false,
    },
    isSmall: {
      type: Boolean,
      default: false,
    }
  },
}
</script>

<style lang="scss" scoped>
@import '@/assets/style';

.button-component {
  outline: none;
  vertical-align: middle;
  font-weight: 600;
  color: white;
  text-transform: uppercase;
  padding: .5em 1.5em;
  background: $pp-purple-2; // Now the button is green
  border: 0;
  font-size: 2em;

  &.small {
    font-size: 1.25em;
  }

  &:hover {
    color: $pp-green-1;
  }

  &:active {
    background: $pp-green-1;
    color: $pp-purple-1;
  }

  &:disabled {
    cursor: not-allowed;
    color: $button-disabled-text;
    background: $button-disabled-bg;
  }
}
</style>
