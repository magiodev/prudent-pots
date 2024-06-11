<template>
  <div class="game-activity-component position-relative">
    <h3>Game Activity</h3>
    <div class="rounds overflow-y-scroll">
      <div class="pp-card round mb-3 p-3" v-for="group in paginatedRounds" :key="group.round_count">
        <h5>Round #{{ group.round_count }}</h5>

        <div class="round-content small" v-for="(tx, index) in [...group.transactions].reverse()" :key="index">
          <hr/>

          <!-- Only: allocates_tokens -->
          <div class="allocate-tokens" v-if="getTxEvents(tx.events)[0].action === 'allocate_tokens'">
            <!-- Reversed index -->
            <span class="badge bg-pp-color-4 text-black me-1">
              #{{ group.transactions.length - index }}
            </span>

            <span class="me-1">
              <UserAddressComponent :cut="10" :address="getTxEvents(tx.events)[0].player"/>
            </span>

            <span class="me-1" v-if="getTxEvents(tx.events)[0].action">
              {{ getActionName(getTxEvents(tx.events)[0].action) }}
            </span>

            <!-- Common -->
            <span class="me-1" v-if="getTxEvents(tx.events)[0].amount">
              {{ displayAmount(getTxEvents(tx.events)[0].amount, 2) }} <CoinComponent/>
            </span>

            <!-- Only: allocate_tokens -->
            <span class="me-1" v-if="getTxEvents(tx.events)[0].pot_id">
              to {{ getPotName(getTxEvents(tx.events)[0].pot_id) }}:
            </span>

            <!-- Common -->
            <span>
              <a :href="`${explorerBaseUrl}/${chainId}/txs/${tx.transactionHash}`" target="_blank"
                 class="align-text-bottom">
                <b-icon-box-arrow-up-right></b-icon-box-arrow-up-right>
              </a>
            </span>
          </div>

          <!-- Only: reallocates_tokens -->
          <div class="reallocate-tokens" v-if="getTxEvents(tx.events)[0].action === 'reallocate_tokens'">
            <!-- Reversed index -->
            <span class="badge bg-pp-color-4 text-black me-1" v-if="getTxEvents(tx.events)[0].action !== 'game_end'">
              #{{ group.transactions.length - index }}
            </span>

            <span class="me-1">
              <UserAddressComponent :cut="10" :address="getTxEvents(tx.events)[0].player"/>
            </span>

            <span class="me-1">
              {{ getActionName(getTxEvents(tx.events)[0].action) }}
            </span>

            <!-- Common -->
            <span class="me-1">
              {{ displayAmount(getTxEvents(tx.events)[0].amount, 2) }} <CoinComponent/>
            </span>
            <!-- Only: reallocate_tokens -->
            <span class="me-1">
              paying {{ displayAmount(getTxEvents(tx.events)[0].fee, 4) }} fee
            </span>

            <!-- Only: reallocate_tokens -->
            <span class="me-1">
              from {{ getPotName(getTxEvents(tx.events)[0].from_pot_id) }} <b-icon-arrow-right></b-icon-arrow-right>
              {{ getPotName(getTxEvents(tx.events)[0].to_pot_id) }}:
            </span>

            <!-- Common -->
            <span>
              <a :href="`${explorerBaseUrl}/${chainId}/txs/${tx.transactionHash}`" target="_blank"
                 class="align-text-bottom">
                <b-icon-box-arrow-up-right></b-icon-box-arrow-up-right>
              </a>
            </span>
          </div>

          <!-- Only: game_end -->
          <div class="game-end" v-if="getTxEvents(tx.events)[0].action === 'game_end'">
            <span class="me-1" v-if="getTxEvents(tx.events)[0].action">
              {{ getActionName(getTxEvents(tx.events)[0].action) }}
            </span>

            <!-- Common -->
            <span>
              <a :href="`${explorerBaseUrl}/${chainId}/txs/${tx.transactionHash}`" target="_blank"
                 class="align-text-bottom">
                <b-icon-box-arrow-up-right></b-icon-box-arrow-up-right>
              </a>
            </span>

            <ul class="list-unstyled">
              <li>
                <strong>Winning Pots:</strong> {{
                  formattedPotNames(JSON.parse(getTxEvents(tx.events)[0].winning_pots))
                }}
              </li>

              <li>
                <strong>Winning Distribution:</strong>
                {{ displayAmount(getTxEvents(tx.events)[0].winning_outgoing_tokens, 2) }}
                <CoinComponent/>
              </li>
              <li>
                <strong>Winning Treasury Fee:</strong>
                {{ displayAmount(getTxEvents(tx.events)[0].treasury_outgoing_tokens, 2) }}
                <CoinComponent/>
              </li>
              <li>
                <strong>Total Outgoing Tokens:</strong>
                {{ displayAmount(getTxEvents(tx.events)[0].total_outgoing_tokens, 2) }}
                <CoinComponent/>
              </li>
              <li>
                <strong>Raffle Winner:</strong>
                <UserAddressComponent v-if="getTxEvents(tx.events)[0].raffle_winner" :cut="10"
                                      :address="getTxEvents(tx.events)[0].raffle_winner"/>
                <span v-else>No Winner</span>
              </li>
              <ul>
                <li>
                  <strong>Prize to Winner:</strong>
                  {{ displayAmount(getTxEvents(tx.events)[0].raffle_outgoing_tokens_winner, 2) }}
                  <CoinComponent/>
                </li>
                <li>
                  <strong>Prize to Treasury:</strong>
                  {{ displayAmount(getTxEvents(tx.events)[0].raffle_outgoing_tokens_treasury, 2) }}
                  <CoinComponent/>
                </li>
                <li v-if="getTxEvents(tx.events)[0].raffle_outgoing_nft_id">
                  <strong>Prize NFT:</strong> MS #{{ getTxEvents(tx.events)[0].raffle_outgoing_nft_id }}
                </li>
              </ul>
            </ul>
          </div>
        </div>
      </div>
    </div>

    <nav aria-label="Page navigation example" class="mt-3">
      <ul class="pagination justify-content-center">
        <li class="page-item" :class="{ disabled: currentPage === this.totalPages }">
          <button class="page-link" @click="changePage(currentPage + 1)" aria-label="Previous">
            <span aria-hidden="true">&laquo;</span>
          </button>
        </li>
        <li class="page-item" v-for="page in visiblePages" :key="page" :class="{ active: currentPage === page }">
          <button class="page-link" @click="changePage(page)">{{ page }}</button>
        </li>
        <li class="page-item" :class="{ disabled: currentPage === 1 }">
          <button class="page-link" @click="changePage(currentPage - 1)" aria-label="Next">
            <span aria-hidden="true">&raquo;</span>
          </button>
        </li>
      </ul>
    </nav>
  </div>
