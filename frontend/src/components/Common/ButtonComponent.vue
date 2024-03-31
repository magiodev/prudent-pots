<template>
  <button class="button-component" :disabled="isDisabled">{{text}}</button>
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
@import '@/assets/style'; // Ensure this path points to your style.scss

.button-component {
  position: relative;
  display: inline-block;
  cursor: pointer;
  outline: none;
  vertical-align: middle;
  text-decoration: none;
  font-size: inherit;
  font-family: inherit;
  transition: transform 150ms cubic-bezier(0, 0, 0.58, 1), background 150ms cubic-bezier(0, 0, 0.58, 1);
  font-weight: 600;
  color: $color-main; // Changed to main color for text
  text-transform: uppercase;
  padding: 1.25em 2em;
  background: $color-button; // Now the button is green
  border: 2px solid $color-accent; // Using accent color for border
  border-radius: 0.75em;
  transform-style: preserve-3d;

  &::before {
    position: absolute;
    content: '';
    width: 100%;
    height: 100%;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: darken($color-button, 10%); // Slightly darker green for effect
    border-radius: inherit;
    box-shadow: 0 0 0 2px $color-accent; // Accent color for shadow
    transform: translate3d(0, 0.75em, -1em);
    transition: transform 150ms cubic-bezier(0, 0, 0.58, 1), box-shadow 150ms cubic-bezier(0, 0, 0.58, 1);
  }

  &:hover {
    background: lighten($color-button, 5%); // Lighter green on hover
    transform: translate(0, 0.25em);

    &::before {
      box-shadow: 0 0 0 2px $color-accent;
      transform: translate3d(0, 0.5em, -1em);
    }
  }

  &:active {
    background: darken($color-button, 5%); // Even darker green on active
    transform: translate(0em, 0.75em);

    &::before {
      box-shadow: 0 0 0 2px $color-accent;
      transform: translate3d(0, 0, -1em);
    }
  }

  &:disabled {
    cursor: not-allowed;
    color: $button-disabled-text;
    background: $button-disabled-bg;
    transform: translate(0em, 0.75em);

    &::before {
      //background: $button-disabled-bg;
      box-shadow: $button-disabled-bg;
      transform: translate3d(0, 0, -1em);
    }
  }
}
</style>
