import { ref, reactive, computed } from 'vue'

export function useForm(initialData = {}, validationRules = {}) {
  const form = reactive({ ...initialData })
  const errors = ref({})
  const touched = ref({})

  const validate = () => {
    const newErrors = {}
    
    for (const [field, rules] of Object.entries(validationRules)) {
      const value = form[field]
      
      for (const rule of rules) {
        const error = rule(value, form)
        if (error) {
          newErrors[field] = error
          break
        }
      }
    }
    
    errors.value = newErrors
    return Object.keys(newErrors).length === 0
  }

  const validateField = (field) => {
    const rules = validationRules[field]
    if (!rules) return true

    const value = form[field]
    let error = null
    
    for (const rule of rules) {
      error = rule(value, form)
      if (error) break
    }
    
    if (error) {
      errors.value[field] = error
    } else {
      delete errors.value[field]
    }
    
    return !error
  }

  const reset = () => {
    Object.assign(form, initialData)
    errors.value = {}
    touched.value = {}
  }

  const setTouched = (field) => {
    touched.value[field] = true
  }

  const isValid = computed(() => Object.keys(errors.value).length === 0)

  return {
    form,
    errors,
    touched,
    validate,
    validateField,
    reset,
    setTouched,
    isValid
  }
}