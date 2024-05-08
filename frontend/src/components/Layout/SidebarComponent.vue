<template>
  <div :class="['sidebar', { expanded: isExpanded }]" class="z-3">
    <button @click="toggleSidebar" class="toggle-btn">
      <b-arrow-left v-if="isExpanded"></b-arrow-left>
      <b-arrow-right v-else></b-arrow-right>
    </button>

    <div class="content p-3 d-flex flex-column h-100">
      <GameActivityComponent/>
    </div>
  </div>
</template>

<script>
import {mapGetters} from "vuex";
import GameActivityComponent from "@/components/Common/GameActivityComponent.vue";

export default {
  name: 'SidebarComponent',

  components: {GameActivityComponent},

  computed: {
    ...mapGetters(['allPlayersAllocations'])
  },

  data() {
    return {
      isExpanded: false
    }
  },

  methods: {
    toggleSidebar() {
      this.isExpanded = !this.isExpanded;
    }
  }
}
</script>

<style lang="scss" scoped>
@import "@/assets/style.scss";

.sidebar {
  position: fixed;
  top: 0;
  right: 0;
  width: 300px;
  @media (min-width: 768px) {
    width: 320px;
  }
  @media (min-width: 992px) {
    width: 420px;
  }
  @media (min-width: 1200px) {
    width: 568px;
  }
  height: 100%;
  background-color: $pp-color-5;
  border-left: 1px solid $pp-color-3;
  transition: transform 0.3s ease;
  transform: translateX(100%);
}

.sidebar.expanded {
  transform: translateX(0);
}

.toggle-btn {
  position: absolute;
  top: 50%;
  left: -3em;
  width: 0;
  height: 0;
  border-style: solid;
  border-width: 25px 25px 25px 0;
  border-color: transparent $pp-color-3 transparent transparent;
  background-color: transparent;
  cursor: pointer;
  transform: translateY(-50%);
}
</style>
