<script setup lang="ts">
defineProps<{
  variant: "symlink" | "copy"
}>()
</script>

<template>
  <span class="neu-tag" :class="`neu-tag--${variant}`">
    {{ variant === "symlink" ? "Symlink" : "Copy" }}
  </span>
</template>

<style scoped>
.neu-tag {
  display: inline-flex;
  align-items: center;
  padding: 0.15rem 0.55rem;
  font-size: 0.7rem;
  font-weight: 600;
  border-radius: 8px;
  text-transform: uppercase;
  letter-spacing: 0.03em;
}

.neu-tag--symlink {
  background: rgba(108, 142, 191, 0.18);
  color: #4a6fa5;
}

.neu-tag--copy {
  background: rgba(72, 187, 120, 0.18);
  color: #2f855a;
}
</style>
