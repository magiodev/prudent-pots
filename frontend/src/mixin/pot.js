const mxPot = {
  methods: {
    getPotName(potId) {
      const potNames = {
        1: 'Highest',
        2: 'Median',
        3: 'Lowest',
        4: 'Even',
        5: 'Odd'
      };
      return potNames[potId] || 'Unknown Pot';
    },

    getPotDescription(potId) {
      const potDescriptions = {
        1: 'This pot wins if it has the most tokens.',
        2: 'This pot wins if it has the median number of tokens.',
        3: 'This pot wins if it has the fewest tokens.',
        4: 'This pot wins if it has an even number of tokens.',
        5: 'This pot wins if it has an odd number of tokens.',
      };
      return potDescriptions[potId] || 'No description available.';
    },
  }
}

export default mxPot
