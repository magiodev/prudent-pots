const mxPot = {
  methods: {
    getPotName(potId) {
      const potNames = {
        1: 'Lowest',
        2: 'Even',
        3: 'Median',
        4: 'Odd',
        5: 'Highest'
      };
      return potNames[potId] || 'Unknown Pot';
    },

    getPotDescription(potId) {
      const potDescriptions = {
        1: 'This pot wins if it has the fewest tokens.',
        2: 'This pot wins if it has an even number of tokens.',
        3: 'This pot wins if it has the median number of tokens.',
        4: 'This pot wins if it has an odd number of tokens.',
        5: 'This pot wins if it has the most tokens.',
      };
      return potDescriptions[potId] || 'No description available.';
    },
  }
}

export default mxPot
