<template>
  <div class="footer container bg-pp-color-3">
    <footer class="row align-items-center py-3 text-white">
      <div class="col-md-4 text-md-start text-center mb-2 mb-md-0">
        © {{ new Date().getFullYear() }} Mad Pots
      </div>

      <div class="col-md-4 text-center d-none d-md-block">
        <img class="logo" src="favicon.png" alt="Mad Pots">
      </div>

      <div class="col-md-4">
        <ul class="nav list-unstyled d-flex justify-content-center justify-content-md-end">
          <li class="me-3">
            <a href="https://github.com/magiodev/prudent-pots" target="_blank" rel="noopener noreferrer">
              <b-icon-github class="text-white"></b-icon-github>
            </a>
          </li>
          <li class="me-3">
            <a href="https://x.com/magiodev/status/1800558963278201023" target="_blank" rel="noopener noreferrer">
              <b-icon-twitter class="text-white"></b-icon-twitter>
            </a>
          </li>
          <li>
            <a href="https://discord.gg/madscientists" target="_blank" rel="noopener noreferrer">
              <b-icon-discord class="text-white"></b-icon-discord>
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