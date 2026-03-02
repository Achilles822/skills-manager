<script setup lang="ts">
const props = defineProps<{
  modelValue: boolean
  loading?: boolean
  disabled?: boolean
}>()

const emit = defineEmits<{
  "update:modelValue": [value: boolean]
}>()

function handleClick(e: MouseEvent) {
  e.stopPropagation()
  if (props.loading || props.disabled) return
  emit("update:modelValue", !props.modelValue)
}
</script>

<template>
  <div
    class="neu-toggle"
    :class="{
      'neu-toggle--on': modelValue,
      'neu-toggle--loading': loading,
      'neu-toggle--disabled': disabled,
    }"
    role="switch"
    :aria-checked="modelValue"
    :aria-disabled="loading || disabled"
    tabindex="0"
    @click="handleClick"
    @keydown.enter.prevent.stop="handleClick($event as any)"
    @keydown.space.prevent.stop="handleClick($event as any)"
  >
    <div class="neu-toggle__track">
      <div class="neu-toggle__knob">
        <span v-if="loading" class="neu-toggle__spinner" aria-hidden="true" />
      </div>
    </div>
  </div>
</template>

<style scoped>
.neu-toggle {
  display: inline-flex;
  cursor: pointer;
  user-select: none;
  flex-shrink: 0;
}

.neu-toggle--disabled,
.neu-toggle--loading {
  cursor: not-allowed;
  opacity: 0.6;
}

.neu-toggle__track {
  width: 48px;
  height: 26px;
  background: var(--neu-bg);
  border-radius: 13px;
  box-shadow: var(--neu-shadow-in-sm);
  padding: 3px;
  transition: background var(--neu-transition);
}

.neu-toggle--on .neu-toggle__track {
  background: var(--neu-accent-light);
}

.neu-toggle__knob {
  width: 20px;
  height: 20px;
  background: var(--neu-bg);
  border-radius: 50%;
  box-shadow: 2px 2px 4px var(--neu-dark), -1px -1px 3px var(--neu-light);
  margin-left: 0;
  transition: margin-left 250ms ease, box-shadow 250ms ease, background 250ms ease;
  display: flex;
  align-items: center;
  justify-content: center;
}

.neu-toggle--on .neu-toggle__knob {
  margin-left: 22px;
  background: var(--neu-accent);
  box-shadow: 2px 2px 5px rgba(209, 217, 230, 0.6), -1px -1px 3px rgba(255, 255, 255, 0.8);
}

.neu-toggle__spinner {
  width: 10px;
  height: 10px;
  border: 2px solid rgba(255, 255, 255, 0.4);
  border-top-color: #fff;
  border-radius: 50%;
  animation: neu-spin 0.8s linear infinite;
}

@keyframes neu-spin {
  to {
    transform: rotate(360deg);
  }
}

.neu-toggle:focus-visible .neu-toggle__track {
  outline: 2px solid var(--neu-accent);
  outline-offset: 2px;
}
</style>
