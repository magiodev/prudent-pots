<template>
  <span class="badge" :class="badgeClasses">
    APR : {{ getDataAPRs[contract] ? Number(getDataAPRs[contract]).toFixed(6) : '...' }}
    <span class="spinner-border spinner-border-sm ms-1" role="status" v-if="getLoadingAPRs"></span>
  </span>
</template>

<script>
import {mapGetters} from "vuex";

export default {
  name: "APRComponent",

  props: {
    badgeClasses: {
      type: String,
      required: true
    },
    contract: {
      type: Number,
      required: true
    }
  },

  computed: {
    ...mapGetters(['getDataAPRs', 'getLoadingAPRs'])
  }
}
</script>