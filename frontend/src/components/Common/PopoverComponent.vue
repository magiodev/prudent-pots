<template>
  <button
    ref="popover"
    class="bg-transparent border-0"
    data-bs-toggle="popover"
    data-bs-placement="top"
    :data-bs-content="text"
    data-bs-trigger="hover"
  >
    <b-icon-info-circle></b-icon-info-circle>
  </button>
</template>

<script>
import { Popover } from 'bootstrap';

export default {
  name: "PopoverComponent",

  props: {
    text: {
      type: String,
      required: true,
    }
  },

  mounted() {
    // Use this.$refs to access the DOM element
    const popoverElement = this.$refs.popover;
    if (popoverElement) {
      new Popover(popoverElement);
    }
  },
}
</script>
