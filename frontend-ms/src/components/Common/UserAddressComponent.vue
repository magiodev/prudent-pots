<template>
  <a :href="`${baseUrl}/accounts/${address}`" target="_blank" class="badge text-decoration-none" :class="isUserAddress ? 'bg-pp-color-5 text-black' : 'bg-dark'">{{ formattedAddress }}</a>
</template>

<script>
import {mapGetters} from "vuex";

export default {
  name: "UserAddressComponent",

  props: {
    address: {
      type: String,
      required: true,
    },
    cut: {
      type: Number,
      required: true
    }
  },

  data() {
    return {
      baseUrl: process.env.VUE_APP_EXPLORER_BASE_URL
    }
  },

  computed: {
    ...mapGetters(['userAddress']),

    isUserAddress() {
      return this.userAddress === this.address;
    },

    formattedAddress() {
      // Remove 'osmo1' prefix and cut the address
      return this.address.replace(/^osmo1/, '').substring(0, this.cut);
    }
  }
}
</script>
