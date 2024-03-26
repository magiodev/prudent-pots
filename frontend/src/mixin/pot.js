const mxPot = {
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
  }
}

export default mxPot
