<template>
  <div class="col players-allocations mb-3">
    <h3 class="text-center">User Allocations</h3>

    <div class="overflow-x-scroll" v-if="Object.entries(statistics.playerStatistics).length">
      <table class="table always-left mb-0 small">
        <thead>
        <tr>
          <th scope="col">Player</th>
          <th scope="col">Total</th>
          <th scope="col">Winning</th>
          <th scope="col">Losing</th>
          <th scope="col">Share of Prize ({{ displayAmount(totalPrizeOnlyLosingDistribution + (calculateInitialFundsShare({
            pot_id: this.pots[0].pot_id,
            amount: this.pots[0].amount
          }) * this.winningPots.length), 2) }})</th>
          <th scope="col">Receives</th>
        </tr>
        </thead>
        <tbody>
        <tr v-for="(stats, address) in statistics.playerStatistics" :key="address">
          <td>{{ address.substring(0, 15) }}...</td>
          <td>{{ displayAmount(stats.totalBet, 2) }} <CoinComponent/></td>
          <td>{{ displayAmount(stats.winningPots, 2) }} <CoinComponent/></td>
          <td>{{ displayAmount(stats.losingPots, 2) }} <CoinComponent/></td>
          <td>{{ displayAmount(stats.redistributionShare, 2) }} <CoinComponent/> ({{ stats.sharesInPercentage }}%)
            <!--{{ displayAmount(stats.winningFee) }} fee. <CoinComponent/> -->
          </td>
          <td>{{ displayAmount(stats.winningPots + stats.redistributionShare, 2) }} <CoinComponent/></td>
        </tr>
        </tbody>
      </table>
    </div>
    <p v-else class="text-center text-pp-color-4 small">There are no bets in this round yet.</p>
  </div>

  <div class="col raffle text-white mb-3">
    <h3 class="text-center">Raffle</h3>

    <template v-if="raffle.cw721_token_id || Number(raffle.denom_amount)">
      <table class="table m-0 bg-transparent text-pp-color-3 small">
        <tbody>
        <tr>
          <td>NFT Prize</td>
          <td>
            {{ raffle?.nft?.id ? `MS #${raffle.nft.id}` : 'There is raffle NFT for this round.' }}
          </td>
        </tr>
        <tr>
          <td>Denom Prize</td>
          <td>
            {{ displayAmount(raffle.denom_amount, 2) }}
            <CoinComponent/>
          </td>
        </tr>
        <tr>
          <td>{{ timeLeftSeconds ? 'Current ' : '' }}Winner</td>
          <td>
            {{ raffleWinner ? `${raffleWinner.substring(0, 15)}...` : 'No raffle winner in this round. Balances will be kept by the contract for the next round.'
            }}{{ raffleWinner && timeLeftSeconds ? ', but the round is still ongoing and it could change.' : '' }}
          </td>
        </tr>
        <tr v-if="Number(raffle.denom_amount)">
          <td>Treasury Split</td>
          <td>Raffle $ prize will be split, as {{ gameState.extend_count }} time extends:
            <ul class="list-unstyled">
              <li v-if="raffleWinner">Winner: {{ displayAmount(raffleDenomSplit.distributedPrize, 2) }}
                <CoinComponent/>
              </li>
              <li v-else>Contract: {{ displayAmount(raffleDenomSplit.distributedPrize, 2) }}
                <CoinComponent/>
              </li>
              <li>Treasury: {{ displayAmount(raffleDenomSplit.remainingPrize, 2) }}
                <CoinComponent/>
              </li>
            </ul>
          </td>
        </tr>
        </tbody>
      </table>
    </template>
    <p v-else class="text-center text-pp-color-4 small">There are no raffle prizes assigned to this round.</p>
  </div>

  <div class="col general-stats text-center text-white" v-if="!timeLeftSeconds">
    <h3 class="text-center">General Winning Stats</h3>

    <table class="table m-0 small">
      <tbody>
      <tr>
        <td>Winning pots</td>
        <td>
          {{ winningPots.length ? winningPots.map(potId => getPotName(potId)).join(', ') : 'There are no winning pots for this round. All the tokens will be reserved for the next game.'
          }}
        </td>
      </tr>
      <tr>
        <td>Losing pots</td>
        <td>
          {{ winningPots.length < 5 ? pots.filter(pot => !winningPots.includes(pot.pot_id)).map(pot => getPotName(pot.pot_id)).join(', ') : 'There are no losing pots for this round. All the tokens will be distributed to the current round winners.'
          }}
        </td>
      </tr>
      <tr>
        <td>Total Distributed Losing (Incl. Fees)</td>
        <td>{{ displayAmount(statistics.totalRedistributed, 2) }}
          <CoinComponent/>
        </td>
      </tr>
      <tr>
        <td>Total Distributed From Losing (Net Fees)</td>
        <td>{{ displayAmount(statistics.totalRedistributed - statistics.totalFees, 2) }}
          <CoinComponent/>
        </td>
      </tr>
      <tr>
        <td>Total Fees Collected ({{ gameConfig.fee.toFixed(2) }}%)</td>
        <td>{{ displayAmount(statistics.totalFees, 2) }}
          <CoinComponent/>
        </td>
      </tr>
      <tr>
        <td>Amount Reserved for Next Game</td>
        <td>
          {{ displayAmount(totalPrizeOnlyLosingDistribution, 2) }}
          <CoinComponent/>
          <br/>
          x pot: {{ displayAmount(totalPrizeOnlyLosingDistribution / 5, 2) }}
          <CoinComponent/>
        </td>
      </tr>
      <tr>
        <td>Total bets</td>
        <td>{{ allPlayersAllocations.length }} players: {{ displayAmount(calculateTotalBets(), 2) }}
          <CoinComponent/>
        </td>
      </tr>
      <tr>
        <td>Reallocation fee pool</td>
        <td>{{ displayAmount(reallocationFeePool, 4) }}
          <CoinComponent/>
        </td>
      </tr>
      </tbody>
    </table>
  </div>
