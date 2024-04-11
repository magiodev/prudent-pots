<template>
  <!-- Button trigger modal -->
  <div class="readme-component d-flex justify-content-end">
    <ButtonComponent text="README" data-bs-toggle="modal" data-bs-target="#exampleModal" :is-small="true"/>
  </div>

  <!-- Modal -->
  <div class="readme-modal modal fade modal-lg" id="exampleModal" tabindex="-1" aria-labelledby="exampleModalLabel"
       aria-hidden="true">
    <div class="modal-dialog">
      <div class="modal-content">
        <div class="modal-body p-5" v-if="gameConfig">
          <div class="d-flex justify-content-end">
            <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
          </div>

          <h3>Initial Setup</h3>
          <p>The game board features a row of 5 pots, each with unique winning rules. Initially, the contract balance is
            evenly distributed among these pots, sourced either from the developer's funds for game instantiation or
            remaining funds from previous games' losing pots.</p>

          <h3>Game Duration</h3>
          <p>Each round lasts {{ gameDuration }}, with potential extensions if players make allocation or
            reallocation actions within the last {{ gameConfig.game_extend }} seconds of the game, resetting the timer.
            Following the distribution of winnings, the next game starts immediately.</p>

          <h3>Pot Rules</h3>
          <ul>
            <li><strong>Pot 1 (Lowest Pot):</strong> Wins with the lowest token count.</li>
            <li><strong>Pot 2 (Even Pot):</strong> Wins with an even token count.</li>
            <li><strong>Pot 3 (Median Pot):</strong> Wins by holding the median token count.</li>
            <li><strong>Pot 4 (Odd Pot):</strong> Wins with an odd token count.</li>
            <li><strong>Pot 5 (Highest Pot):</strong> Wins with the highest token count.</li>
          </ul>

          <h3>Token Allocation and Reallocation</h3>
          <p>Players can allocate tokens to any pot without a fee. Yet, reallocating tokens incurs a
            {{ gameConfig.fee_reallocation }}% fee. This reallocation fee contributes to the next game's pool,
            encouraging
            players to make thoughtful decisions about their token placements.</p>

          <h3>Dynamic Bid Constraints and Reallocation Limits</h3>
          <p>Minimum and maximum bid limits are dynamically set based on the average token count across pots, promoting
            strategic engagement and fairness. These constraints apply to both allocations and reallocations.</p>

          <h3>Winning Pot Determination</h3>
          <p>The winning pot is determined by its specific rules, and players in this pot receive their proportional
            share
            of the total tokens, post the deduction of a {{ gameConfig.fee }}% winning fee, along with redistributed
            tokens from the less successful pots.</p>

          <p>Explore detailed gameplay examples and strategic insights at the <a
            href="https://github.com/magiodev/prudent-pots">GitHub repository</a>.</p>

          <!-- "Do not show again" link -->
          <div class="text-center mb-5 pb-5">
            <ButtonComponent text="Do not show again" :is-small="true" @click.prevent="setDontShowAgain" />
          </div>
        </div>

        <LoadingComponent v-else/>
      </div>
    </div>
  </div>
</template>

<script>
import {mapGetters} from "vuex";
import LoadingComponent from "@/App.vue";
import ButtonComponent from "@/components/Common/ButtonComponent.vue";
import {Modal} from "bootstrap"

export default {
  name: "ReadmeComponent",
  components: {ButtonComponent, LoadingComponent},

  data() {
    return {
      dontShowAgain: false,
    };
  },

  mounted() {
    this.checkDontShowAgain();
    if (!this.dontShowAgain) {
      this.showModal();
    }
  },

  methods: {
    checkDontShowAgain() {
      this.dontShowAgain = localStorage.getItem('dontShowReadme') === 'true';
    },

    setDontShowAgain() {
      localStorage.setItem('dontShowReadme', 'true');
      this.dontShowAgain = true;
      this.$nextTick(() => {
        const modalElement = document.getElementById('exampleModal');
        const modalInstance = Modal.getInstance(modalElement);
        if (modalInstance) {
          modalInstance.hide();
        }
      });
    },

    showModal() {
      this.$nextTick(() => {
        const modal = new Modal(document.getElementById('exampleModal'));
        modal.show();
      });
    },
  },

  computed: {
    ...mapGetters(['gameConfig']),

    gameDuration() {
      const minutes = Math.floor(this.gameConfig.game_duration / 60);
      const seconds = this.gameConfig.game_duration % 60;
      return `${minutes} minutes and ${seconds} seconds`;
    },
  },
}
</script>

<style lang="scss" scoped>
.readme-modal {
  .modal-content {
    background: transparent;
    border: none;
    box-shadow: none;
  }

  .modal-header, .modal-footer {
    background: transparent;
    border: none;
  }

  .modal-body {
    background: url("@/assets/tomb-bg.png");
    background-size: 100% 100%;
    background-repeat: no-repeat;
    background-position: center;
  }
}
</style>