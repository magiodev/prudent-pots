<template>
  <div class="footer container bg-pp-color-5">
    <footer class="row align-items-center py-3 text-white">
      <div class="col-md-4 text-md-start text-center mb-2 mb-md-0">
        © {{ new Date().getFullYear() }} {{title}}
      </div>

      <div class="col-md-4 text-center d-none d-md-block">
        <img class="logo" :src="`${skin}/favicon.png`" :alt="title">
      </div>

      <div class="col-md-4">
        <ul class="nav list-unstyled d-flex justify-content-center justify-content-md-end">
          <li class="ms-3">
            <a href="https://github.com/magiodev/prudent-pots-ms" target="_blank" rel="noopener noreferrer">
              <b-icon-github class="text-white"></b-icon-github>
            </a>
          </li>
          <li class="ms-3">
            <a href="https://twitter.com/magiodev/status/1781957200446922947" target="_blank" rel="noopener noreferrer">
              <b-icon-twitter class="text-white"></b-icon-twitter>
            </a>
          </li>
        </ul>
      </div>
    </footer>
  </div>
</template>

<script>
export default {
  name: "FooterComponent",

  data() {
    return {
      skin: process.env.VUE_APP_SKIN,
      title: process.env.VUE_APP_TITLE
    }
  }
}
</script>

<style scoped lang="scss">
@import "@/assets/style.scss";

.footer {
  border: 1px solid $pp-color-5;

  .logo {
    height: 32px;
  }
}
</style>