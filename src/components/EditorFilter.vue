<script setup lang="ts">
import { computed } from "vue"
import NeuSelect from "@/components/NeuSelect.vue"
import type { EditorInfo } from "@/types"

const props = defineProps<{
  editors: EditorInfo[]
  selectedEditorIds: string[]
}>()

const emit = defineEmits<{
  "update:selectedEditorIds": [value: string[]]
}>()

const options = computed(() => {
  const allOption = { id: "all", label: "全部" }
  const editorOptions = props.editors
    .filter((e) => e.installed)
    .map((e) => ({ id: e.id, label: e.display_name }))
  return [allOption, ...editorOptions]
})
</script>

<template>
  <div class="editor-filter">
    <label class="editor-filter__label">编辑器</label>
    <NeuSelect
      :options="options"
      :model-value="selectedEditorIds"
      @update:model-value="emit('update:selectedEditorIds', $event)"
    />
  </div>
</template>

<style scoped>
.editor-filter {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.editor-filter__label {
  font-size: 0.75rem;
  font-weight: 600;
  color: var(--neu-text-muted);
  text-transform: uppercase;
  letter-spacing: 0.05em;
}
</style>
