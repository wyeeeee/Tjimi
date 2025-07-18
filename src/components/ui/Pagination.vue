<template>
  <div class="pagination" v-if="totalPages > 1">
    <div class="pagination-info">
      <span class="pagination-text">
        显示 {{ (currentPage - 1) * perPage + 1 }} - {{ Math.min(currentPage * perPage, totalCount) }} 
        / 共 {{ totalCount }} 项
      </span>
    </div>
    
    <div class="pagination-controls">
      <button 
        @click="goToPage(1)"
        :disabled="currentPage === 1"
        class="pagination-btn"
        :class="{ 'disabled': currentPage === 1 }"
      >
        <Icon name="chevron-double-left" size="16" />
      </button>
      
      <button 
        @click="goToPage(currentPage - 1)"
        :disabled="currentPage === 1"
        class="pagination-btn"
        :class="{ 'disabled': currentPage === 1 }"
      >
        <Icon name="chevron-left" size="16" />
      </button>
      
      <div class="pagination-pages">
        <button
          v-for="page in visiblePages"
          :key="page"
          @click="goToPage(page)"
          class="pagination-btn page-btn"
          :class="{ 'active': page === currentPage }"
        >
          {{ page }}
        </button>
      </div>
      
      <button 
        @click="goToPage(currentPage + 1)"
        :disabled="currentPage === totalPages"
        class="pagination-btn"
        :class="{ 'disabled': currentPage === totalPages }"
      >
        <Icon name="chevron-right" size="16" />
      </button>
      
      <button 
        @click="goToPage(totalPages)"
        :disabled="currentPage === totalPages"
        class="pagination-btn"
        :class="{ 'disabled': currentPage === totalPages }"
      >
        <Icon name="chevron-double-right" size="16" />
      </button>
    </div>
    
    <div class="pagination-size">
      <select 
        v-model="selectedPerPage" 
        @change="handlePerPageChange"
        class="per-page-select"
      >
        <option v-for="size in perPageOptions" :key="size" :value="size">
          {{ size }} 条/页
        </option>
      </select>
    </div>
  </div>
</template>

<script setup>
import { computed, ref, watch } from 'vue'
import Icon from './Icon.vue'

const props = defineProps({
  currentPage: {
    type: Number,
    default: 1
  },
  totalPages: {
    type: Number,
    required: true
  },
  totalCount: {
    type: Number,
    required: true
  },
  perPage: {
    type: Number,
    default: 20
  },
  perPageOptions: {
    type: Array,
    default: () => [10, 20, 50, 100]
  }
})

const emit = defineEmits(['page-change', 'per-page-change'])

const selectedPerPage = ref(props.perPage)

// 计算可见页码
const visiblePages = computed(() => {
  const pages = []
  const maxVisible = 5
  
  if (props.totalPages <= maxVisible) {
    for (let i = 1; i <= props.totalPages; i++) {
      pages.push(i)
    }
  } else {
    const start = Math.max(1, props.currentPage - 2)
    const end = Math.min(props.totalPages, start + maxVisible - 1)
    
    for (let i = start; i <= end; i++) {
      pages.push(i)
    }
  }
  
  return pages
})

const goToPage = (page) => {
  if (page >= 1 && page <= props.totalPages && page !== props.currentPage) {
    emit('page-change', page)
  }
}

const handlePerPageChange = () => {
  emit('per-page-change', selectedPerPage.value)
}

// 同步 perPage 属性变化
watch(() => props.perPage, (newValue) => {
  selectedPerPage.value = newValue
})
</script>

<style scoped>
.pagination {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 1rem;
  padding: 1rem;
  background: var(--color-surface);
  border-radius: 0.5rem;
  border: 1px solid var(--color-border);
}

.pagination-info {
  flex-shrink: 0;
}

.pagination-text {
  font-size: 0.875rem;
  color: var(--color-text-secondary);
}

.pagination-controls {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.pagination-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  border: 1px solid var(--color-border);
  background: var(--color-surface);
  color: var(--color-text);
  border-radius: 0.375rem;
  cursor: pointer;
  transition: all 0.2s;
}

.pagination-btn:hover:not(.disabled) {
  background: var(--color-surface-hover);
  border-color: var(--color-primary);
}

.pagination-btn.disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.pagination-btn.active {
  background: var(--color-primary);
  color: white;
  border-color: var(--color-primary);
}

.pagination-pages {
  display: flex;
  gap: 0.25rem;
}

.page-btn {
  min-width: 32px;
  font-weight: 500;
}

.pagination-size {
  flex-shrink: 0;
}

.per-page-select {
  padding: 0.5rem;
  border: 1px solid var(--color-border);
  border-radius: 0.375rem;
  background: var(--color-surface);
  color: var(--color-text);
  font-size: 0.875rem;
  cursor: pointer;
}

.per-page-select:focus {
  outline: none;
  border-color: var(--color-primary);
}

/* 移动端适配 */
@media (max-width: 768px) {
  .pagination {
    flex-direction: column;
    gap: 0.75rem;
  }
  
  .pagination-controls {
    order: 2;
  }
  
  .pagination-info {
    order: 1;
  }
  
  .pagination-size {
    order: 3;
  }
  
  .pagination-text {
    font-size: 0.75rem;
  }
  
  .pagination-btn {
    width: 28px;
    height: 28px;
  }
}

@media (max-width: 480px) {
  .pagination-pages {
    gap: 0.125rem;
  }
  
  .pagination-btn {
    width: 24px;
    height: 24px;
  }
  
  .page-btn {
    min-width: 24px;
  }
}
</style>