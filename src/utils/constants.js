// 应用常量
export const APP_NAME = 'Tjimi Proxy'
export const APP_VERSION = '1.0.0'

// 页面标题
export const PAGE_TITLES = {
  home: '首页',
  apiKeys: 'API 密钥管理',
  logs: '请求日志',
  settings: '系统设置',
  login: '登录'
}

// 导航菜单
export const NAVIGATION_ITEMS = [
  {
    path: '/api-keys',
    label: '控制面板',
    icon: 'key'
  },
  {
    path: '/logs',
    label: '请求日志',
    icon: 'logs'
  },
  {
    path: '/settings',
    label: '系统设置',
    icon: 'settings'
  }
]

// API密钥状态
export const API_KEY_STATUS = {
  ACTIVE: 'active',
  INACTIVE: 'inactive',
  ERROR: 'error'
}

// API密钥状态标签
export const API_KEY_STATUS_LABELS = {
  [API_KEY_STATUS.ACTIVE]: '正常',
  [API_KEY_STATUS.INACTIVE]: '已禁用',
  [API_KEY_STATUS.ERROR]: '异常'
}

// 请求状态
export const REQUEST_STATUS = {
  SUCCESS: 'success',
  ERROR: 'error',
  PENDING: 'pending'
}

// 页面大小选项
export const PAGE_SIZE_OPTIONS = [10, 20, 50, 100]

// 默认页面大小
export const DEFAULT_PAGE_SIZE = 20

// 日期格式
export const DATE_FORMAT = 'YYYY-MM-DD HH:mm:ss'

// 响应式断点
export const BREAKPOINTS = {
  xs: 480,
  sm: 768,
  md: 1024,
  lg: 1280,
  xl: 1920
}