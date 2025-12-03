<template>
  <el-container style="height: 100vh;">
    <!-- 侧边栏 -->
    <el-aside width="200px">
      <el-menu
        :default-active="$route.path"
        class="el-menu-vertical-demo"
        @select="handleSelect"
        router
      >
        <el-menu-item index="/">
          <el-icon><HomeFilled /></el-icon>
          <span>主页</span>
        </el-menu-item>
        <el-menu-item index="/workbench">
          <el-icon><Setting /></el-icon>
          <span>工作台</span>
        </el-menu-item>
        <el-menu-item index="/marker">
          <el-icon><EditPen /></el-icon>
          <span>贴图标记</span>
        </el-menu-item>
      </el-menu>
    </el-aside>

    <!-- 主内容区域 -->
    <el-container>
      <el-main>
        <router-view />
      </el-main>
    </el-container>
  </el-container>
</template>

<script setup lang="ts">
import { useRouter } from 'vue-router'
import { HomeFilled, Setting, EditPen } from '@element-plus/icons-vue'

const router = useRouter()

const handleSelect = (key: string) => {
  router.push(key)
}
</script>

<style scoped>
.el-menu-vertical-demo {
  height: 100%;
}
</style>