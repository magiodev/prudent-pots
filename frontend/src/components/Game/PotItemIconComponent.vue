<template>
  <div class="pot-item-icon-component">
    <!-- For potId 1 -->
    <img v-if="potId === 1 && isWinning" :src="iconLowestGreen" />
    <img v-else-if="potId === 1 && !isWinning" :src="iconLowestRed" />

    <!-- For potId 2 (assuming it's "Even") -->
    <img v-else-if="potId === 2 && isWinning" :src="iconEvenGreen" />
    <img v-else-if="potId === 2 && !isWinning" :src="iconEvenRed" />

    <!-- For potId 3 (assuming it's "Median") -->
    <img v-else-if="potId === 3 && isWinning" :src="iconMedianGreen" />
    <img v-else-if="potId === 3 && !isWinning" :src="iconMedianRed" />

    <!-- For potId 4 (assuming it's "Odd") -->
    <img v-else-if="potId === 4 && isWinning" :src="iconOddGreen" />
    <img v-else-if="potId === 4 && !isWinning" :src="iconOddRed" />

    <!-- For potId 5 (assuming it's "Highest") -->
    <img v-else-if="potId === 5 && isWinning" :src="iconHighestGreen" />
    <img v-else-if="potId === 5 && !isWinning" :src="iconHighestRed" />
  </div>
</template>

<script>
// 1. Lowest
import iconLowestGreen from "@/assets/icons/pot-icon-lowest-green.png"
import iconLowestRed from "@/assets/icons/pot-icon-lowest-red.png"
// 2. Even
import iconEvenGreen from "@/assets/icons/pot-icon-even-green.png"
import iconEvenRed from "@/assets/icons/pot-icon-even-red.png"
// 3. Median
import iconMedianGreen from "@/assets/icons/pot-icon-median-green.png"
import iconMedianRed from "@/assets/icons/pot-icon-median-red.png"
// 4. Odd
import iconOddGreen from "@/assets/icons/pot-icon-odd-green.png"
import iconOddRed from "@/assets/icons/pot-icon-odd-red.png"
// 5. Highest
import iconHighestGreen from "@/assets/icons/pot-icon-highest-green.png"
import iconHighestRed from "@/assets/icons/pot-icon-highest-red.png"

export default {
  name: "PotItemIconComponent",

  props: {
    potId: {
      type: Number,
      required: true
    },
    isWinning: {
      type: Boolean,
      required: true
    }
  },

  data() {
    return {
      iconLowestRed,
      iconLowestGreen,
      iconEvenGreen,
      iconEvenRed,
      iconMedianGreen,
      iconMedianRed,
      iconOddGreen,
      iconOddRed,
      iconHighestGreen,
      iconHighestRed
    }
  }
}
</script>

<style scoped lang="scss">
.pot-item-icon-component {
  display: inline-block;
  vertical-align: super;
  img {
    height: 36px;
    width: auto;
    @media (max-width: 767px) {
      height: 28px;
    }
  }
}
</style>