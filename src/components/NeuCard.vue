<script setup lang="ts">
defineProps<{
  selected?: boolean
  clickable?: boolean
}>()

defineEmits<{
  click: []
}>()
</script>

<template>
  <div
    class="neu-card"
    :class="{
      'neu-card--selected': selected,
      'neu-card--clickable': clickable !== false,
    }"
    role="button"
    :tabindex="clickable !== false ? 0 : undefined"
    @click="clickable !== false && $emit('click')"
    @keydown.enter="clickable !== false && $emit('click')"
    @keydown.space.prevent="clickable !== false && $emit('click')"
  >
    <slot />
  </div>
</template>

<style scoped>
.neu-card {
  background: var(--neu-bg);
  border-radius: var(--neu-radius-md);
  padding: 0.875rem 1rem;
  box-shadow: var(--neu-shadow-out);
  transition: box-shadow var(--neu-transition-slow), transform var(--neu-transition);
}

.neu-card--clickable {
  cursor: pointer;
}

.neu-card--clickable:hover {
  box-shadow: 6px 6px 14px var(--neu-dark), -6px -6px 14px var(--neu-light);
}

.neu-card--clickable:active {
  transform: scale(0.99);
}

.neu-card--selected {
  box-shadow: var(--neu-shadow-in);
}

.neu-card--clickable:focus-visible {
  outline: 2px solid var(--neu-accent);
  outline-offset: 2px;
}
</style>
