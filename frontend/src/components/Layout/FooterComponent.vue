<template>
  <div class="container">
    <footer class="d-flex flex-wrap justify-content-between align-items-center py-3 my-4 border-top">
      <p class="col-md-4 mb-0">© {{ new Date().getFullYear() }} Prudent Pots</p>

      <div
        class="col-md-4 d-flex flex-column align-items-center justify-content-center mb-3 mb-md-0 me-md-auto link-dark text-decoration-none">
        <img class="logo" src="favicon.ico" alt="Prudent Pots">
      </div>

      <ul class="nav col-md-4 justify-content-end list-unstyled d-flex">
        <li class="ms-3"><a href="#">
          <b-icon-twitter></b-icon-twitter>
        </a></li>
        <li class="ms-3"><a href="#">
          <b-icon-discord></b-icon-discord>
        </a></li>
      </ul>
    </footer>
  </div>
</template>

<script>
export default {
  name: "FooterComponent",
}
</script>

<style scoped lang="scss">
.logo {
  height: 32px;
}
</style>