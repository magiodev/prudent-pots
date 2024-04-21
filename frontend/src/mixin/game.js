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
      'fetchPots',
      'fetchWinningPots',
      'fetchBidRange',
      'fetchReallocationFeePool',
      'fetchPlayerData',
    ]),

    async fetchOnce() {
      // Init signer and querier
      await this.initUser();
      if (this.userAddress) {
        console.log("Init user successful. Fetching balance and allocations.")
        await this.fetchPlayerData();
      }
      console.log("Fetching general information.")
      await this.fetchGameConfig();
      await this.fetchGameState();
    },

    async fetchInterval(gameEnd = false) {
      await this.fetchPots();
      await this.fetchWinningPots();
      await this.fetchBidRange();
      await this.fetchReallocationFeePool();
      if (gameEnd) await this.fetchGameState();
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
