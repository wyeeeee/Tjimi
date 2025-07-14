<template>
  <div :class="wrapperClasses">
    <label v-if="label" :for="inputId" class="input-label">
      {{ label }}
      <span v-if="required" class="input-required">*</span>
    </label>
    
    <div class="input-container">
      <Icon v-if="prefixIcon" :name="prefixIcon" class="input-prefix-icon" />
      
      <input
        :id="inputId"
        :class="inputClasses"
        :type="inputType"
        :value="modelValue"
        :placeholder="placeholder"
        :disabled="disabled"
        :readonly="readonly"
        :required="required"
        :autocomplete="autocomplete"
        :maxlength="maxlength"
        @input="handleInput"
        @blur="handleBlur"
        @focus="handleFocus"
        @keydown="handleKeydown"
        ref="inputRef"
      />
      
      <button
        v-if="type === 'password'"
        type="button"
        class="input-suffix-button"
        @click="togglePasswordVisibility"
        tabindex="-1"
      >
        <Icon :name="showPassword ? 'eye-off' : 'eye'" />
      </button>
      
      <Icon v-else-if="suffixIcon" :name="suffixIcon" class="input-suffix-icon" />
      
      <div v-if="loading" class="input-loading">
        <Icon name="loading" />
      </div>
    </div>
    
    <div v-if="error || hint" class="input-message">
      <p v-if="error" class="input-error">{{ error }}</p>
      <p v-else-if="hint" class="input-hint">{{ hint }}</p>
    </div>
  </div>
</template>

<script setup>
import { computed, ref, nextTick, watch } from 'vue'
import Icon from './Icon.vue'

const props = defineProps({
  modelValue: {
    type: [String, Number],
    default: ''
  },
  type: {
    type: String,
    default: 'text',
    validator: (value) => ['text', 'email', 'password', 'number', 'tel', 'url', 'search'].includes(value)
  },
  label: {
    type: String,
    default: null
  },
  placeholder: {
    type: String,
    default: null
  },
  hint: {
    type: String,
    default: null
  },
  error: {
    type: String,
    default: null
  },
  prefixIcon: {
    type: String,
    default: null
  },
  suffixIcon: {
    type: String,
    default: null
  },
  size: {
    type: String,
    default: 'md',
    validator: (value) => ['sm', 'md', 'lg'].includes(value)
  },
  disabled: {
    type: Boolean,
    default: false
  },
  readonly: {
    type: Boolean,
    default: false
  },
  required: {
    type: Boolean,
    default: false
  },
  loading: {
    type: Boolean,
    default: false
  },
  autocomplete: {
    type: String,
    default: null
  },
  maxlength: {
    type: [String, Number],
    default: null
  },
  autofocus: {
    type: Boolean,
    default: false
  }
})

const emit = defineEmits(['update:modelValue', 'blur', 'focus', 'keydown'])

const inputRef = ref(null)
const showPassword = ref(false)
const isFocused = ref(false)

const inputId = computed(() => `input-${Math.random().toString(36).substr(2, 9)}`)

const inputType = computed(() => {
  if (props.type === 'password' && showPassword.value) {
    return 'text'
  }
  return props.type
})

const wrapperClasses = computed(() => [
  'input-wrapper',
  `input-wrapper--${props.size}`,
  {
    'input-wrapper--error': props.error,
    'input-wrapper--disabled': props.disabled,
    'input-wrapper--focused': isFocused.value,
    'input-wrapper--readonly': props.readonly
  }
])

const inputClasses = computed(() => [
  'input',
  {
    'input--with-prefix': props.prefixIcon,
    'input--with-suffix': props.suffixIcon || props.type === 'password' || props.loading
  }
])

const handleInput = (event) => {
  emit('update:modelValue', event.target.value)
}

const handleBlur = (event) => {
  isFocused.value = false
  emit('blur', event)
}

const handleFocus = (event) => {
  isFocused.value = true
  emit('focus', event)
}

const handleKeydown = (event) => {
  emit('keydown', event)
}

const togglePasswordVisibility = () => {
  showPassword.value = !showPassword.value
}

const focus = () => {
  nextTick(() => {
    inputRef.value?.focus()
  })
}

// Auto focus
watch(() => props.autofocus, (newValue) => {
  if (newValue) {
    focus()
  }
}, { immediate: true })

defineExpose({
  focus,
  blur: () => inputRef.value?.blur()
})
</script>

