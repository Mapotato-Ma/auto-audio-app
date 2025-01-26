<template>
  <main class="container">
    <h1>文本转语音</h1>
    <h1></h1>
    <div class="speed-control">
      <button
        :class="{ active: button.value === speed }"
        @click="setSpeed(button.value)"
        v-for="button in speedControls"
        :key="button.name"
      >
        {{ button.name }}
      </button>
    </div>
    <select name="voices" id="voices" v-model="currentVoice">
      <option :value="index" v-for="(voice, index) in voices" :key="voice">{{ voice }}</option>
    </select>
    <div class="volume-control">
      <input type="range" min="0" max="1" step="0.1" v-model="volume" />
      <span>{{ currentVolumeIcon() }}</span>
    </div>
    <form class="row" @submit.prevent>
      <input :maxlength="10" v-model="textToSpeak" placeholder="请输入朗读文本..." />
      <button type="submit" @click="speak">朗读</button>
      <span class="tips" v-show="isSpeaking">朗读进行中，无法操作~~~</span>
    </form>
  </main>
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';

const textToSpeak = ref('');
const speed = ref(1);
const isSpeaking = ref(false);
const voices = ref([]);
const currentVoice = ref(0);
const volume = ref(1);
const numberOfVolume = computed(() => Number(volume.value));
const currentVolumeIcon = () => {
  if (numberOfVolume.value === 0) return '🔇';
  if (numberOfVolume.value < 0.5) return '🔈';
  if (numberOfVolume.value < 1) return '🔉';
  return '🔊';
};

const speedControls = [
  { name: '0.5x', value: 0.5 },
  { name: '0.75x', value: 0.75 },
  { name: '1x', value: 1 },
  { name: '2x', value: 2 },
  { name: '3x', value: 3 },
];

async function speak() {
  isSpeaking.value = true;
  await invoke('speech', {
    text: textToSpeak.value,
    voiceIndex: currentVoice.value,
    rate: speed.value,
    volume: numberOfVolume.value,
  });
  isSpeaking.value = false;
}

async function getVoices() {
  return (await invoke<[]>('get_voices', {})) ?? [];
}

const setSpeed = (value: number) => {
  speed.value = value;
};

onMounted(async () => {
  voices.value = await getVoices();
});
</script>

<style>
html,
body {
  height: 100%;
}

body {
  display: grid;
  place-content: center;
}
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  height: 100%;
  padding: 0;
  display: flex;
  gap: 1em;
  flex-direction: column;
  justify-content: center;
  text-align: center;
  .speed-control {
    display: flex;
    justify-content: center;
    align-items: center;
    gap: 5px;
    button {
      height: 30px;
      font-size: 12px;
    }
  }
  .volume-control {
    display: flex;
    justify-content: center;
    align-items: center;
    gap: 5px;
  }
}

.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: 0.75s;
}

.row {
  position: relative;
  display: flex;
  justify-content: center;
  gap: 10px;
  .tips {
    position: absolute;
    bottom: -100%;
    font-size: xx-small;
    opacity: 0.6;
  }
}

a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover {
  color: #535bf2;
}

h1 {
  text-align: center;
}

input:not([type='range']),
button,
select {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

input[type='range'] {
  flex: 1;
  cursor: pointer;
}

select {
  width: 100%;
  background-color: #0f0f0f;
  color: #ffffff;
  &:focus {
    outline: none;
  }
}

button {
  cursor: pointer;
  &.active {
    background: #396cd8;
  }
}

button:hover {
  border-color: #396cd8;
}
button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
button {
  outline: none;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }
}
</style>
