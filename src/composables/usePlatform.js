import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export function usePlatform() {
  const isDesktop = ref(false)
  const isMobile = ref(false)
  const isAndroid = ref(false)
  const isIOS = ref(false)
  const platform = ref('unknown')

  const detectPlatform = async () => {
    try {
      // 通过Tauri命令检测是否为桌面端
      const desktop = await invoke('is_desktop')
      isDesktop.value = desktop
      isMobile.value = !desktop

      // 通过user agent检测具体平台
      const userAgent = navigator.userAgent.toLowerCase()
      
      if (desktop) {
        if (userAgent.includes('win')) {
          platform.value = 'windows'
        } else if (userAgent.includes('mac')) {
          platform.value = 'macos'
        } else if (userAgent.includes('linux')) {
          platform.value = 'linux'
        } else {
          platform.value = 'desktop'
        }
      } else {
        if (userAgent.includes('android')) {
          platform.value = 'android'
          isAndroid.value = true
        } else if (userAgent.includes('iphone') || userAgent.includes('ipad')) {
          platform.value = 'ios'
          isIOS.value = true
        } else {
          platform.value = 'mobile'
        }
      }
    } catch (error) {
      console.error('Failed to detect platform:', error)
      // 如果Tauri命令失败，回退到基于user agent的检测
      const userAgent = navigator.userAgent.toLowerCase()
      if (userAgent.includes('android')) {
        platform.value = 'android'
        isAndroid.value = true
        isMobile.value = true
      } else if (userAgent.includes('iphone') || userAgent.includes('ipad')) {
        platform.value = 'ios'
        isIOS.value = true
        isMobile.value = true
      } else {
        // 默认为桌面端
        platform.value = 'desktop'
        isDesktop.value = true
      }
    }
  }

  onMounted(detectPlatform)

  return {
    isDesktop,
    isMobile,
    isAndroid,
    isIOS,
    platform,
    detectPlatform
  }
}