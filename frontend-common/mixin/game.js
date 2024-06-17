import { mapActions, mapGetters, mapMutations } from "vuex";

const mxGame = {
  data() {
    return {
      currentTime: new Date().getTime(),
    };
  },

  computed: {
    ...mapGetters(['userAddress', 'gameState', "gameActivitySelectedRound"]),

    isCountingDownToStart() {
      if (!this.gameState) return null;
      const startTime = this.gameState.start_time * 1000;
      return this.currentTime < startTime;
    },

    timeLeftSeconds() {
      if (!this.gameState) return null;
      // take the start time
      const startTime = this.gameState.start_time * 1000;
      // take the end time
      const endTime = this.gameState.end_time * 1000;
      const now = this.currentTime;
      if (now < startTime) {
        // we are before the start
        return Math.floor((startTime - now) / 1000); // Countdown to start
      } else if (now < endTime) {
        // we are during the game
        return Math.floor((endTime - now) / 1000); // Countdown to end
      } else {
        // the game finished and it hasnt been restarted/scheduled
        return 0;
      }
    },

    timeLeftHuman() {
      const timeDiff = this.timeLeftSeconds * 1000;
      if (timeDiff <= 0) {
        return "Time's up!";
      }
      const days = Math.floor(timeDiff / (1000 * 60 * 60 * 24));
      const hours = Math.floor((timeDiff / (1000 * 60 * 60)) % 24);
      const minutes = Math.floor((timeDiff / (1000 * 60)) % 60);
      const seconds = Math.floor((timeDiff / 1000) % 60);
      return `${days}d ${hours}h ${minutes}m ${seconds}s`;
    },
  },

  methods: {
    ...mapActions([
      'initUser',
      'fetchGameConfig',
      'fetchGameState',
      'fetchGameActivity',
      'fetchAllPlayersAllocations',
      'fetchPots',
      'fetchWinningPots',
      'fetchBidRange',
      'fetchReallocationFeePool',
      'fetchPlayerData',
      'fetchCw721Tokens',
      "fetchRaffle",
      "fetchRaffleWinner",
      "fetchRaffleDenomSplit"
    ]),
    ...mapMutations(['setGameActivitySelectedRound']),

    async fetchOnce() {
      await this.initUser();

      await this.fetchGameConfig();
      await this.fetchGameState();
      this.setGameActivitySelectedRound(this.gameState.round_count) // set the current round for the paginated gameActivity fetch
      await this.fetchAllPlayersAllocations() // this is also included in the fetchInterval, we do it twice only the first App render

      // Init signer and querier
      if (this.userAddress) {
        await this.fetchPlayerData();
        await this.fetchCw721Tokens()
      }
    },

    async fetchInterval(gameEnd = false) {
      await this.fetchAllPlayersAllocations()
      await this.fetchPots();
      await this.fetchWinningPots();
      await this.fetchBidRange();
      await this.fetchReallocationFeePool();
      if (gameEnd) await this.fetchGameState();
      // Raffle
      await this.fetchRaffle()
      await this.fetchRaffleWinner()
      // try catch to ignore game activity errors
      try {
        await this.fetchGameActivity(this.gameActivitySelectedRound)
      } catch (e) {
        console.log(e)
      }
    },

    updateCurrentTime() {
      this.currentTime = new Date().getTime();
    },
  },

  created() {
    this.intervalId = setInterval(this.updateCurrentTime, 1000);
  },

  unmounted() {
    clearInterval(this.intervalId);
  },
};

export default mxGame;
