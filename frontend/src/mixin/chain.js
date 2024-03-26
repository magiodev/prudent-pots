import mxToast from "@/mixin/toast";
import {mapGetters} from "vuex";
import {toUtf8} from "@cosmjs/encoding";

const mxChain = {
  mixins: [mxToast],

  computed: {
    ...mapGetters(['userSigner', 'userAddress', 'gameConfig'])
  },

  methods: {
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
            {denom: this.gameConfig.game_denom, amount: amount.toString()}
          ],
        }
      }
      return this.submitTx(msg)
    },

    async reallocateTokens(fromPotId, toPotId, amount) {
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
          funds: [
            {denom: this.gameConfig.game_denom, amount: amount.toString()}
          ],
        }
      }
      return this.submitTx(msg)
    },

    async endGame() {
      const address = this.userAddress

      /** @type {import("@cosmjs/proto-signing").EncodeObject} */
      const msg = {
        typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
        value: {
          sender: address,
          contract: process.env.VUE_APP_CONTRACT,
          msg: toUtf8(JSON.stringify({
            game_end: {}
          })),
          funds: [],
        }
      }
      return this.submitTx(msg)
    },

    // PRIVATE

    async submitTx(message) {
      try {
        const gasWanted = await this.userSigner.simulate(this.address, [message])
        const fee = await this.calculateFee(gasWanted);
        const response = await this.userSigner.signAndBroadcast(this.address, [message], fee);

        // Log and return success immediately if transaction succeeds
        console.log(`Transaction successful: ${response.transactionHash}`)

        return response; // Return successful response
      } catch (e) {
        console.log(e)
      }
    },

    // This has implemented as: https://hackmd.io/@3DOBr1TJQ3mQAFDEO0BXgg/S1N09wpQp
    async calculateFee(gasWanted) {
      const gas = Math.ceil(gasWanted * 1.3);
      // let baseFee;
      //
      // try {
      //   const baseFeeResponse = await axios.get(process.env.MERKLE_SUBMIT_OSMOSIS_BASE_FEE!);
      //   baseFee = Number(baseFeeResponse.data?.base_fee);
      // } catch (e) {
      //   console.log(e);
      //   baseFee = 0.0025; // Fallback base fee if the request fails
      // }
      const baseFee = 0.0025

      // baseFee * 3 doesn't seem to be necessary after v23 upgrade, but leaving that here for the moment
      const amount = String(Math.ceil((baseFee * 1) * gas));
      return {
        amount: [{denom: this.gameConfig.game_denom, amount}],
        gas: gas.toString(),
      }
    }
  }
}

export default mxChain
