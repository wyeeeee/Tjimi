// 验证规则
export const validationRules = {
  required: (message = '此字段为必填项') => (value) => {
    if (!value || (typeof value === 'string' && !value.trim())) {
      return message
    }
    return null
  },

  minLength: (min, message) => (value) => {
    if (!value || value.length < min) {
      return message || `最少需要${min}个字符`
    }
    return null
  },

  maxLength: (max, message) => (value) => {
    if (value && value.length > max) {
      return message || `最多允许${max}个字符`
    }
    return null
  },

  email: (message = '请输入有效的邮箱地址') => (value) => {
    if (!value) return null
    const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/
    if (!emailRegex.test(value)) {
      return message
    }
    return null
  },

  confirmPassword: (message = '两次输入的密码不一致') => (value, form) => {
    if (value !== form.newPassword) {
      return message
    }
    return null
  },

  apiKey: (message = '请输入有效的API密钥') => (value) => {
    if (!value) return null
    // 简单的API密钥格式验证
    if (value.length < 20 || !/^[A-Za-z0-9_-]+$/.test(value)) {
      return message
    }
    return null
  }
}

// 常用验证组合
export const commonValidations = {
  password: [
    validationRules.required('请输入密码'),
    validationRules.minLength(6, '密码长度至少6位')
  ],
  
  newPassword: [
    validationRules.required('请输入新密码'),
    validationRules.minLength(6, '密码长度至少6位')
  ],
  
  confirmPassword: [
    validationRules.required('请确认密码'),
    validationRules.confirmPassword()
  ],
  
  apiKey: [
    validationRules.required('请输入API密钥'),
    validationRules.apiKey()
  ],
  
  name: [
    validationRules.required('请输入名称'),
    validationRules.maxLength(50, '名称长度不能超过50个字符')
  ]
}