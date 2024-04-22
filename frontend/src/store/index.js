import {createStore} from "vuex";
import {AminoTypes, SigningStargateClient} from "@cosmjs/stargate";
import {CosmWasmClient} from "@cosmjs/cosmwasm-stargate";
import {Registry} from "@cosmjs/proto-signing";
import {cosmosAminoConverters, cosmosProtoRegistry, cosmwasmAminoConverters, cosmwasmProtoRegistry} from "osmojs";
// import {fromUtf8} from "@cosmjs/encoding";

export default createStore({
  /**
   * State containing primary Keys of the Vue store. Persisting of state objects.
   */
  state: {
    user: {
      signer: null,
      querier: null,
      address: null,
      balance: null,
      allocations: []
    },

    // data: null,

    gameConfig: null,
    gameState: null,
    pots: [],
    winningPots: [],
    bidRange: {
      min_bid: null,
      max_bid: null,
    },
    reallocationFeePool: null,

    utils: {
      selectedPot: null
    }
  },

  getters: {
    userSigner(state) {
      return state.user.signer;
    },

    userQuerier(state) {
      return state.user.querier;
    },

    userAddress(state) {
      return state.user.address;
    },

    userBalance(state) {
      return state.user.balance;
    },

    playerAllocations(state) {
      return state.user.allocations
    },

    gameConfig(state) {
      return state.gameConfig;
    },

    gameState(state) {
      return state.gameState;
    },

    pots(state) {
      return state.pots;
    },

    winningPots(state) {
      return state.winningPots;
    },

    minBid(state) {
      return state.bidRange.min_bid;
    },

    maxBid(state) {
      return state.bidRange.max_bid;
    },

    reallocationFeePool(state) {
      return state.reallocationFeePool;
    },

    utils(state) {
      return state.utils
    }
  },

  mutations: {
    // setAllContractState(state, data) {
    //   state.data = data
    // },

    setUserSigner(state, signer) {
      state.user.signer = signer;
    },

    setUserQuerier(state, querier) {
      state.user.querier = querier;
    },

    setUserAddress(state, address) {
      state.user.address = address;
    },

    setUserBalance(state, balance) {
      state.user.balance = balance;
    },

    setPlayerAllocations(state, allocations) {
      state.user.allocations = allocations;
    },

    // Game

    setGameConfig(state, gameConfig) {
      state.gameConfig = gameConfig;
    },

    setGameState(state, gameState) {
      state.gameState = gameState;
    },

    setPots(state, pots) {
      state.pots = pots;
    },

    setWinningPots(state, winningPots) {
      state.winningPots = winningPots;
    },

    setBidRange(state, {min_bid, max_bid}) {
      state.bidRange.min_bid = Number(min_bid);
      state.bidRange.max_bid = Number(max_bid);
    },

    setReallocationFeePool(state, reallocationFeePool) {
      state.reallocationFeePool = reallocationFeePool;
    },

    // Utils
    setSelectedPot(state, potId) {
      state.utils.selectedPot = Number(potId);
    }
  },

  actions: {
    async initUser({commit}) {
      const chainId = process.env.VUE_APP_CHAIN_ID;

      if (!window.keplr) {
        alert("Please install keplr extension");
      } else {
        await window.keplr.enable(chainId);

        const offlineSigner = window.getOfflineSigner(chainId);
        const accounts = await offlineSigner.getAccounts();
        commit("setUserAddress", accounts[0].address);

        const protoRegistry = [
          ...cosmosProtoRegistry,
          ...cosmwasmProtoRegistry,
        ];
        const aminoConverters = {
          ...cosmosAminoConverters,
          ...cosmwasmAminoConverters,
        };
        const registry = new Registry(protoRegistry);
        const aminoTypes = new AminoTypes(aminoConverters);

        const signingClient = await SigningStargateClient.connectWithSigner(
          process.env.VUE_APP_RPC_EXECUTE,
          offlineSigner,
          // other options
          {
            registry,
            aminoTypes
          }
        );
        commit("setUserSigner", signingClient);
      }

      // Initialize CosmWasmClient for querying
      const queryClient = await CosmWasmClient.connect(process.env.VUE_APP_RPC_QUERY);
      commit("setUserQuerier", queryClient);
    },

    // async fetchAllContractState({state, commit}) {
    //   if (!state.user.address || !state.user.querier) {
    //     console.error("Address or Querier is not initialized");
    //     return;
    //   }
    //   console.log(state.user.querier)
    //
    //   // Use CosmWasmClient for the query
    //   let data = await state.user.querier.queryClient.wasm.getAllContractState(
    //     process.env.VUE_APP_CONTRACT
    //   );
    //   data.models.map(item => {
    //     item.key = fromUtf8(item.key)
    //     item.value = JSON.parse(fromUtf8(item.value))
    //   })
    //   console.log(data.models)
    //   // GameConfig should be queried once
    //   // PlayerAllocations only after a player did something
    //   commit("setAllContractState", data.models);
    // },

    async fetchPlayerData({state, commit}) {
      if (!state.user.address || !state.user.querier) {
        console.error("Address or Querier is not initialized");
        return;
      }

      // Use CosmWasmClient for the query
      const balance = await state.user.querier.queryClient.bank.balance(
        state.user.address,
        process.env.VUE_APP_GAME_DENOM
      );

      commit("setUserBalance", Number(balance.amount) / 1000000);

      // Use CosmWasmClient for the query
      const queryResponse = await state.user.querier.queryContractSmart(
        process.env.VUE_APP_CONTRACT,
        {
          query_player_allocations: {
            address: state.user.address
          }
        }
      );

      // Filter out allocations where the amount is "0"
      const filteredAllocations = queryResponse.allocations.allocations.filter(allocation => allocation.amount !== "0");

      commit("setPlayerAllocations", filteredAllocations);
    },

    async fetchGameConfig({state, commit}) {
      if (!state.user.querier) {
        console.error("Querier is not initialized");
        return;
      }

      // Use CosmWasmClient for the query
      const data = await state.user.querier.queryContractSmart(
        process.env.VUE_APP_CONTRACT,
        {query_game_config: {}}
      );
      commit("setGameConfig", data.config);
    },

    async fetchGameState({state, commit}) {
      if (!state.user.querier) {
        console.error("Querier is not initialized");
        return;
      }

      // Fetch the game state from the contract
      // Replace this with a call to your contract's query interface
      const data = await state.user.querier.queryContractSmart(
        process.env.VUE_APP_CONTRACT,
        {query_game_state: {}}
      );
      commit("setGameState", data.state);
    },

    async fetchPots({state, commit}) {
      if (!state.user.querier) {
        console.error("Querier is not initialized");
        return;
      }

      // Fetch the game state from the contract
      // Replace this with a call to your contract's query interface
      const data = await state.user.querier.queryContractSmart(
        process.env.VUE_APP_CONTRACT,
        {query_pots_state: {}}
      );
      commit("setPots", data.pots);
    },

    async fetchWinningPots({state, commit}) {
      if (!state.user.querier) {
        console.error("Querier is not initialized");
        return;
      }

      // Fetch the game state from the contract
      // Replace this with a call to your contract's query interface
      const data = await state.user.querier.queryContractSmart(
        process.env.VUE_APP_CONTRACT,
        {query_winning_pots: {}}
      );
      commit("setWinningPots", data.pots);
    },

    async fetchBidRange({state, commit}) {
      if (!state.user.querier) {
        console.error("Querier is not initialized");
        return;
      }

      // Fetch the bid range from the contract
      // Replace this with a call to your contract's query interface
      const data = await state.user.querier.queryContractSmart(
        process.env.VUE_APP_CONTRACT,
        {query_bid_range: {}}
      );
      commit("setBidRange", {min_bid: Number(data.min_bid), max_bid: Number(data.max_bid)});
    },

    async fetchReallocationFeePool({state, commit}) {
      if (!state.user.querier) {
        console.error("Querier is not initialized");
        return;
      }

      // Fetch the reallocation fee pool from the contract
      // Replace this with a call to your contract's query interface
      const data = await state.user.querier.queryContractSmart(
        process.env.VUE_APP_CONTRACT,
        {query_reallocation_fee_pool: {}}
      );
      commit("setReallocationFeePool", data.reallocation_fee_pool);
    },
  },

  modules: {},
});
