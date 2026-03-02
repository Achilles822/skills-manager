<script setup lang="ts">
import NeuCard from "@/components/NeuCard.vue"
import NeuToggle from "@/components/NeuToggle.vue"
import type { Skill } from "@/types"

defineProps<{
  skill: Skill
  selected: boolean
  toggling: boolean
}>()

const emit = defineEmits<{
  select: []
  toggle: []
}>()

function truncate(str: string | null | undefined, len: number) {
  if (!str) return ""
  return str.length > len ? str.slice(0, len) + "…" : str
}

const editorDisplayNames: Record<string, string> = {
  cursor: "Cursor",
  "claude-code": "Claude Code",
}
</script>

<template>
  <NeuCard :selected="selected" :clickable="true" @click="emit('select')">
    <div class="skill-item">
      <div class="skill-item__header">
        <h3 class="skill-item__name">{{ skill.meta.name }}</h3>
        <NeuToggle
          :model-value="skill.enabled"
          :loading="toggling"
          @update:model-value="emit('toggle')"
        />
      </div>
      <p v-if="skill.meta.description" class="skill-item__desc">
        {{ truncate(skill.meta.description, 80) }}
      </p>
      <div class="skill-item__tags">
        <span
          v-for="ed in skill.editors"
          :key="ed"
          class="skill-item__tag skill-item__tag--editor"
        >{{ editorDisplayNames[ed] || ed }}</span>
        <span
          v-if="skill.meta.version"
          class="skill-item__tag skill-item__tag--version"
        >v{{ skill.meta.version }}</span>
      </div>
    </div>
  </NeuCard>
</template>

<style scoped>
.skill-item {
  display: flex;
  flex-direction: column;
  gap: 0.35rem;
}

.skill-item__header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 0.5rem;
}

.skill-item__name {
  margin: 0;
  font-size: 0.9rem;
  font-weight: 600;
  color: var(--neu-text);
  flex: 1;
  min-width: 0;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.skill-item__desc {
  margin: 0;
  font-size: 0.78rem;
  color: var(--neu-text-muted);
  line-height: 1.4;
}

.skill-item__tags {
  display: flex;
  flex-wrap: wrap;
  gap: 0.35rem;
  margin-top: 0.1rem;
}

.skill-item__tag {
  display: inline-flex;
  align-items: center;
  padding: 0.1rem 0.5rem;
  font-size: 0.68rem;
  font-weight: 500;
  border-radius: 6px;
  letter-spacing: 0.01em;
  line-height: 1.5;
}

.skill-item__tag--editor {
  background: rgba(108, 142, 191, 0.14);
  color: #4a6fa5;
}

.skill-item__tag--version {
  background: rgba(72, 187, 120, 0.14);
  color: #2f855a;
}
</style>
