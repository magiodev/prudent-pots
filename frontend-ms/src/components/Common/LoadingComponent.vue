<template>
  <div class="text-center">
    <img v-if="!isSpinner" class="img-fluid" :src="imageLoading" alt="Loading..."/>
    <div v-else class="spinner-border" role="status"></div>
  </div>
</template>

<script>
import imageLoading from "@/assets/stickers/loading.gif"

export default {
  name: "LoadingComponent",

  props: {
    isSpinner: {
      type: Boolean,
      required: false,
      default: false
    }
  },

  data() {
    return {
      imageLoading
    }
  }
}
</script>