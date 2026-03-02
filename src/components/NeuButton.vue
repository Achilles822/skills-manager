<script setup lang="ts">
defineProps<{
  disabled?: boolean
  loading?: boolean
}>()
</script>

<template>
  <button
    type="button"
    class="neu-button"
    :class="{
      'neu-button--disabled': disabled,
      'neu-button--loading': loading,
    }"
    :disabled="disabled || loading"
  >
    <span v-if="loading" class="neu-button__spinner" aria-hidden="true" />
    <slot v-else />
  </button>
</template>

<style scoped>
.neu-button {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 0.5rem;
  padding: 0.5rem 1rem;
  font-size: 0.85rem;
  font-weight: 500;
  font-family: inherit;
  color: var(--neu-text);
  background: var(--neu-bg);
  border: none;
  border-radius: var(--neu-radius-sm);
  box-shadow: var(--neu-shadow-out-sm);
  cursor: pointer;
  transition: box-shadow var(--neu-transition), transform var(--neu-transition);
}

.neu-button:hover:not(:disabled) {
  box-shadow: var(--neu-shadow-out);
}

.neu-button:active:not(:disabled) {
  box-shadow: var(--neu-shadow-in-sm);
  transform: scale(0.98);
}

.neu-button--disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.neu-button--loading {
  cursor: wait;
}

.neu-button__spinner {
  width: 14px;
  height: 14px;
  border: 2px solid var(--neu-dark);
  border-top-color: var(--neu-accent);
  border-radius: 50%;
  animation: neu-spin 0.8s linear infinite;
}

@keyframes neu-spin {
  to {
    transform: rotate(360deg);
  }
}

.neu-button:focus-visible {
  outline: 2px solid var(--neu-accent);
  outline-offset: 2px;
}
</style>
