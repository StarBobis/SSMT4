<template>
  <div class="home">
    <h1>{{ $t('home.title') }}</h1>
    <p>{{ $t('home.welcome') }}</p>

    <el-card class="box-card" style="margin-bottom: 20px;" :header="$t('home.settings')">
      <el-row style="margin: 5px 5px 5px 5px;">
        <el-col :span="24">
          <el-text class="mx-1" type="primary" style="display: block; text-align: left;">{{ $t('home.path3dmigoto') }}</el-text>
          <div style="display: flex; align-items: center; margin-top: 5px;">
            <el-input v-model="path_3dmigoto_folder" :placeholder="$t('home.placeholder3dmigoto')" style="flex: 1;"></el-input>
            <el-button type="primary" style="margin-left: 10px;" @click="selectPath3dmigoto">{{ $t('home.selectPath') }}</el-button>
          </div>
        </el-col>
      </el-row>

      <el-row style="margin: 5px 5px 5px 5px;">
        <el-col :span="24">
          <el-text class="mx-1" type="primary" style="display: block; text-align: left;">{{ $t('home.pathProcess') }}</el-text>
          <div style="display: flex; align-items: center; margin-top: 5px;">
            <el-input v-model="path_process_folder" :placeholder="$t('home.placeholderProcess')" style="flex: 1;"></el-input>
            <el-button type="primary" style="margin-left: 10px;" @click="selectPathProcess">{{ $t('home.selectPath') }}</el-button>
          </div>
        </el-col>
      </el-row>

      <el-row style="margin: 5px 5px 5px 5px;">
        <el-col :span="24">
          <el-text class="mx-1" type="primary" style="display: block; text-align: left;">{{ $t('home.pathStartup') }}</el-text>
          <div style="display: flex; align-items: center; margin-top: 5px;">
            <el-input v-model="path_startup_folder" :placeholder="$t('home.placeholderStartup')" style="flex: 1;"></el-input>
            <el-button type="primary" style="margin-left: 10px;" @click="selectPathStartup">{{ $t('home.selectPath') }}</el-button>
          </div>
        </el-col>
      </el-row>

      <el-row style="margin: 5px 5px 5px 5px;">
        <el-col :span="24">
          <el-text class="mx-1" type="primary" style="display: block; text-align: left;">{{ $t('home.startupParams') }}</el-text>
          <div style="display: flex; align-items: center; margin-top: 5px;">
            <el-input v-model="startup_parameters" :placeholder="$t('home.placeholderParams')" style="flex: 1;"></el-input>
          </div>
        </el-col>
      </el-row>
    </el-card>

    <el-row style="margin: 5px 5px 5px 5px;">
      <el-col :span="24">
        <el-button type="primary" @click="startGame">{{ $t('home.startGame') }}</el-button>
      </el-col>
    </el-row>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { open } from '@tauri-apps/plugin-dialog'
import { invoke } from '@tauri-apps/api/core'
import { ElNotification } from 'element-plus'

// 主页组件逻辑
const path_3dmigoto_folder = ref('')
const path_process_folder = ref('')
const path_startup_folder = ref('')
const startup_parameters = ref('')

const startGame = async () => {
  // 调用后端 launch 命令启动指定程序
  if (!path_startup_folder.value) {

    ElNotification({
  title: '提示',
  message: '未设置启动路径',
  type: 'warning', // 可选：'success', 'info', 'warning', 'error'
  duration: 3000, // 自动关闭时间，单位为毫秒，0 表示不会自动关闭
      position: 'bottom-right' // 可选：'top-right', 'top-left', 'bottom-right', 'bottom-left'
})
    //await ElMessageBox.alert('未设置启动路径', '提示', { confirmButtonText: '确定' })
    return
  }

  try {
    // admin: true 强制以管理员权限启动，触发 UAC
    await invoke('launch', { path: path_startup_folder.value, args: startup_parameters.value || null, admin: true })
    console.log('已启动程序')
  } catch (e) {
    console.error('启动程序失败:', e)
  }
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