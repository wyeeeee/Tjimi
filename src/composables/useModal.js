import { ref } from 'vue'

export function useModal() {
  const isOpen = ref(false)
  const data = ref(null)

  const open = (payload = null) => {
    data.value = payload
    isOpen.value = true
  }

  const close = () => {
    isOpen.value = false
    // 延迟清空data，避免模板中访问undefined
    setTimeout(() => {
      data.value = null
    }, 300)
  }

  const toggle = () => {
    if (isOpen.value) {
      close()
    } else {
      open()
    }
  }

  return {
    isOpen,
    data,
    open,
    close,
    toggle
  }
}