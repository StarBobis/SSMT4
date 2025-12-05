<!-- 模板部分：定义页面的 HTML 结构 -->
<template>
  <!-- 最外层容器：使用 Element Plus 的布局容器 -->
  <!-- style="height: 100vh;" 设置容器高度占满整个视口高度 (100% Viewport Height) -->
  <el-container style="height: 100vh;">
    
    <!-- 侧边栏区域：宽度固定为 150px -->
    <el-aside width="150px">
      <!-- 菜单组件： -->
      <!-- :default-active="$route.path"  根据当前路由路径高亮对应的菜单项 -->
      <!-- class="el-menu-vertical-demo"  应用自定义样式类 -->
      <!-- @select="handleSelect"          当菜单项被选中时触发 handleSelect 函数 -->
      <!-- router                         开启 vue-router 模式，index 属性将作为路由路径 -->
      <el-menu
        :default-active="$route.path"
        class="el-menu-vertical-demo"
        @select="handleSelect"
        router
      >
        <!-- 菜单项 1：主页 -->
        <!-- index="/" 对应路由路径 / -->
        <el-menu-item index="/">
          <!-- 图标组件：使用 HomeFilled 图标 -->
          <el-icon><HomeFilled /></el-icon>
          <!-- 文本内容 -->
          <span>{{ $t('nav.home') }}</span>
        </el-menu-item>

        <!-- 菜单项 2：工作台 -->
        <!-- index="/workbench" 对应路由路径 /workbench -->
        <el-menu-item index="/workbench">
          <el-icon><Setting /></el-icon>
          <span>{{ $t('nav.workbench') }}</span>
        </el-menu-item>

        <!-- 菜单项 3：贴图标记 -->
        <!-- index="/marker" 对应路由路径 /marker -->
        <el-menu-item index="/marker">
          <el-icon><EditPen /></el-icon>
          <span>{{ $t('nav.marker') }}</span>
        </el-menu-item>

        <!-- 菜单项 4：设置 -->
        <!-- index="/settings" 对应路由路径 /settings -->
        <!-- style="margin-top: auto;" 利用 Flex 布局特性，将此项推到容器最底部 -->
        <el-menu-item index="/settings" style="margin-top: auto;">
          <el-icon><Setting /></el-icon>
          <span>{{ $t('nav.settings') }}</span>
        </el-menu-item>

      </el-menu>
    </el-aside>

    <!-- 右侧主内容区域容器 -->
    <el-container>
      <!-- 主要内容显示区 -->
      <el-main>
        <!-- 路由视图占位符：根据当前 URL 显示对应的页面组件（如 Home.vue, Workbench.vue 等） -->
        <router-view />
      </el-main>
    </el-container>
  </el-container>
</template>

<!-- 脚本部分：使用 setup 语法糖，lang="ts" 表示使用 TypeScript -->
<script setup lang="ts">
// 导入 Vue Router 的钩子函数，用于获取路由实例
import { useRouter } from 'vue-router'
// 导入 Element Plus 的图标组件
import { HomeFilled, Setting, EditPen } from '@element-plus/icons-vue'

// 获取路由实例，用于编程式导航（如 router.push）
const router = useRouter()

// 定义菜单选择处理函数
// key 是被选中菜单项的 index 值（即路由路径）
const handleSelect = (key: string) => {
  // 使用 router.push 跳转到对应的路径
  router.push(key)
}
</script>

<!-- 全局样式：不带 scoped，作用于整个应用 -->
<style>
/* 设置 body 标签的基础样式 */
body {
  margin: 0;   /* 去除浏览器默认的 8px 外边距 */
  padding: 0;  /* 去除内边距 */
  overflow: hidden; /* 禁止整个浏览器窗口出现滚动条，让应用看起来像原生程序 */
}
</style>

<!-- 局部样式：scoped 表示样式只作用于当前组件 -->
<style scoped>
/* 自定义菜单样式类 */
.el-menu-vertical-demo {
  height: 100%;           /* 高度占满父容器（侧边栏） */
  display: flex;          /* 启用 Flex 布局 */
  flex-direction: column; /* 设置主轴方向为垂直列，让子元素从上到下排列 */
  border-right: none;     /* 移除菜单默认的右边框，避免出现双重边框或滚动条 */
}
</style>