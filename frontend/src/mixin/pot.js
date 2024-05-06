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
      // Ensure potId is an integer if it's coming as a string
      const key = parseInt(potId, 10);
      return potNames[key] || 'Unknown Pot';
    },

    getPotDescription(potId) {
      const potDescriptions = {
        1: 'This pot wins if it has the fewest tokens.',
        2: 'This pot wins if it has an even number of tokens.',
        3: 'This pot wins if it has the median number of tokens.',
        4: 'This pot wins if it has an odd number of tokens.',
        5: 'This pot wins if it has the most tokens.',
      };
      const key = parseInt(potId, 10);
      return potDescriptions[key] || 'No description available.';
    },

    formattedPotNames(potIds) {
      console.log(typeof potIds)
      if (Array.isArray(potIds)) {
        return potIds.map(potId => this.getPotName(potId)).join(', ');
      }
      return this.getPotName(potIds);
    }
  }
}

export default mxPot;
