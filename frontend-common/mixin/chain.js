import mxToast from "./toast";
import {mapGetters} from "vuex";
import {toUtf8} from "@cosmjs/encoding";

const mxChain = {
  mixins: [mxToast],

  computed: {
    ...mapGetters(['userSigner', 'userAddress', 'gameConfig']),
  },

  methods: {
    async suggestChain() {
      await window.keplr.experimentalSuggestChain(JSON.parse(process.env.VUE_APP_CHAIN_INFO));
    },

    async allocateTokens(potId, amount) {
      /** @type {import("@cosmjs/proto-signing").EncodeObject} */
      const msg = {
        typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
        value: {
          sender: this.userAddress,
          contract: process.env.VUE_APP_CONTRACT,
          msg: toUtf8(JSON.stringify({
            allocate_tokens: {
              pot_id: Number(potId)
            }
          })),
          funds: [
            {denom: this.gameConfig.game_denom, amount: Math.ceil(amount).toString()}
          ],
        }
      }
      return this._submitTx(msg)
    },

    async reallocateTokens(fromPotId, toPotId) {
      /** @type {import("@cosmjs/proto-signing").EncodeObject} */
      const msg = {
        typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
        value: {
          sender: this.userAddress,
          contract: process.env.VUE_APP_CONTRACT,
          msg: toUtf8(JSON.stringify({
            reallocate_tokens: {
              from_pot_id: Number(fromPotId),
              to_pot_id: Number(toPotId),
            }
          })),
          funds: [],
        }
      }
      return this._submitTx(msg)
    },

    async approveCw721(tokenId) {
      /** @type {import("@cosmjs/proto-signing").EncodeObject} */
      const msg = {
        typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
        value: {
          sender: this.userAddress,
          contract: process.env.VUE_APP_CONTRACT_CW721,
          msg: toUtf8(JSON.stringify({
            approve_all: {
              token_id: tokenId.toString(),
              spender: process.env.VUE_APP_CONTRACT,
              expires: null,
            }
          })),
          funds: [],
        }
      }
      return this._submitTx(msg)
    },

    async approveAllCw721() {
      /** @type {import("@cosmjs/proto-signing").EncodeObject} */
      const msg = {
        typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
        value: {
          sender: this.userAddress,
          contract: process.env.VUE_APP_CONTRACT_CW721,
          msg: toUtf8(JSON.stringify({
            approve_all: {
              operator: process.env.VUE_APP_CONTRACT,
            }
          })),
          funds: [],
        }
      }
      return this._submitTx(msg)
    },

    async endGame(tokenContract, tokenId, denomAmount) {
      /** @type {import("@cosmjs/proto-signing").EncodeObject} */
      let msg = {
        typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
        value: {
          sender: this.userAddress,
          contract: process.env.VUE_APP_CONTRACT,
          msg: toUtf8(JSON.stringify({
            game_end: {
              raffle_cw721_token_addr: tokenContract || null,
              raffle_cw721_token_id: tokenId || null
            }
          })),
          funds: [],
        }
      }

      // Only if there is any raffle denom amount.
      if (denomAmount) {
        msg.value.funds.push({
          denom: this.gameConfig.game_denom,
          amount: denomAmount.toString()
        })
      }

      return this._submitTx(msg)
    },

    // Utils

    displayAmount(amount, decimals = 6) {
      return (amount / 1000000).toFixed(decimals);
    },

    // PRIVATE

    async _submitTx(message) {
      const gasWanted = await this.userSigner.simulate(this.userAddress, [message])
      const fee = this._calculateFee(gasWanted);
      return await this.userSigner.signAndBroadcast(this.userAddress, [message], fee); // Return successful response
    },

    // This has implemented as: https://hackmd.io/@3DOBr1TJQ3mQAFDEO0BXgg/S1N09wpQp
    _calculateFee(gasWanted) {
      const gas = Math.ceil(gasWanted * 1.3);
      const baseFee = Number(process.env.VUE_APP_BASE_FEE)

      // baseFee * 3 doesn't seem to be necessary after v23 upgrade, but leaving that here for the moment
      const amount = Math.ceil(baseFee * gas).toString();
      return {
        amount: [{denom: this.gameConfig.game_denom, amount}],
        gas: gas.toString(),
      }
    }
  }
}

export default mxChain
