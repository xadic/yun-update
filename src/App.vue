<script setup>
import { invoke } from '@tauri-apps/api';
import HelloWorld from './components/HelloWorld.vue'
import { emit } from '@tauri-apps/api/event'

function yun_update() {
  console.log("invoke yun_update")
  emit('update', { message: 'Hello from Vue' })
}

function backup() {
  console.log("invoke backup")
  invoke('backup')
}

</script>

<template>
  <div>
    <a href="https://vitejs.dev" target="_blank">
      <img src="/vite.svg" class="logo" alt="Vite logo" />
    </a>
    <a href="https://vuejs.org/" target="_blank">
      <img src="./assets/vue.svg" class="logo vue" alt="Vue logo" />
    </a>

    <div>
      <button @click="yun_update">一键升级</button>
      <button @click="backup">备份数据库</button>
    </div>
  </div>
  <HelloWorld msg="Vite + Vue" />
</template>

<style scoped>
.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: filter 300ms;
}

.logo:hover {
  filter: drop-shadow(0 0 2em #646cffaa);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #42b883aa);
}
</style>
