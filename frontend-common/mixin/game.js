import {mapActions, mapGetters} from "vuex";

const mxGame = {
  data() {
    return {
      currentTime: new Date().getTime(),
    };
  },

  computed: {
    ...mapGetters(['userAddress', 'gameState']),

    timeLeftSeconds() {
      if (!this.gameState) return null;
      const endTime = this.gameState.end_time * 1000;
      const timeDiff = endTime - this.currentTime;
      return timeDiff > 0 ? Math.floor(timeDiff / 1000) : 0;
    },

    timeLeftHuman() {
      const timeDiff = this.timeLeftSeconds * 1000;
      if (timeDiff <= 0) {
        return "0d 0h 0m 0s";
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

    async fetchOnce() {
      await this.initUser();

      await this.fetchGameConfig();
      await this.fetchGameState();
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
      // TODO: await this.fetchRaffleDenomSplit()
      // try catch to ignore game activity errors
      try {
        this.fetchGameActivity()
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
