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
          <th scope="col">Share of Prize
            ({{ displayAmount(totalPrizeOnlyLosingDistribution + totalWinningInitialFundsWithAllocations, 2) }})
          </th>
          <th scope="col">Receives</th>
        </tr>
        </thead>
        <tbody>
        <tr v-for="(stats, address) in statistics.playerStatistics" :key="address">
          <td><UserAddressComponent :cut="10" :address="address"/></td>
          <td>{{ displayAmount(stats.totalBet, 2) }}
            <CoinComponent/>
          </td>
          <td>{{ displayAmount(stats.winningPots, 2) }}
            <CoinComponent/>
          </td>
          <td>{{ displayAmount(stats.losingPots, 2) }}
            <CoinComponent/>
          </td>
          <td>{{ displayAmount(stats.redistributionShare, 2) }}
            <CoinComponent/>
            ({{ stats.sharesInPercentage }}%)
            <!--{{ displayAmount(stats.winningFee) }} fee. <CoinComponent/> -->
          </td>
          <td>{{ displayAmount(stats.winningPots + stats.redistributionShare, 2) }}
            <CoinComponent/>
          </td>
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
          <td>$OSMO Prize</td>
          <td>
            {{ displayAmount(raffle.denom_amount, 2) }}
            <CoinComponent/>
          </td>
        </tr>
        <tr>
          <td>{{ timeLeftSeconds ? 'Current ' : '' }}Winner</td>
          <td>
            {{ raffleWinner
            ? `${raffleWinner.substring(0, 15)}...`
            : 'No raffle winner in this round. Balances will be kept by the contract for the next round.'
            }}
            {{ raffleWinner && timeLeftSeconds
            ? ', but the round is still ongoing and it could change.'
            : ''
            }}
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
          {{ winningPots.length
          ? winningPots.map(potId => getPotName(potId)).join(', ')
          : 'There are no winning pots for this round. All the tokens will be reserved for the next game.'
          }}
        </td>
      </tr>
      <tr>
        <td>Losing pots</td>
        <td>
          {{ winningPots.length < 5
          ? pots.filter(pot => !winningPots.includes(pot.pot_id)).map(pot => getPotName(pot.pot_id)).join(', ')
          : 'There are no losing pots for this round. All the tokens will be distributed to the current round winners.'
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
        <td>{{ allPlayersAllocations.length }} players: {{ displayAmount(totalBetsAllPlayers, 2) }}
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
import mxGame from "../../../../frontend-common/mixin/game";
import mxPot from "../../../../frontend-common/mixin/pot";
import CoinComponent from "@/components/Common/CoinComponent.vue";
import mxChain from "../../../../frontend-common/mixin/chain";
import UserAddressComponent from "@/components/Common/UserAddressComponent.vue";

export default {
  name: "PlayersAllocations",
  components: {UserAddressComponent, CoinComponent},

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

    totalWinningInitialFundsWithAllocations() {
      let count = 0
      this.winningPots.forEach(potId => {
        const totalInPot = this.calculateTotalInPots([this.pots.find(p => p.pot_id === potId)]) - this.initialFundsPerPot
        if (totalInPot > 0) count++
      })
      return (this.initialFundsPerPot * count)
    },

    totalBetsAllPlayers() {
      return this.allPlayersAllocations.reduce((totalSum, [, allocationsObj]) => {
        // Accessing allocations array and summing the amounts
        const playerTotal = allocationsObj.reduce((sum, allocation) => {
          return sum + parseInt(allocation.amount); // Convert the amount string to number
        }, 0);
        return totalSum + playerTotal;
      }, 0);
    },

    initialFundsPerPot() {
      // Whatever pot is fine as we always start with the same initial amount per pot
      const whateverPot = {pot_id: this.pots[0].pot_id, amount: this.pots[0].amount}
      const totalPotAllocations = this.calculateTotalInPots(
        this.allPlayersAllocations.flatMap(([, alloc]) =>
          alloc.filter(a => a.pot_id === whateverPot.pot_id)
        )
      );
      return Math.max(0, parseInt(whateverPot.amount) - totalPotAllocations);
    },

    statistics() {
      const redistributionAmount = this.totalPrizeOnlyLosingDistribution;
      const feePercentage = this.gameConfig.fee / 100;
      let playerStatistics = {};
      let totalFees = 0;
      let totalRedistributed = 0;

      this.allPlayersAllocations.forEach(([address, allocations]) => {
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
          stats.redistributionShare += this.getPotInitialFundsShareByPlayer(address);
          stats.redistributionShare += this.getLosingFundsShareByPlayer(address, redistributionAmount);
          stats.sharesInPercentage = (stats.redistributionShare / (redistributionAmount + this.totalWinningInitialFundsWithAllocations) * 100).toFixed(2);

          const fee = stats.redistributionShare * feePercentage;
          stats.winningFee = fee;
          totalFees += fee;

          totalRedistributed += stats.redistributionShare;
        }

        playerStatistics[address] = stats;
      });

      return {playerStatistics, totalFees, totalRedistributed};
    }
  },

  methods: {
    calculateTotalInPots(pots) {
      return pots.reduce((total, {amount}) => total + parseInt(amount), 0);
    },

    getPotInitialFundsShareByPlayer(playerAddress) {
      const winningPots = this.pots.filter(pot => this.winningPots.includes(pot.pot_id));

      let result = 0
      winningPots.forEach(pot => {
        const initialFunds = this.initialFundsPerPot;
        const playerBet = this.allPlayersAllocations
          .find(item => item[0] === playerAddress)[1]
          .find(a => a.pot_id === pot.pot_id)?.amount || "0"

        const totalBetsInPot = this.calculateTotalInPots(
          this.allPlayersAllocations.flatMap(([, alloc]) =>
            alloc.filter(a => a.pot_id === pot.pot_id)
          )
        );

        result += totalBetsInPot > 0
          ? (parseInt(playerBet) / totalBetsInPot) * initialFunds
          : 0;
      });

      return result
    },

    getLosingFundsShareByPlayer(playerAddress, redistributionAmount) {
      const winningPots = this.pots.filter(pot => this.winningPots.includes(pot.pot_id));

      const effectiveWinningTotal = winningPots.reduce((sum, pot) => {
        const initialFunds = this.initialFundsPerPot;
        return sum + (parseInt(pot.amount) - initialFunds); // Subtracting initial funds from each pot's total amount
      }, 0);

      let totalBetsFromPlayerOnWinningPots = 0
      winningPots.forEach(pot => {
        const playerBet = this.allPlayersAllocations
          .find(item => item[0] === playerAddress)[1]
          .find(a => a.pot_id === pot.pot_id)?.amount || "0"
        totalBetsFromPlayerOnWinningPots += parseInt(playerBet)
      });

      return (totalBetsFromPlayerOnWinningPots / effectiveWinningTotal) * redistributionAmount
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
