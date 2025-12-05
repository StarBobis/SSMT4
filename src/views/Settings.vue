<template>
  <div class="settings">
    <h1>{{ $t('settings.title') }}</h1>
    <p>{{ $t('settings.description') }}</p>

    <el-card class="box-card" style="margin-bottom: 20px;" :header="$t('settings.language')">
      <el-row style="margin: 5px 5px 5px 5px;">
        <el-col :span="24">
          <el-text class="mx-1" type="primary" style="display: block; text-align: left;">{{ $t('settings.currentLanguage') }}: {{ currentLocale === 'zh-CN' ? '中文' : 'English' }}</el-text>
          <div style="display: flex; align-items: center; margin-top: 10px;">
            <el-button-group>
              <el-button @click="switchLanguage('zh-CN')" :type="currentLocale === 'zh-CN' ? 'primary' : 'default'">{{ $t('settings.chinese') }}</el-button>
              <el-button @click="switchLanguage('en-US')" :type="currentLocale === 'en-US' ? 'primary' : 'default'">{{ $t('settings.english') }}</el-button>
            </el-button-group>
          </div>
        </el-col>
      </el-row>
    </el-card>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useI18n } from 'vue-i18n'

// 获取 i18n 实例
const { locale } = useI18n()

// 当前语言
const currentLocale = computed(() => locale.value)

// 语言切换函数
const switchLanguage = (lang: string) => {
  locale.value = lang
}
</script>

<style scoped>
.settings {
  padding: 20px;
}
</style>