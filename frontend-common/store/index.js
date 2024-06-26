import {createStore} from "vuex";
import {AminoTypes} from "@cosmjs/stargate";
import {CosmWasmClient, SigningCosmWasmClient} from "@cosmjs/cosmwasm-stargate";
import {Registry} from "@cosmjs/proto-signing";
import {cosmosAminoConverters, cosmosProtoRegistry, cosmwasmAminoConverters, cosmwasmProtoRegistry} from "osmojs";
import axios from "axios";
import mxChain from "../mixin/chain";

const mxChainUtils = {
  methods: mxChain.methods
};

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
      cw721balance: [],
      // allocations: [],
      reallocations: null
    },

    gameConfig: null,
    gameState: null,
    gameActivity: null,
    gameActivitySelectedRound: null,

    pots: [],
    winningPots: [],
    bidRange: {
      min_bid: null,
      max_bid: null,
    },
    reallocationFeePool: null,

    allPlayersAllocations: null,

    utils: {
      selectedPot: null
    },

    raffle: null,
    raffleWinner: null,
    raffleDenomSplit: null
  },

  getters: {
    userSigner(state) {
      return state.user.signer;
    },

    // userQuerier(state) {
    //   return state.user.querier;
    // },

    userAddress(state) {
      return state.user.address;
    },

    userBalance(state) {
      return state.user.balance;
    },

    userCw721Balance(state) {
      return state.user.cw721balance
    },

    playerAllocations(state) {
      // This is not a state per say, we want to match the addy by userAddy in the mega list
      const foundAllocations = state.allPlayersAllocations.find(a => a[0] === state.user.address)
      return foundAllocations ? foundAllocations[1].filter(allocation => allocation.amount !== "0") : []
    },

    playerReallocations(state) {
      return state.user.reallocations
    },

    gameConfig(state) {
      return state.gameConfig;
    },

    gameState(state) {
      return state.gameState;
    },

    gameActivity(state) {
      return state.gameActivity;
    },

    gameActivitySelectedRound(state) {
      return state.gameActivitySelectedRound;
    },

    allPlayersAllocations(state) {
      return state.allPlayersAllocations;
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
    },

    raffle(state) {
      return state.raffle
    },

    raffleWinner(state) {
      return state.raffleWinner
    },

    raffleDenomSplit(state) {
      return state.raffleDenomSplit
    },
  },

  mutations: {
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

    setUserCw721Balance(state, balance) {
      state.user.cw721balance = balance;
    },

    setPlayerReallocations(state, reallocations) {
      state.user.reallocations = reallocations;
    },

    // Game

    setGameConfig(state, gameConfig) {
      state.gameConfig = gameConfig;
    },

    setGameState(state, gameState) {
      state.gameState = gameState;
    },

    setGameActivity(state, gameActivity) {
      state.gameActivity = gameActivity;
    },

    setGameActivitySelectedRound(state, selectedRound) {
      state.gameActivitySelectedRound = selectedRound;
    },

    setAllPlayersAllocations(state, allPlayersAllocations) {
      state.allPlayersAllocations = allPlayersAllocations;
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
    },

    // Raffle
    setRaffle(state, raffle) {
      state.raffle = raffle;
    },

    setRaffleWinner(state, raffleWinner) {
      state.raffleWinner = raffleWinner;
    },

    setRaffleDenomSplit(state, raffleDenomSplit) {
      state.raffleDenomSplit = raffleDenomSplit;
    },
  },

  actions: {
    async initUser({commit}) {
      const chainId = process.env.VUE_APP_CHAIN_ID;

      if (!window.keplr) {
        alert("Please install keplr extension")
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

        const signingClient = await SigningCosmWasmClient.connectWithSigner(
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

    async fetchPlayerData({state, commit}) {
      if (!state.user.address || !state.user.querier) {
        console.error("Address or Querier is not initialized");
        return;
      }

      // Balance
      const balance = await state.user.querier.queryClient.bank.balance(
        state.user.address,
        state.gameConfig.game_denom
      );
      commit("setUserBalance", mxChainUtils.methods.displayAmount(Number(balance.amount)));

      // Player Reallocations
      const reallocationsResponse = await state.user.querier.queryContractSmart(
        process.env.VUE_APP_CONTRACT,
        {
          player_reallocations: {
            address: state.user.address
          }
        }
      );
      commit("setPlayerReallocations", reallocationsResponse.reallocations);
    },

    async fetchGameConfig({state, commit}) {
      if (!state.user.querier) {
        console.error("Querier is not initialized");
        return;
      }

      // Use CosmWasmClient for the query
      const data = await state.user.querier.queryContractSmart(
        process.env.VUE_APP_CONTRACT,
        {game_config: {}}
      );
      commit("setGameConfig", data.config);
    },

    async fetchGameState({state, commit}) {
      if (!state.user.querier) {
        console.error("Querier is not initialized");
        return;
      }

      const data = await state.user.querier.queryContractSmart(
        process.env.VUE_APP_CONTRACT,
        {game_state: {}}
      );
      commit("setGameState", data.state);
    },

    async fetchGameActivity({state, commit}) {
      if (!state.user.querier) {
        console.error("Querier is not initialized");
        return;
      }

      let groupedByRoundCount = {};

      // Fetch all rounds, that's why round_count is commented out
      const data = await state.user.querier.searchTx([
        {key: "wasm._contract_address", value: process.env.VUE_APP_CONTRACT},
        {key: "wasm.round_count", value: state.gameActivitySelectedRound.toString()}
      ]);

      data.forEach(item => {
        // Flatten events and check for round_count
        const roundEvents = item.events.flatMap(event => event.attributes)
          .filter(attr => attr.key === "round_count");

        if (roundEvents.length > 0) {
          const roundCount = roundEvents[0].value;
          if (!groupedByRoundCount[roundCount]) {
            groupedByRoundCount[roundCount] = {round_count: roundCount, transactions: []};
          }
          groupedByRoundCount[roundCount].transactions.push({
            transactionHash: item.hash,
            events: item.events,
            height: item.height
          });
        }
      });

      // Sort by round_count descending and restructure data for Vuex
      const sortedGroupedByRoundCount = Object.keys(groupedByRoundCount)
        .sort((a, b) => b - a)
        .reduce((acc, key) => {
          acc.push(groupedByRoundCount[key]); // Push the whole object including round_count and transactions
          return acc;
        }, []);

      commit("setGameActivity", sortedGroupedByRoundCount);
    },

    async fetchAllPlayersAllocations({state, commit}) {
      if (!state.user.querier) {
        console.error("Querier is not initialized");
        return;
      }
      const data = await state.user.querier.queryContractSmart(
        process.env.VUE_APP_CONTRACT,
        {all_players_allocations: {}}
      );
      commit("setAllPlayersAllocations", data.allocations);
    },

    async fetchPots({state, commit}) {
      if (!state.user.querier) {
        console.error("Querier is not initialized");
        return;
      }
      const data = await state.user.querier.queryContractSmart(
        process.env.VUE_APP_CONTRACT,
        {pots_state: {}}
      );
      commit("setPots", data.pots);
    },

    async fetchWinningPots({state, commit}) {
      if (!state.user.querier) {
        console.error("Querier is not initialized");
        return;
      }
      const data = await state.user.querier.queryContractSmart(
        process.env.VUE_APP_CONTRACT,
        {winning_pots: {}}
      );
      commit("setWinningPots", data.pots);
    },

    async fetchBidRange({state, commit}) {
      if (!state.user.querier) {
        console.error("Querier is not initialized");
        return;
      }
      const data = await state.user.querier.queryContractSmart(
        process.env.VUE_APP_CONTRACT,
        {bid_range: {address: state.user.address}}
      );
      commit("setBidRange", {min_bid: Number(data.min_bid), max_bid: Number(data.max_bid)});
    },

    async fetchReallocationFeePool({state, commit}) {
      if (!state.user.querier) {
        console.error("Querier is not initialized");
        return;
      }
      const data = await state.user.querier.queryContractSmart(
        process.env.VUE_APP_CONTRACT,
        {reallocation_fee_pool: {}}
      );
      commit("setReallocationFeePool", data.reallocation_fee_pool);
    },

    // Raffle

    async fetchRaffle({state, commit}) {
      if (!state.user.querier) {
        console.error("Querier is not initialized");
        return;
      }

      let data = await state.user.querier.queryContractSmart(
        process.env.VUE_APP_CONTRACT,
        {raffle: {}}
      );

      if (data.raffle.cw721_token_id) {
        const metadata = (await axios.get(`${process.env.VUE_APP_NFT_BASE_URL}/${data.raffle.cw721_token_id}.json`)).data

        // {NFT_NAME} id fix: Extract the real token id from .name
        const parts = metadata.name.split('#');
        const id = parts.length > 1 ? parts[1] : null;

        // TODO_FUTURE: create env var
        const imageUrl = `https://mintdao-ipfs.b-cdn.net/ipfs/${metadata.image.replace('ipfs://', '')}`

        data.raffle.nft = {id, metadata, imageUrl}
      }

      commit("setRaffle", data.raffle);
    },

    async fetchRaffleWinner({state, commit}) {
      if (!state.user.querier) {
        console.error("Querier is not initialized");
        return;
      }

      const data = await state.user.querier.queryContractSmart(
        process.env.VUE_APP_CONTRACT,
        {raffle_winner: {}}
      );
      commit("setRaffleWinner", data.raffle_winner);
    },

    async fetchRaffleDenomSplit({state, commit}) {
      if (!state.user.querier) {
        console.error("Querier is not initialized");
        return;
      }

      const data = await state.user.querier.queryContractSmart(
        process.env.VUE_APP_CONTRACT,
        {raffle_denom_split: {}}
      );
      commit("setRaffleDenomSplit", data.raffle_denom_split);
    },

    // CW721

    async fetchCw721Tokens({state, commit}) {
      if (!state.user.querier) {
        console.error("Querier is not initialized");
        return;
      }

      if (process.env.VUE_APP_CONTRACT_CW721) {
        const data = await state.user.querier.queryContractSmart(
          process.env.VUE_APP_CONTRACT_CW721,
          {
            tokens: {
              owner: state.user.address
            }
          }
        );
        commit("setUserCw721Balance", data.tokens);
      }
    },
  },

  modules: {},
});
