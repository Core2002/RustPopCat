<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";

async function make_pop_cat(operate: string) {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  await invoke("cmd_pop_cat", { operate: operate });
}
</script>

<template>
  <div class="row">
    <div @click="make_pop_cat('1')">
      <img src="../assets/popcat.png" class="logo popcat" alt="popcat" />
    </div>
    <div>
      <img src="../assets/rust.png" class="logo rust" alt="rust" />
    </div>
    <div @click="make_pop_cat('0')">
      <img src="../assets/recyclebin.png" class="logo tauri" alt="Tauri logo" />
    </div>
  </div>
</template>

<style scoped>
.logo.popcat:hover {
  filter: drop-shadow(0 0 2em #fdba36);
}

</style>
