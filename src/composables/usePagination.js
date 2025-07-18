import { ref, computed, watch } from 'vue'

export function usePagination(options = {}) {
  const {
    defaultPage = 1,
    defaultPerPage = 20,
    perPageOptions = [10, 20, 50, 100],
    onPageChange = null,
    onPerPageChange = null
  } = options

  const currentPage = ref(defaultPage)
  const perPage = ref(defaultPerPage)
  const totalCount = ref(0)
  const totalPages = ref(0)
  const loading = ref(false)

  // 计算总页数
  const computedTotalPages = computed(() => {
    return Math.ceil(totalCount.value / perPage.value)
  })

  // 监听总页数变化
  watch(computedTotalPages, (newTotal) => {
    totalPages.value = newTotal
    // 如果当前页超过总页数，回到第一页
    if (currentPage.value > newTotal && newTotal > 0) {
      currentPage.value = 1
    }
  })

  // 监听页码变化
  watch(currentPage, (newPage) => {
    if (onPageChange) {
      onPageChange(newPage, perPage.value)
    }
  })

  // 监听每页条数变化
  watch(perPage, (newPerPage) => {
    // 重置到第一页
    currentPage.value = 1
    totalPages.value = Math.ceil(totalCount.value / newPerPage)
    
    if (onPerPageChange) {
      onPerPageChange(newPerPage, currentPage.value)
    }
  })

  const goToPage = (page) => {
    if (page >= 1 && page <= totalPages.value && page !== currentPage.value) {
      currentPage.value = page
    }
  }

  const goToFirstPage = () => {
    goToPage(1)
  }

  const goToLastPage = () => {
    goToPage(totalPages.value)
  }

  const goToNextPage = () => {
    goToPage(currentPage.value + 1)
  }

  const goToPrevPage = () => {
    goToPage(currentPage.value - 1)
  }

  const setPerPage = (newPerPage) => {
    perPage.value = newPerPage
  }

  const setTotalCount = (count) => {
    totalCount.value = count
  }

  const reset = () => {
    currentPage.value = defaultPage
    perPage.value = defaultPerPage
    totalCount.value = 0
    totalPages.value = 0
  }

  // 获取当前页面的数据范围
  const getDataRange = () => {
    const start = (currentPage.value - 1) * perPage.value + 1
    const end = Math.min(currentPage.value * perPage.value, totalCount.value)
    return { start, end }
  }

  return {
    // 状态
    currentPage,
    perPage,
    totalCount,
    totalPages,
    loading,
    perPageOptions,

    // 方法
    goToPage,
    goToFirstPage,
    goToLastPage,
    goToNextPage,
    goToPrevPage,
    setPerPage,
    setTotalCount,
    reset,
    getDataRange,

    // 计算属性
    computedTotalPages
  }
}