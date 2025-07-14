<template>
  <div class="data-table">
    <!-- Header -->
    <div class="table-header" v-if="title || $slots.header || searchable">
      <div class="header-content">
        <div class="header-main" v-if="title || $slots.header">
          <slot name="header">
            <h3 class="table-title">{{ title }}</h3>
          </slot>
        </div>
        
        <div class="header-actions">
          <div v-if="searchable" class="search-container">
            <Input
              v-model="searchQuery"
              placeholder="搜索..."
              icon="search"
              size="sm"
              class="search-input"
            />
          </div>
          <slot name="actions" />
        </div>
      </div>
    </div>

    <!-- Table Container -->
    <Card class="table-container">
      <!-- Loading State -->
      <div v-if="loading && (!data || data.length === 0)" class="table-loading">
        <div class="loading-content">
          <Icon name="loading" size="32" />
          <h4>加载中...</h4>
          <p>正在获取数据</p>
        </div>
      </div>

      <!-- Error State -->
      <div v-else-if="error" class="table-error">
        <div class="error-content">
          <Icon name="error" size="32" />
          <h4>加载失败</h4>
          <p>{{ error }}</p>
          <Button variant="outline" size="sm" @click="$emit('retry')" icon="refresh">
            重试
          </Button>
        </div>
      </div>

      <!-- Table -->
      <div v-else class="table-wrapper">
        <table class="table">
          <thead class="table-head">
            <tr>
              <th 
                v-for="column in columns" 
                :key="column.key"
                :class="[
                  'table-header-cell',
                  { 'sortable': column.sortable },
                  { 'sorted': sortBy === column.key }
                ]"
                @click="column.sortable && handleSort(column.key)"
              >
                <div class="header-cell-content">
                  <span>{{ column.title }}</span>
                  <Icon 
                    v-if="column.sortable" 
                    :name="getSortIcon(column.key)" 
                    size="14" 
                    class="sort-icon"
                  />
                </div>
              </th>
            </tr>
          </thead>
          <tbody class="table-body">
            <tr 
              v-for="(row, index) in paginatedData" 
              :key="getRowKey(row, index)"
              class="table-row"
              @click="$emit('row-click', row)"
            >
              <td 
                v-for="column in columns" 
                :key="column.key"
                :class="['table-cell', column.class]"
              >
                <slot 
                  :name="`cell-${column.key}`" 
                  :row="row" 
                  :value="getColumnValue(row, column.key)"
                  :column="column"
                  :index="index"
                >
                  <component 
                    v-if="column.component"
                    :is="column.component"
                    :value="getColumnValue(row, column.key)"
                    :row="row"
                    v-bind="column.props"
                  />
                  <span v-else>{{ formatValue(getColumnValue(row, column.key), column) }}</span>
                </slot>
              </td>
            </tr>
          </tbody>
        </table>

        <!-- Empty State -->
        <div v-if="filteredData.length === 0" class="table-empty">
          <div class="empty-content">
            <slot name="empty">
              <Icon :name="emptyIcon" size="48" />
              <h4>{{ emptyTitle }}</h4>
              <p>{{ emptyDescription }}</p>
            </slot>
          </div>
        </div>
      </div>

      <!-- Pagination -->
      <div 
        v-if="pagination && filteredData.length > 0" 
        class="table-pagination"
      >
        <div class="pagination-info">
          <span class="pagination-text">
            显示 {{ startIndex + 1 }}-{{ endIndex }} 项，共 {{ filteredData.length }} 项
          </span>
        </div>
        <div class="pagination-controls">
          <Button
            variant="outline"
            size="sm"
            icon="arrow-left"
            :disabled="currentPage === 1"
            @click="goToPage(currentPage - 1)"
          >
            上一页
          </Button>
          <span class="page-info">
            第 {{ currentPage }} 页，共 {{ totalPages }} 页
          </span>
          <Button
            variant="outline"
            size="sm"
            icon="arrow-right"
            :disabled="currentPage === totalPages"
            @click="goToPage(currentPage + 1)"
          >
            下一页
          </Button>
        </div>
      </div>
    </Card>
  </div>
</template>

<script setup>
import { ref, computed, watch } from 'vue'
import Button from './Button.vue'
import Card from './Card.vue'
import Icon from './Icon.vue'
import Input from './Input.vue'

