<script setup lang="ts">
import { computed } from "vue"

interface Option {
  id: string
  label: string
}

const props = defineProps<{
  options: Option[]
  modelValue: string[]
}>()

const emit = defineEmits<{
  "update:modelValue": [value: string[]]
}>()

const isAllSelected = computed(() => {
  const selectableOptions = props.options.filter((o) => o.id !== "all")
  if (selectableOptions.length === 0) return false
  return selectableOptions.every((o) => props.modelValue.includes(o.id))
})

function toggleOption(id: string) {
  if (id === "all") {
    if (isAllSelected.value) {
      emit("update:modelValue", [])
    } else {
      emit(
        "update:modelValue",
        props.options.filter((o) => o.id !== "all").map((o) => o.id)
      )
    }
  } else {
    const current = [...props.modelValue]
    const idx = current.indexOf(id)
    if (idx >= 0) {
      current.splice(idx, 1)
    } else {
      current.push(id)
    }
    emit("update:modelValue", current)
  }
}

function isSelected(id: string) {
  if (id === "all") return isAllSelected.value
  return props.modelValue.includes(id)
}
</script>

<template>
  <div class="neu-select">
    <button
      v-for="opt in options"
      :key="opt.id"
      type="button"
      class="neu-select__pill"
      :class="{ 'neu-select__pill--selected': isSelected(opt.id) }"
      @click="toggleOption(opt.id)"
    >
      {{ opt.label }}
    </button>
  </div>
</template>

<style scoped>
.neu-select {
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
}

.neu-select__pill {
  padding: 0.4rem 0.9rem;
  font-size: 0.85rem;
  font-weight: 500;
  font-family: inherit;
  color: var(--neu-text);
  background: var(--neu-bg);
  border: none;
  border-radius: var(--neu-radius-lg);
  box-shadow: var(--neu-shadow-out-sm);
  cursor: pointer;
  transition: box-shadow var(--neu-transition), transform var(--neu-transition);
}

.neu-select__pill:hover {
  box-shadow: var(--neu-shadow-out);
}

.neu-select__pill--selected {
  box-shadow: var(--neu-shadow-in-sm);
  color: var(--neu-accent);
  font-weight: 600;
}

.neu-select__pill:active {
  transform: scale(0.97);
}

.neu-select__pill:focus-visible {
  outline: 2px solid var(--neu-accent);
  outline-offset: 2px;
}
</style>
