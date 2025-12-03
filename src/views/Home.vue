<template>
  <div class="home">
    <h1>主页</h1>
    <p>欢迎使用 SSMT4 应用！</p>
    <el-button type="primary" @click="startGame">开始游戏</el-button>
  </div>
</template>

<script setup lang="ts">
// 主页组件逻辑
const startGame = () => {
  // 这里可以添加开始游戏的逻辑，比如导航到游戏页面或调用 Rust 函数
  console.log('开始游戏！')
}
</script>

<style scoped>
.home {
  padding: 20px;
}
</style>