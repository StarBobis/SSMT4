import { createI18n } from 'vue-i18n'

// 定义语言包
const messages = {
  'zh-CN': {
    nav: {
      home: '主页',
      workbench: '工作台',
      marker: '贴图标记',
      settings: '设置'
    },
    home: {
      title: '主页',
      welcome: '欢迎使用 SSMT4 应用！',
      settings: '3Dmigoto设置',
      path3dmigoto: '3Dmigoto路径',
      placeholder3dmigoto: '请输入3Dmigoto路径',
      selectPath: '选择路径',
      pathProcess: '进程路径',
      placeholderProcess: '请输入进程路径（文件或可执行）',
      pathStartup: '启动路径',
      placeholderStartup: '请输入启动路径（文件或可执行）',
      startupParams: '启动参数',
      placeholderParams: '请输入启动参数',
      startGame: '开始游戏'
    },
    settings: {
      title: '设置页面',
      description: '这里是设置页面的内容。',
      language: '语言设置',
      currentLanguage: '当前语言',
      chinese: '中文',
      english: 'English'
    }
  },
  'en-US': {
    nav: {
      home: 'Home',
      workbench: 'Workbench',
      marker: 'Marker',
      settings: 'Settings'
    },
    home: {
      title: 'Home',
      welcome: 'Welcome to SSMT4 Application!',
      settings: '3Dmigoto Settings',
      path3dmigoto: '3Dmigoto Path',
      placeholder3dmigoto: 'Please enter 3Dmigoto path',
      selectPath: 'Select Path',
      pathProcess: 'Process Path',
      placeholderProcess: 'Please enter process path (file or executable)',
      pathStartup: 'Startup Path',
      placeholderStartup: 'Please enter startup path (file or executable)',
      startupParams: 'Startup Parameters',
      placeholderParams: 'Please enter startup parameters',
      startGame: 'Start Game'
    },
    settings: {
      title: 'Settings Page',
      description: 'Here is the content of the settings page.',
      language: 'Language Settings',
      currentLanguage: 'Current Language',
      chinese: '中文',
      english: 'English'
    }
  }
}

// 创建i18n实例
const i18n = createI18n({
  legacy: false, // 使用Composition API
  locale: 'zh-CN', // 默认语言
  fallbackLocale: 'en-US', // 回退语言
  messages
})

export default i18n