</template>

<script>
import {mapGetters} from 'vuex';
import mxGame from "@/mixin/game";
import mxPot from "@/mixin/pot";
import CoinComponent from "@/components/Common/CoinComponent.vue";
import mxChain from "@/mixin/chain";

export default {
  name: "PlayersAllocations",
  components: {CoinComponent},

  mixins: [mxGame, mxPot, mxChain],

  computed: {
    ...mapGetters(['allPlayersAllocations', 'winningPots', 'pots', 'gameConfig', "reallocationFeePool", "raffle", "raffleWinner", "gameState"]),

    raffleDenomSplit() {
      const extendCount = parseInt(this.gameState.extend_count)
      const decayFactor = parseInt(this.gameConfig.decay_factor)
      const denomAmount = parseInt(this.raffle.denom_amount)

      let prizePercentage = 100;

      for (let i = 0; i < extendCount; i++) {
        prizePercentage *= (decayFactor / 100);
      }
      const distributedPrize = denomAmount * (prizePercentage / 100);
      const remainingPrize = denomAmount - distributedPrize;

      return {
        distributedPrize: Math.floor(distributedPrize), // Assuming integer results
        remainingPrize: Math.floor(remainingPrize)
      };
    },

    totalPrizeOnlyLosingDistribution() {
      const losingPots = this.pots.filter(pot => !this.winningPots.includes(pot.pot_id));
      const totalLosing = this.calculateTotalInPots(losingPots)
      // 50% of losing pots' amounts for redistribution only in case of at least 1 pot wins
      return this.winningPots.length
        ? totalLosing * 0.5
        : totalLosing; // if not pot wins
    },

    statistics() {
      const redistributionAmount = this.totalPrizeOnlyLosingDistribution;
      const feePercentage = this.gameConfig.fee / 100;
      let playerStatistics = {};
      let totalFees = 0;
      let totalRedistributed = 0;

      this.allPlayersAllocations.forEach(([address, {allocations}]) => {
        let stats = {
          totalBet: 0,
          winningPots: 0,
          losingPots: 0,
          redistributionShare: 0,
          sharesInPercentage: 0,
          winningFee: 0
        };

        allocations.forEach(({pot_id, amount}) => {
          const amountNumeric = parseInt(amount);
          stats.totalBet += amountNumeric;

          if (this.winningPots.includes(pot_id)) {
            stats.winningPots += amountNumeric;
          } else {
            stats.losingPots += amountNumeric;
          }
        });

        if (stats.winningPots > 0) {
          const winningPots = this.pots.filter(pot => this.winningPots.includes(pot.pot_id));
          const effectiveWinningTotal = winningPots.reduce((sum, pot) => {
            const initialFunds = this.calculateInitialFundsShare(pot);
            return sum + (parseInt(pot.amount) - initialFunds); // Subtracting initial funds from each pot's total amount
          }, 0);

          // TODO: This is working with just one user, but with more users we are inflating the redistributionShare as we are arbitrarily adding the same initial allocated funds from winning pots to all of them.
          // TODO: The concept is that in a pot with i.e. 10 winning tokens, there will always be some initial fund allocated i.e. 1.0 which should be distributed proportionally between the same pot's contributors.
          // TODO: A concrete example is that there are 2 winning pots, and 2 users playing. The first user bet in both, the second one only in the second. The first one a part from the players bet, as the others, have 1 token from initial allocation. In this pot the user1 bet 1 token, the user2 bet 2 tokens. The user 2 should take 2:1 respect the user1 only form this pot. In the second pot where only the user2 played and won, the extra 1.0 token should be distributed only to him.
          stats.redistributionShare = redistributionAmount * (stats.winningPots / effectiveWinningTotal) + this.calculateInitialFundsShare({
            pot_id: this.pots[0].pot_id,
            amount: this.pots[0].amount
          }) * this.winningPots.length;
          const fee = stats.redistributionShare * feePercentage;
          stats.winningFee = fee;
          totalFees += fee;
          totalRedistributed += stats.redistributionShare;
          stats.sharesInPercentage = (stats.redistributionShare / (redistributionAmount + this.calculateInitialFundsShare({
            pot_id: this.pots[0].pot_id,
            amount: this.pots[0].amount
          }) * this.winningPots.length) * 100).toFixed(2);
        }

        playerStatistics[address] = stats;
      });

      return {playerStatistics, totalFees, totalRedistributed};
    }
  },

  methods: {
    calculateTotalBets() {
      return this.allPlayersAllocations.reduce((totalSum, [, allocationsObj]) => {
        // Accessing allocations array and summing the amounts
        const playerTotal = allocationsObj.allocations.reduce((sum, allocation) => {
          return sum + parseInt(allocation.amount); // Convert the amount string to number
        }, 0);
        return totalSum + playerTotal;
      }, 0);
    },

    calculateTotalInPots(pots) {
      return pots.reduce((total, {amount}) => total + parseInt(amount), 0);
    },

    calculateInitialFundsShare(pot) {
      const potInitialFunds = parseInt(pot.amount);
      const totalPotAllocations = this.calculateTotalInPots(
        this.allPlayersAllocations.flatMap(([, alloc]) =>
          alloc.allocations.filter(a => a.pot_id === pot.pot_id)
        )
      );
      return potInitialFunds - totalPotAllocations;
    }
  }
}
</script>

<style lang="scss" scoped>
@import "@/assets/style";

.table {
  width: 100%;
  border-collapse: collapse;
}

.table th, .table td {
  padding: 8px;
  text-align: left;
  //border-bottom: 1px solid #ddd;

  background-color: transparent;
  border-color: $pp-color-4;
  color: $pp-color-4 !important;
}

.table:not(.always-left) td:last-child {
  text-align: right;
}

.summary-section p {
  font-size: 1.1em;
}
</style>