const props = defineProps({
  title: {
    type: String,
    default: null
  },
  data: {
    type: Array,
    default: () => []
  },
  columns: {
    type: Array,
    required: true
  },
  loading: {
    type: Boolean,
    default: false
  },
  error: {
    type: String,
    default: null
  },
  searchable: {
    type: Boolean,
    default: false
  },
  pagination: {
    type: Boolean,
    default: true
  },
  pageSize: {
    type: Number,
    default: 10
  },
  rowKey: {
    type: [String, Function],
    default: 'id'
  },
  emptyIcon: {
    type: String,
    default: 'logs'
  },
  emptyTitle: {
    type: String,
    default: '暂无数据'
  },
  emptyDescription: {
    type: String,
    default: '当前没有可显示的数据'
  }
})

const emit = defineEmits(['retry', 'row-click', 'sort'])

// Search
const searchQuery = ref('')

// Sorting
const sortBy = ref('')
const sortOrder = ref('asc')

// Pagination
const currentPage = ref(1)

// Computed
const filteredData = computed(() => {
  let result = props.data || []
  
  // Apply search filter
  if (searchQuery.value) {
    const query = searchQuery.value.toLowerCase()
    result = result.filter(row => {
      return props.columns.some(column => {
        const value = getColumnValue(row, column.key)
        return String(value).toLowerCase().includes(query)
      })
    })
  }
  
  // Apply sorting
  if (sortBy.value) {
    result = [...result].sort((a, b) => {
      const aValue = getColumnValue(a, sortBy.value)
      const bValue = getColumnValue(b, sortBy.value)
      
      let comparison = 0
      if (aValue > bValue) comparison = 1
      if (aValue < bValue) comparison = -1
      
      return sortOrder.value === 'asc' ? comparison : -comparison
    })
  }
  
  return result
})

const totalPages = computed(() => {
  if (!props.pagination) return 1
  return Math.ceil(filteredData.value.length / props.pageSize)
})

const startIndex = computed(() => {
  if (!props.pagination) return 0
  return (currentPage.value - 1) * props.pageSize
})

const endIndex = computed(() => {
  if (!props.pagination) return filteredData.value.length
  return Math.min(startIndex.value + props.pageSize, filteredData.value.length)
})

const paginatedData = computed(() => {
  if (!props.pagination) return filteredData.value
  return filteredData.value.slice(startIndex.value, endIndex.value)
})

// Methods
const getColumnValue = (row, key) => {
  return key.split('.').reduce((obj, k) => obj?.[k], row)
}

const getRowKey = (row, index) => {
  if (typeof props.rowKey === 'function') {
    return props.rowKey(row, index)
  }
  return getColumnValue(row, props.rowKey) || index
}

const formatValue = (value, column) => {
  if (column.formatter && typeof column.formatter === 'function') {
    return column.formatter(value)
  }
  if (value === null || value === undefined) {
    return '-'
  }
  return value
}

const handleSort = (key) => {
  if (sortBy.value === key) {
    sortOrder.value = sortOrder.value === 'asc' ? 'desc' : 'asc'
  } else {
    sortBy.value = key
    sortOrder.value = 'asc'
  }
  
  emit('sort', { key, order: sortOrder.value })
}

const getSortIcon = (key) => {
  if (sortBy.value !== key) return 'arrow-up-down'
  return sortOrder.value === 'asc' ? 'arrow-up' : 'arrow-down'
}

const goToPage = (page) => {
  if (page >= 1 && page <= totalPages.value) {
    currentPage.value = page
  }
}

// Reset pagination when search changes
watch(searchQuery, () => {
  currentPage.value = 1
})

// Reset pagination when data changes
watch(() => props.data, () => {
  currentPage.value = 1
})
</script>

<style scoped>
.data-table {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-4);
}

/* Header */
.table-header {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-4);
}

.header-content {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: var(--spacing-4);
  flex-wrap: wrap;
}

.header-main {
  flex: 1;
  min-width: 0;
}

.table-title {
  font-size: var(--text-2xl);
  font-weight: var(--font-bold);
  color: var(--color-text);
  margin: 0;
}

.header-actions {
  display: flex;
  align-items: center;
  gap: var(--spacing-3);
  flex-wrap: wrap;
}

.search-container {
  min-width: 200px;
}

/* Table Container */
.table-container {
  overflow: hidden;
}

/* Loading State */
.table-loading {
  display: flex;
  justify-content: center;
  align-items: center;
  padding: var(--spacing-12);
}

.loading-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: var(--spacing-4);
  text-align: center;
}

.loading-content h4 {
  font-size: var(--text-lg);
  font-weight: var(--font-semibold);
  color: var(--color-text);
  margin: 0;
}

.loading-content p {
  font-size: var(--text-base);
  color: var(--color-text-secondary);
  margin: 0;
}