</template>

<script>
import {mapActions, mapGetters, mapMutations} from 'vuex';
import mxPot from "../../../../frontend-common/mixin/pot";
import mxChain from "../../../../frontend-common/mixin/chain";
import CoinComponent from "@/components/Common/CoinComponent.vue";
import UserAddressComponent from "@/components/Common/UserAddressComponent.vue";

export default {
  name: "GameActivityComponent",
  components: {UserAddressComponent, CoinComponent},

  mixins: [mxPot, mxChain],

  computed: {
    ...mapGetters(['gameActivity', 'gameState']),

    totalPages() {
      return Math.ceil(this.gameState.round_count / this.itemsPerPage);
    },

    paginatedRounds() {
      return this.gameActivity;
    },

    pages() {
      // Generate pages in reverse order
      return Array.from({length: this.totalPages}, (_, i) => this.totalPages - i);
    },

    visiblePages() {
      const totalPages = this.totalPages;
      const currentPage = this.currentPage;
      const maxVisiblePages = 5;

      let startPage = Math.max(1, currentPage - Math.floor(maxVisiblePages / 2));
      let endPage = startPage + maxVisiblePages - 1;

      if (endPage > totalPages) {
        endPage = totalPages;
        startPage = Math.max(1, endPage - maxVisiblePages + 1);
      }

      startPage = Math.max(0, totalPages - endPage);
      endPage = Math.min(totalPages, startPage + maxVisiblePages);

      return this.pages.slice(startPage, endPage);
    }

  },

  data() {
    return {
      explorerBaseUrl: process.env.VUE_APP_EXPLORER_BASE_URL,
      chainId: process.env.VUE_APP_CHAIN_ID,
      currentPage: null,
      itemsPerPage: 1
    };
  },

  methods: {
    ...mapActions(['fetchGameActivity']),
    ...mapMutations(['setGameActivitySelectedRound']),

    getActionName(action) {
      switch (action) {
        case "allocate_tokens":
          return 'allocates'
        case "reallocate_tokens":
          return 'reallocates'
        case "game_end":
          return 'Prize distribution and next round preparation.'
        default:
          return
      }
    },

    getTxEvents(events) {
      const relevantDetails = [];

      events.forEach(event => {
        if (event.type === "wasm") {
          const attributes = event.attributes.reduce((acc, attr) => {
            acc[attr.key] = attr.value;
            return acc;
          }, {});

          switch (attributes.action) {
            case 'allocate_tokens':
            case 'reallocate_tokens':
            case 'game_end':
              relevantDetails.push(attributes);
              break;
            default:
              break;
          }
        }
      });

      return relevantDetails.length > 0 ? relevantDetails : "No relevant transaction data";
    },

    async changePage(page) {
      if (page > 0 && page <= this.totalPages) {
        this.currentPage = page;
        this.setGameActivitySelectedRound(page)
        await this.fetchGameActivity(page);
      }
    }
  },

  async created() {
    this.currentPage = this.gameState.round_count;
    await this.fetchGameActivity(this.currentPage);
  }
};
</script>

<style lang="scss" scoped>
@import "@/assets/style.scss";

.rounds {
  max-height: 80vh;
  /* Hide scrollbar for IE, Edge and Firefox */
  -ms-overflow-style: none; /* IE and Edge */
  scrollbar-width: none; /* Firefox */
}

/* Hide scrollbar for Chrome, Safari and Opera */
.rounds::-webkit-scrollbar {
  display: none;
}

.page-link {
  color: black;
}
.active>.page-link, .page-link.active {
  background-color: $pp-color-5;
  border-color: $pp-color-5;
  color: black;
}
</style>
