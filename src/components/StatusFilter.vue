<script setup lang="ts">
export type StatusFilterValue = "all" | "enabled" | "disabled"

defineProps<{
  modelValue: StatusFilterValue
}>()

const emit = defineEmits<{
  "update:modelValue": [value: StatusFilterValue]
}>()

const options: { id: StatusFilterValue; label: string }[] = [
  { id: "all", label: "全部" },
  { id: "enabled", label: "应用中" },
  { id: "disabled", label: "已禁用" },
]
</script>

<template>
  <div class="status-filter">
    <label class="status-filter__label">状态</label>
    <div class="status-filter__pills">
      <button
        v-for="opt in options"
        :key="opt.id"
        type="button"
        class="status-filter__pill"
        :class="{ 'status-filter__pill--selected': modelValue === opt.id }"
        @click="emit('update:modelValue', opt.id)"
      >
        {{ opt.label }}
      </button>
    </div>
  </div>
</template>

<style scoped>
.status-filter {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.status-filter__label {
  font-size: 0.75rem;
  font-weight: 600;
  color: var(--neu-text-muted);
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.status-filter__pills {
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
}

.status-filter__pill {
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

.status-filter__pill:hover {
  box-shadow: var(--neu-shadow-out);
}

.status-filter__pill--selected {
  box-shadow: var(--neu-shadow-in-sm);
  color: var(--neu-accent);
  font-weight: 600;
}

.status-filter__pill:active {
  transform: scale(0.97);
}

.status-filter__pill:focus-visible {
  outline: 2px solid var(--neu-accent);
  outline-offset: 2px;
}
</style>
