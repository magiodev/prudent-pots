<template>
  <div class="pot-item-component col-md-2 text-center text-black">
    <!-- TODO: Highlight the pot red or green based on if its currently winning or not. -->
    <div class="pot-header">
      <h5>{{ getPotName(pot.pot_id) }}</h5>
    </div>
    <div class="pot-content">
      <p>{{ getPotDescription(pot.pot_id) }}</p>
      <!-- TODO cut decimals to 6 only if more-->
      <span class="pot-tokens p-1">{{ Number(pot.pot_state / 1000000) }} $OSMO</span>
    </div>
    <div class="pot-footer mt-3">
      <button @click="onAllocateClick">Allocate</button>
    </div>
  </div>
</template>

<script>
export default {
  name: "PotItemComponent",
  props: {
    pot: {
      type: Object,
      required: true
    }
  },
  methods: {
    getPotName(potId) {
      const potNames = {
        1: 'Median Pot',
        2: 'Highest Pot',
        3: 'Even Pot',
        4: 'Lowest Pot',
        5: 'Prime Pot'
      };
      return potNames[potId] || 'Unknown Pot';
    },

    getPotDescription(potId) {
      const potDescriptions = {
        1: 'This pot wins if it has the median number of tokens.',
        2: 'This pot wins if it has the most tokens.',
        3: 'This pot wins if it has an even number of tokens.',
        4: 'This pot wins if it has the fewest tokens.',
        5: 'This pot wins if its number of tokens is a prime number.'
      };
      return potDescriptions[potId] || 'No description available.';
    },

    // TODO: Implement this.electedPot Vuex store item
    onPotClick() {
      // Do something with userSigner
      alert("TODO!")
    }
  }
};
</script>

<style scoped>
.pot-item-component {
  border: 1px solid #ddd;
  padding: 1rem;
  margin-bottom: 1rem;
  border-radius: 0.5rem;
  background-color: #f9f9f9;
}

.pot-tokens {
  font-size: 1.2rem;
  font-weight: bold;
  margin-bottom: 1rem;
  border-radius: 0.5rem;
  background-color: #2743b2;
}

.pot-footer > button {
  background-color: #4caf50;
  padding: 0.5rem 1rem;
  border: none;
  border-radius: 0.3rem;
  cursor: pointer;
  transition: background-color 0.3s;
}

.pot-footer > button:hover {
  background-color: #45a049;
}
</style>