<style scoped>
.input-wrapper {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.input-label {
  display: block;
  font-size: 0.875rem;
  font-weight: 500;
  color: var(--color-text);
  line-height: 1.4;
}

.input-required {
  color: var(--color-danger);
  margin-left: 0.125rem;
}

.input-container {
  position: relative;
  display: flex;
  align-items: center;
}

.input {
  flex: 1;
  width: 100%;
  background-color: var(--color-input-bg);
  border: 1px solid var(--color-border);
  border-radius: 0.5rem;
  color: var(--color-text);
  font-size: 0.875rem;
  line-height: 1.5;
  transition: all 0.2s ease-in-out;
  outline: none;
}

.input::placeholder {
  color: var(--color-text-placeholder);
}

.input:focus {
  border-color: var(--color-primary);
  box-shadow: 0 0 0 3px rgba(var(--color-primary-rgb), 0.1);
}

.input:disabled {
  background-color: var(--color-surface-disabled);
  color: var(--color-text-disabled);
  cursor: not-allowed;
}

.input:readonly {
  background-color: var(--color-surface-secondary);
  cursor: default;
}

/* Sizes */
.input-wrapper--sm .input {
  height: 2.25rem;
  padding: 0 0.75rem;
  font-size: 0.8125rem;
}

.input-wrapper--md .input {
  height: 2.5rem;
  padding: 0 1rem;
  font-size: 0.875rem;
}

.input-wrapper--lg .input {
  height: 2.75rem;
  padding: 0 1.25rem;
  font-size: 1rem;
}

/* With icons */
.input--with-prefix {
  padding-left: 2.5rem;
}

.input--with-suffix {
  padding-right: 2.5rem;
}

.input-wrapper--sm .input--with-prefix {
  padding-left: 2.25rem;
}

.input-wrapper--sm .input--with-suffix {
  padding-right: 2.25rem;
}

.input-wrapper--lg .input--with-prefix {
  padding-left: 2.75rem;
}

.input-wrapper--lg .input--with-suffix {
  padding-right: 2.75rem;
}

/* Icons */
.input-prefix-icon,
.input-suffix-icon {
  position: absolute;
  color: var(--color-text-secondary);
  pointer-events: none;
  z-index: 1;
}

.input-prefix-icon {
  left: 0.75rem;
}

.input-suffix-icon {
  right: 0.75rem;
}

.input-wrapper--sm .input-prefix-icon {
  left: 0.625rem;
}

.input-wrapper--sm .input-suffix-icon {
  right: 0.625rem;
}

.input-wrapper--lg .input-prefix-icon {
  left: 1rem;
}

.input-wrapper--lg .input-suffix-icon {
  right: 1rem;
}

/* Suffix button */
.input-suffix-button {
  position: absolute;
  right: 0.75rem;
  background: none;
  border: none;
  color: var(--color-text-secondary);
  cursor: pointer;
  padding: 0.25rem;
  border-radius: 0.25rem;
  transition: color 0.2s ease-in-out;
  z-index: 1;
}

.input-suffix-button:hover {
  color: var(--color-text);
}

.input-suffix-button:focus {
  outline: 2px solid var(--color-primary);
  outline-offset: 2px;
}

/* Loading */
.input-loading {
  position: absolute;
  right: 0.75rem;
  color: var(--color-text-secondary);
  pointer-events: none;
  z-index: 1;
}

/* States */
.input-wrapper--error .input {
  border-color: var(--color-danger);
}

.input-wrapper--error .input:focus {
  border-color: var(--color-danger);
  box-shadow: 0 0 0 3px rgba(var(--color-danger-rgb), 0.1);
}

.input-wrapper--focused .input-prefix-icon,
.input-wrapper--focused .input-suffix-icon {
  color: var(--color-primary);
}

/* Messages */
.input-message {
  min-height: 1.25rem;
}

.input-error,
.input-hint {
  margin: 0;
  font-size: 0.8125rem;
  line-height: 1.4;
}

.input-error {
  color: var(--color-danger);
}

.input-hint {
  color: var(--color-text-secondary);
}

/* Mobile optimizations */
@media (max-width: 640px) {
  .input {
    font-size: 16px; /* Prevent zoom on iOS */
  }
  
  .input-wrapper--sm .input {
    height: 44px; /* Minimum touch target */
  }
  
  .input-wrapper--md .input {
    height: 44px;
  }
  
  .input-wrapper--lg .input {
    height: 48px;
  }
}
</style>