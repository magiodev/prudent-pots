import mxToast from "@/mixin/toast";
import {mapGetters} from "vuex";
import {toBase64, toUtf8} from "@cosmjs/encoding";

const mxChain = {
  mixins: [mxToast],

  computed: {
    ...mapGetters(['getUserSigner', 'getUserQuerier', 'getUserWalletAddress'])
  },

  methods: {
    getVoteMsg(contractAddress, nonce, executionPayload) {
      // Remove trigger_price from the new object if it exists
      if (executionPayload.trigger_price) delete executionPayload.trigger_price;

      return {
        wasm: {
          execute: {
            contract_addr: process.env.VUE_APP_CONTRACT_MIDDLEWARE,
            msg: toBase64(toUtf8(JSON.stringify({
              range_msg: {
                submit_new_range: {
                  new_range: {
                    cl_vault_address: contractAddress,
                    lower_price: executionPayload.lower_price.toString(),
                    upper_price: executionPayload.upper_price.toString(),
                  }
                }
              }
            }))),
            funds: [],
          }
        }
      }
    }
  }
}

export default mxChain
