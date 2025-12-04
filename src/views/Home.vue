<template>
  <div class="home">
    <h1>主页</h1>
    <p>欢迎使用 SSMT4 应用！</p>

    <el-row style="margin: 5px 5px 5px 5px;">
      <el-col :span="24">
        <el-text class="mx-1" type="primary" style="display: block; text-align: left;">3Dmigoto路径</el-text>
        <div style="display: flex; align-items: center; margin-top: 5px;">
          <el-input v-model="path_3dmigoto_folder" placeholder="请输入3Dmigoto路径" style="flex: 1;"></el-input>
          <el-button type="primary" style="margin-left: 10px;" @click="selectPath3dmigoto">选择路径</el-button>
        </div>
      </el-col>
    </el-row>

    <el-row style="margin: 5px 5px 5px 5px;">
      <el-col :span="24">
        <el-text class="mx-1" type="primary" style="display: block; text-align: left;">进程路径</el-text>
        <div style="display: flex; align-items: center; margin-top: 5px;">
          <el-input v-model="path_process_folder" placeholder="请输入进程路径（文件或可执行）" style="flex: 1;"></el-input>
          <el-button type="primary" style="margin-left: 10px;" @click="selectPathProcess">选择路径</el-button>
        </div>
      </el-col>
    </el-row>

    <el-row style="margin: 5px 5px 5px 5px;">
      <el-col :span="24">
        <el-text class="mx-1" type="primary" style="display: block; text-align: left;">启动路径</el-text>
        <div style="display: flex; align-items: center; margin-top: 5px;">
          <el-input v-model="path_startup_folder" placeholder="请输入启动路径（文件或可执行）" style="flex: 1;"></el-input>
          <el-button type="primary" style="margin-left: 10px;" @click="selectPathStartup">选择路径</el-button>
        </div>
      </el-col>
    </el-row>

    <el-row style="margin: 5px 5px 5px 5px;">
      <el-col :span="24">
        <el-text class="mx-1" type="primary" style="display: block; text-align: left;">启动参数</el-text>
        <div style="display: flex; align-items: center; margin-top: 5px;">
          <el-input v-model="startup_parameters" placeholder="请输入启动参数" style="flex: 1;"></el-input>
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
const path_3dmigoto_folder = ref('')
const path_process_folder = ref('')
const path_startup_folder = ref('')
const startup_parameters = ref('')

const startGame = () => {
  // 这里可以添加开始游戏的逻辑，比如导航到游戏页面或调用 Rust 函数
  console.log('开始游戏！')
}

// 选择 3Dmigoto 所在文件夹（目录选择）
const selectPath3dmigoto = async () => {
  try {
    const selected = await open({ directory: true })
    if (selected) {
      path_3dmigoto_folder.value = selected as string
    }
  } catch (error) {
    console.error('选择3Dmigoto路径失败:', error)
  }
}

// 选择进程路径（文件选择），支持选择单个文件
const selectPathProcess = async () => {
  try {
    // 不设置 directory，默认打开文件选择
    const selected = await open({ multiple: false })
    if (selected) {
      path_process_folder.value = selected as string
    }
  } catch (error) {
    console.error('选择进程路径失败:', error)
  }
}

// 选择启动路径（文件选择）
const selectPathStartup = async () => {
  try {
    const selected = await open({ multiple: false })
    if (selected) {
      path_startup_folder.value = selected as string
    }
  } catch (error) {
    console.error('选择启动路径失败:', error)
  }
}
</script>

<style scoped>
.home {
  padding: 20px;
}
</style>