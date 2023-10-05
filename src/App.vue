<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup

import PopCat from "./components/PopCat.vue";
import { invoke } from "@tauri-apps/api/tauri";
import { ref } from "vue";

const permissionsMsg = ref("检测权限中...");
invoke("cmd_has_permissions_and_release_files").then(b => {
  permissionsMsg.value = b ? "权限正常" : "无管理员权限";
})

</script>

<template>
  <div v-if="permissionsMsg == '权限正常'" class="row">
    <div class="container">
      <h1>请选择图标样式 ! </h1>
      <p>Plase select icon !</p>
      <PopCat />
    </div>
  </div>

  <div class="container">
    <div v-if="permissionsMsg == '无管理员权限'" class="row">
      <el-col :sm="12" :lg="6">
        <el-result icon="error" title='无管理员权限' sub-title="请右键->以管理员身份运行">
        </el-result>
      </el-col>
    </div>
  </div>
</template>
