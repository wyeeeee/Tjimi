<template>
  <div class="mobile-pagination" v-if="totalPages > 1">
    <div class="pagination-info">
      <span class="info-text">
        第 {{ currentPage }} / {{ totalPages }} 页
      </span>
      <span class="count-text">
        共 {{ totalCount }} 条
      </span>
    </div>
    
    <div class="pagination-controls">
      <button 
        @click="$emit('page-change', currentPage - 1)"
        :disabled="currentPage === 1"
        class="page-btn"
        :class="{ 'disabled': currentPage === 1 }"
      >
        <Icon name="chevron-left" size="18" />
        上一页
      </button>
      
      <button 
        @click="$emit('page-change', currentPage + 1)"
        :disabled="currentPage === totalPages"
        class="page-btn"
        :class="{ 'disabled': currentPage === totalPages }"
      >
        下一页
        <Icon name="chevron-right" size="18" />
      </button>
    </div>
    
    <div class="load-more-section" v-if="showLoadMore">
      <button 
        @click="$emit('load-more')"
        :disabled="loading"
        class="load-more-btn"
      >
        {{ loading ? '加载中...' : '加载更多' }}
      </button>
    </div>
  </div>
</template>

<script setup>
import Icon from '@/components/ui/Icon.vue'

defineProps({
  currentPage: {
    type: Number,
    required: true
  },
  totalPages: {
    type: Number,
    required: true
  },
  totalCount: {
    type: Number,
    required: true
  },
  showLoadMore: {
    type: Boolean,
    default: false
  },
  loading: {
    type: Boolean,
    default: false
  }
})

defineEmits(['page-change', 'load-more'])
</script>

<style scoped>
.mobile-pagination {
  display: flex;
  flex-direction: column;
  gap: 1rem;
  padding: 1rem;
  background: var(--color-surface);
  border-radius: 0.5rem;
  border: 1px solid var(--color-border);
}

.pagination-info {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.info-text {
  font-size: 0.875rem;
  font-weight: 500;
  color: var(--color-text);
}

.count-text {
  font-size: 0.75rem;
  color: var(--color-text-secondary);
}

.pagination-controls {
  display: flex;
  gap: 0.5rem;
}

.page-btn {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 0.5rem;
  padding: 0.75rem;
  border: 1px solid var(--color-border);
  background: var(--color-surface);
  color: var(--color-text);
  border-radius: 0.375rem;
  cursor: pointer;
  transition: all 0.2s;
  font-size: 0.875rem;
  font-weight: 500;
}

.page-btn:hover:not(.disabled) {
  background: var(--color-surface-hover);
  border-color: var(--color-primary);
}

.page-btn.disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.load-more-section {
  display: flex;
  justify-content: center;
}

.load-more-btn {
  padding: 0.75rem 2rem;
  background: var(--color-primary);
  color: white;
  border: none;
  border-radius: 0.375rem;
  cursor: pointer;
  font-size: 0.875rem;
  font-weight: 500;
  transition: background-color 0.2s;
}

.load-more-btn:hover:not(:disabled) {
  background: var(--color-primary-hover);
}

.load-more-btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

@media (max-width: 480px) {
  .mobile-pagination {
    padding: 0.75rem;
  }
  
  .page-btn {
    padding: 0.625rem;
    font-size: 0.8rem;
  }
  
  .info-text {
    font-size: 0.8rem;
  }
  
  .count-text {
    font-size: 0.7rem;
  }
}
</style>