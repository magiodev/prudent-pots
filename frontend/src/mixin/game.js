import {mapActions, mapGetters} from "vuex";

const mxGame = {
  computed: {
    ...mapGetters(['userAddress'])
  },

  methods: {
    ...mapActions(['initUser', 'fetchGameConfig', 'fetchGameState', 'fetchPots', 'fetchWinningPots', 'fetchBidRange', 'fetchReallocationFeePool', 'fetchUserAllocations']),

    async fetchOnce() {
      await this.initUser();
      if (this.userAddress) await this.fetchUserAllocations()
      await this.fetchGameConfig()
    },

    async fetchInterval() {
      await this.fetchGameState()
      await this.fetchPots()
      await this.fetchWinningPots()
      await this.fetchBidRange()
      await this.fetchReallocationFeePool()
    }
  }
}

export default mxGame
