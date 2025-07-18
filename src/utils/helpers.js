import { DATE_FORMAT } from './constants'

// 格式化日期
export const formatDate = (date, format = DATE_FORMAT) => {
  if (!date) return '-'
  
  const d = new Date(date)
  if (isNaN(d.getTime())) return '-'
  
  return d.toLocaleString('zh-CN', {
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit',
    hour12: false
  })
}

// 格式化文件大小
export const formatFileSize = (bytes) => {
  if (bytes === 0) return '0 B'
  
  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i]
}

// 格式化数字
export const formatNumber = (num) => {
  if (num === null || num === undefined) return '0'
  return num.toLocaleString('zh-CN')
}

// 截断文本
export const truncateText = (text, maxLength = 50) => {
  if (!text || text.length <= maxLength) return text
  return text.substring(0, maxLength) + '...'
}

// 掩码化API密钥
export const maskApiKey = (key) => {
  if (!key || key.length < 8) return '****'
  if (key.startsWith('AIza')) {
    // 保留AIza开头，中间用星号，结尾显示4位
    return key.substring(0, 6) + '****' + key.substring(key.length - 4)
  }
  return key.substring(0, 4) + '****' + key.substring(key.length - 4)
}

// 复制到剪贴板
export const copyToClipboard = async (text) => {
  try {
    await navigator.clipboard.writeText(text)
    return true
  } catch (err) {
    // 降级方案
    const textArea = document.createElement('textarea')
    textArea.value = text
    document.body.appendChild(textArea)
    textArea.select()
    try {
      document.execCommand('copy')
      return true
    } catch (err) {
      return false
    } finally {
      document.body.removeChild(textArea)
    }
  }
}

// 防抖函数
export const debounce = (func, wait) => {
  let timeout
  return function executedFunction(...args) {
    const later = () => {
      clearTimeout(timeout)
      func(...args)
    }
    clearTimeout(timeout)
    timeout = setTimeout(later, wait)
  }
}

// 节流函数
export const throttle = (func, limit) => {
  let inThrottle
  return function(...args) {
    if (!inThrottle) {
      func.apply(this, args)
      inThrottle = true
      setTimeout(() => inThrottle = false, limit)
    }
  }
}

// 获取状态颜色
export const getStatusColor = (status) => {
  const colors = {
    active: 'var(--color-success)',
    inactive: 'var(--color-warning)',
    error: 'var(--color-danger)',
    success: 'var(--color-success)',
    pending: 'var(--color-warning)'
  }
  return colors[status] || 'var(--color-text-secondary)'
}

// 生成UUID
export const generateId = () => {
  return Date.now().toString(36) + Math.random().toString(36).substr(2)
}