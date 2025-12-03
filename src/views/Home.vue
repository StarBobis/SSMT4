<template>
  <div class="home">
    <h1>主页</h1>
    <p>欢迎使用 SSMT4 应用！</p>

    <el-row style="margin: 5px 5px 5px 5px;">
      <el-col :span="24">
        <el-text class="mx-1" type="primary" style="display: block; text-align: left;">3Dmigoto路径</el-text>
        <div style="display: flex; align-items: center; margin-top: 5px;">
          <el-input v-model="path" placeholder="请输入3Dmigoto路径" style="flex: 1;"></el-input>
          <el-button type="primary" style="margin-left: 10px;" @click="selectPath">选择路径</el-button>
        </div>
      </el-col>
    </el-row>

    <el-row style="margin: 5px 5px 5px 5px;">
      <el-col :span="24">
        <el-button type="primary" @click="startGame">开始游戏</el-button>
      </el-col>
    </el-row>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { open } from '@tauri-apps/plugin-dialog'

// 主页组件逻辑
const path = ref('')
const startGame = () => {
  // 这里可以添加开始游戏的逻辑，比如导航到游戏页面或调用 Rust 函数
  console.log('开始游戏！')
}

const selectPath = async () => {
  try {
    const selected = await open({ directory: true })
    if (selected) {
      path.value = selected as string
    }
  } catch (error) {
    console.error('选择路径失败:', error)
  }
}
</script>

<style scoped>
.home {
  padding: 20px;
}
</style>