<template>
  <button class="button-component" :disabled="isDisabled">{{ text }}</button>
</template>

<script>
export default {
  name: "ButtonComponent",
  props: {
    isDisabled: {
      type: Boolean,
      required: true,
    },
    text: {
      type: String,
      required: true,
    },
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