/* Error State */
.table-error {
  display: flex;
  justify-content: center;
  align-items: center;
  padding: var(--spacing-12);
}

.error-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: var(--spacing-4);
  text-align: center;
}

.error-content h4 {
  font-size: var(--text-lg);
  font-weight: var(--font-semibold);
  color: var(--color-text);
  margin: 0;
}

.error-content p {
  font-size: var(--text-base);
  color: var(--color-text-secondary);
  margin: 0;
}

/* Table */
.table-wrapper {
  overflow-x: auto;
}

.table {
  width: 100%;
  border-collapse: collapse;
  font-size: var(--text-sm);
}

.table-head {
  background-color: var(--color-surface-secondary);
  position: sticky;
  top: 0;
  z-index: 1;
}

.table-header-cell {
  padding: var(--spacing-4);
  text-align: left;
  font-weight: var(--font-semibold);
  color: var(--color-text-secondary);
  border-bottom: 1px solid var(--color-border);
  white-space: nowrap;
  user-select: none;
}

.table-header-cell.sortable {
  cursor: pointer;
  transition: all var(--transition-fast);
}

.table-header-cell.sortable:hover {
  background-color: var(--color-surface-hover);
  color: var(--color-text);
}

.table-header-cell.sorted {
  color: var(--color-primary);
  background-color: rgba(var(--color-primary-rgb), 0.05);
}

.header-cell-content {
  display: flex;
  align-items: center;
  gap: var(--spacing-2);
}

.sort-icon {
  color: var(--color-text-tertiary);
  transition: all var(--transition-fast);
}

.table-header-cell.sortable:hover .sort-icon,
.table-header-cell.sorted .sort-icon {
  color: var(--color-primary);
}

.table-body {
  background-color: var(--color-surface);
}

.table-row {
  transition: all var(--transition-fast);
  cursor: pointer;
}

.table-row:hover {
  background-color: var(--color-surface-secondary);
}

.table-row:active {
  background-color: var(--color-surface-hover);
}

.table-cell {
  padding: var(--spacing-4);
  border-bottom: 1px solid var(--color-border);
  color: var(--color-text);
  vertical-align: middle;
}

/* Empty State */
.table-empty {
  display: flex;
  justify-content: center;
  align-items: center;
  padding: var(--spacing-12);
}

.empty-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: var(--spacing-4);
  text-align: center;
  max-width: 400px;
}

.empty-content h4 {
  font-size: var(--text-xl);
  font-weight: var(--font-semibold);
  color: var(--color-text);
  margin: 0;
}

.empty-content p {
  font-size: var(--text-base);
  color: var(--color-text-secondary);
  margin: 0;
  line-height: 1.5;
}

/* Pagination */
.table-pagination {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--spacing-4);
  border-top: 1px solid var(--color-border);
  gap: var(--spacing-4);
  flex-wrap: wrap;
}

.pagination-info {
  flex: 1;
  min-width: 0;
}

.pagination-text {
  font-size: var(--text-sm);
  color: var(--color-text-secondary);
}

.pagination-controls {
  display: flex;
  align-items: center;
  gap: var(--spacing-3);
}

.page-info {
  font-size: var(--text-sm);
  color: var(--color-text-secondary);
  white-space: nowrap;
}

/* Mobile optimizations */
@media (max-width: 640px) {
  .header-content {
    flex-direction: column;
    align-items: stretch;
  }
  
  .header-actions {
    justify-content: stretch;
  }
  
  .search-container {
    min-width: unset;
    flex: 1;
  }
  
  .table {
    font-size: var(--text-xs);
  }
  
  .table-header-cell,
  .table-cell {
    padding: var(--spacing-2) var(--spacing-3);
  }
  
  .table-pagination {
    flex-direction: column;
    align-items: stretch;
    gap: var(--spacing-3);
  }
  
  .pagination-controls {
    justify-content: space-between;
  }
  
  .page-info {
    order: -1;
    text-align: center;
  }
}

/* Tablet optimizations */
@media (max-width: 768px) {
  .table-wrapper {
    border-radius: var(--radius-lg);
    overflow: auto;
  }
}

/* Dark mode enhancements */
@media (prefers-color-scheme: dark) {
  .table-row:hover {
    background-color: rgba(255, 255, 255, 0.05);
  }
  
  .table-row:active {
    background-color: rgba(255, 255, 255, 0.1);
  }
}

/* Reduced motion */
@media (prefers-reduced-motion: reduce) {
  .table-header-cell,
  .table-row,
  .sort-icon {
    transition: none;
  }
}
</style>