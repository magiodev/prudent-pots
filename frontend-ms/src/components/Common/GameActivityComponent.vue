<template>
  <div class="game-activity-component position-relative">
    <h3>Game Activity</h3>
    <div class="rounds overflow-y-scroll">
      <div class="pp-card round mb-3 p-3" v-for="group in gameActivity" :key="group.round_count">
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

            <!-- Only: allocate_tokens -->
            <span class="me-1" v-if="getTxEvents(tx.events)[0].pot_id">
              to {{ getPotName(getTxEvents(tx.events)[0].pot_id) }}:
            </span>

            <!-- Common -->
            <span class="me-1" v-if="getTxEvents(tx.events)[0].amount">
              {{ displayAmount(getTxEvents(tx.events)[0].amount, 2) }} <CoinComponent/>
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

            <!-- Only: reallocate_tokens -->
            <span class="me-1">
              from {{ getPotName(getTxEvents(tx.events)[0].from_pot_id) }} <b-icon-arrow-right></b-icon-arrow-right>
              {{ getPotName(getTxEvents(tx.events)[0].to_pot_id) }}:
            </span>

            <!-- Common -->
            <span class="me-1">
              {{ displayAmount(getTxEvents(tx.events)[0].amount, 2) }} <CoinComponent/>
            </span>
            <!-- Only: reallocate_tokens -->
            <span class="me-1">
              paying {{ displayAmount(getTxEvents(tx.events)[0].fee, 4) }} fee
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

            <ul>
              <li>
                Winning Pots: {{ formattedPotNames(JSON.parse(getTxEvents(tx.events)[0].winning_pots)) }}
              </li>
              <li>
                Outgoing Tokens: {{ displayAmount(getTxEvents(tx.events)[0].total_outgoing_tokens, 2) }}
                <CoinComponent/>
              </li>
              <li v-if=getTxEvents(tx.events)[0].token_id>
                 MS #{{ getTxEvents(tx.events)[0].token_id }}
              </li>
            </ul>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import {mapGetters} from 'vuex';
import mxPot from "../../../../frontend-common/mixin/pot";
import mxChain from "../../../../frontend-common/mixin/chain";
import CoinComponent from "@/components/Common/CoinComponent.vue";
import UserAddressComponent from "@/components/Common/UserAddressComponent.vue";

export default {
  name: "GameActivityComponent",
  components: {UserAddressComponent, CoinComponent},

  mixins: [mxPot, mxChain],

  computed: {
    ...mapGetters(['gameActivity'])
  },

  data() {
    return {
      explorerBaseUrl: process.env.VUE_APP_EXPLORER_BASE_URL,
      chainId: process.env.VUE_APP_CHAIN_ID
    }
  },

  methods: {
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

          // Filter attributes based on the action
          switch (attributes.action) {
            case 'allocate_tokens':
            case 'reallocate_tokens':
            case 'game_end':
            case 'transfer_nft': // TODO: Doesnt appear
              relevantDetails.push(attributes);
              break;
            default:
              break;
          }
        }
      });

      return relevantDetails.length > 0 ? relevantDetails : "No relevant transaction data";
    }
  }
}
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
</style>
