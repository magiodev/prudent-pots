import {useToast} from 'vue-toastification'

const mxToast = {
  data() {
    return {
      toast: useToast()
    }
  },
}

export default mxToast
