<template>
  <nav class="navbar navbar-expand-lg p-0">
    <div class="container">
      <router-link class="navbar-brand" to="/">
        <img class="logo" :src="imageLogo" alt="Mad Pots"/>
      </router-link>

      <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarNavAltMarkup"
              aria-controls="navbarNavAltMarkup" aria-expanded="false" aria-label="Toggle navigation">
        <span class="navbar-toggler-icon"></span>
      </button>

      <div class="collapse navbar-collapse" id="navbarNavAltMarkup">
        <div class="navbar-nav ms-auto text-end text-lg-center">
          <router-link to="/" class="nav-link py-lg-4 px-lg-5">Game</router-link>
          <router-link to="instructions" class="nav-link py-lg-4 px-lg-5">Instructions</router-link>
          <div class="nav-link py-lg-4 px-lg-5">
            <WalletComponent/>
          </div>
        </div>
      </div>
    </div>
  </nav>
</template>

<script>
import imageLogo from "@/assets/logo.png"
import WalletComponent from "@/components/Common/WalletComponent.vue";

export default {
  name: "NavbarComponent",
  components: {WalletComponent},

  data() {
    return {
      imageLogo
    }
  }
}
</script>

<style scoped lang="scss">
@import "@/assets/style.scss";
.navbar-toggler {
  border-color: $pp-color-5; // Change the border color if needed

  .navbar-toggler-icon {
    background-image: url('data:image/svg+xml;charset=UTF8,<svg xmlns="http://www.w3.org/2000/svg" width="30" height="30" fill="%2388E54A" viewBox="0 0 30 30"><path stroke="%2388E54A" stroke-linecap="round" stroke-miterlimit="10" stroke-width="2" d="M4 7h22M4 15h22M4 23h22"/></svg>');
  }
}



.navbar {
  .container {
    border: 1px solid $pp-color-5;

    .logo {
      height: 60px;
    }

    .navbar-nav {
      text-transform: uppercase;

      .nav-link, .navbar-toggler-icon {
        color: $pp-color-5;
        @media(min-width: 992px) {
          border-left: 1px solid $pp-color-5;
        }
      }
    }
  }
}
</style>