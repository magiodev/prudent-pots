<template>
  <div class="audio-player-component">
    <select v-model="currentTrack" @change="stopAudio" class="px-3 mb-2 bg-transparent">
      <option v-for="(track, index) in tracks" :key="index" :value="track.src">{{ track.name }}</option>
    </select>

    <audio ref="audio" controls class="w-100" :src="currentTrack" @play="isPlaying = true" @pause="isPlaying = false"
           @ended="stopAudio">
      Audio not supported
    </audio>
  </div>
</template>

<script>
import songAmbient from "@/assets/soundtracks/Mad_Scientists_Ambient_electric.m4a";
import songChorus from "@/assets/soundtracks/Mad_Scientists_Chorus.m4a";
import songEDM from "@/assets/soundtracks/Mad_Scientists_EDM.m4a";
import songHeavyMetal from "@/assets/soundtracks/Mad_Scientists_Heavy_Metal.m4a";
import songRockLiveConcert from "@/assets/soundtracks/Mad_Scientists_Rock_Live_Concert.m4a";
import songRaggae from "@/assets/soundtracks/Mad_Scientists_Raggae.m4a";
import songTrap from "@/assets/soundtracks/Mad_Scientists_Trap.m4a";

export default {
  name: "AudioPlayerComponent",

  data() {
    return {
      isPlaying: false,
      tracks: [
        {name: "Ambient Electric", src: songAmbient},
        {name: "Chorus Harmony", src: songChorus},
        {name: "EDM", src: songEDM},
        {name: "Heavy Metal", src: songHeavyMetal},
        {name: "Rock Live Concert", src: songRockLiveConcert},
        {name: "Reggae", src: songRaggae},
        {name: "Trap", src: songTrap}
      ],
      currentTrack: null
    };
  },

  mounted() {
    this.selectRandomTrack();
  },

  methods: {
    stopAudio() {
      const audio = this.$refs.audio;
      audio.pause();
      audio.currentTime = 0;
      this.isPlaying = false;
    },
    selectRandomTrack() {
      const randomIndex = Math.floor(Math.random() * this.tracks.length);
      this.currentTrack = this.tracks[randomIndex].src;
    }
  }
}
</script>

<style lang="scss" scoped>
@import "@/assets/style";

.audio-player-component {
  select {
    outline: 0;
    width: 100%;
    border: 0;
    border-bottom: 1px solid $pp-color-5;
    color: $pp-color-5;
  }
}
</style>