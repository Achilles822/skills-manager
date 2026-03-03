<script setup lang="ts">
import { ref } from "vue"
import type { FileEntry } from "@/composables/useSkillFiles"

defineProps<{
  entries: FileEntry[]
  selectedPath: string | null
  depth?: number
}>()

const emit = defineEmits<{
  select: [entry: FileEntry]
}>()

const expanded = ref<Set<string>>(new Set())

function toggle(entry: FileEntry) {
  if (!entry.is_dir) return
  if (expanded.value.has(entry.path)) {
    expanded.value.delete(entry.path)
  } else {
    expanded.value.add(entry.path)
  }
}

function getFileIcon(entry: FileEntry): string {
  if (entry.is_dir) return "📁"
  const ext = entry.name.split(".").pop()?.toLowerCase() || ""
  const iconMap: Record<string, string> = {
    md: "📄",
    js: "🟨",
    ts: "🔷",
    py: "🐍",
    sh: "⚙️",
    json: "📋",
    yaml: "📋",
    yml: "📋",
    toml: "📋",
    png: "🖼️",
    jpg: "🖼️",
    jpeg: "🖼️",
    gif: "🖼️",
    svg: "🖼️",
  }
  return iconMap[ext] || "📄"
}
</script>

<template>
  <ul class="file-tree" :class="{ 'file-tree--root': !depth }">
    <li v-for="entry in entries" :key="entry.path" class="file-tree__item">
      <div
        class="file-tree__row"
        :class="{
          'file-tree__row--selected': !entry.is_dir && selectedPath === entry.path,
          'file-tree__row--dir': entry.is_dir,
        }"
        :style="{ paddingLeft: `${(depth || 0) * 14 + 6}px` }"
        @click="entry.is_dir ? toggle(entry) : emit('select', entry)"
      >
        <span v-if="entry.is_dir" class="file-tree__arrow" :class="{ 'file-tree__arrow--open': expanded.has(entry.path) }">
          <svg width="10" height="10" viewBox="0 0 10 10">
            <path d="M3 2l4 3-4 3z" fill="currentColor"/>
          </svg>
        </span>
        <span class="file-tree__icon">{{ getFileIcon(entry) }}</span>
        <span class="file-tree__name">{{ entry.name }}</span>
      </div>
      <SkillFileTree
        v-if="entry.is_dir && entry.children && expanded.has(entry.path)"
        :entries="entry.children"
        :selected-path="selectedPath"
        :depth="(depth || 0) + 1"
        @select="emit('select', $event)"
      />
    </li>
  </ul>
</template>

<style scoped>
.file-tree {
  list-style: none;
  margin: 0;
  padding: 0;
}

.file-tree--root {
  overflow-y: auto;
}

.file-tree__item {
  margin: 0;
}

.file-tree__row {
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 3px 6px;
  cursor: pointer;
  border-radius: 4px;
  font-size: 0.8rem;
  color: var(--neu-text);
  transition: background var(--neu-transition);
  user-select: none;
}

.file-tree__row:hover {
  background: rgba(209, 217, 230, 0.35);
}

.file-tree__row--selected {
  background: rgba(232, 115, 74, 0.12);
  color: var(--neu-accent);
  font-weight: 600;
}

.file-tree__row--dir {
  font-weight: 500;
}

.file-tree__arrow {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 14px;
  height: 14px;
  flex-shrink: 0;
  color: var(--neu-text-muted);
  transition: transform 150ms ease;
}

.file-tree__arrow--open {
  transform: rotate(90deg);
}

.file-tree__icon {
  font-size: 0.75rem;
  flex-shrink: 0;
  line-height: 1;
}

.file-tree__name {
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}
</style>
