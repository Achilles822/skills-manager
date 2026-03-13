<script setup lang="ts">
import { computed } from "vue"
import { useI18n } from "vue-i18n"
import NeuCard from "@/components/NeuCard.vue"
import type { Skill, EditorInfo } from "@/types"

const { t } = useI18n()

const props = defineProps<{
  skill: Skill
  selected: boolean
  toggling: boolean
  installedEditors: EditorInfo[]
}>()

const emit = defineEmits<{
  select: []
  toggleEditor: [editorId: string]
}>()

const isCenterSkill = computed(() => props.skill.id.startsWith("center:"))

function truncate(str: string | null | undefined, len: number) {
  if (!str) return ""
  return str.length > len ? str.slice(0, len) + "…" : str
}

function editorLabel(ed: EditorInfo) {
  const short: Record<string, string> = { cursor: "C", "claude-code": "CC" }
  return short[ed.id] || ed.display_name.charAt(0)
}

function isEditorLinked(editorId: string) {
  return props.skill.editors.includes(editorId)
}

function handleEditorToggle(e: MouseEvent, editorId: string) {
  e.stopPropagation()
  if (props.toggling) return
  emit("toggleEditor", editorId)
}
</script>

<template>
  <NeuCard :selected="selected" :clickable="true" @click="emit('select')">
    <div class="skill-item">
      <div class="skill-item__header">
        <h3 class="skill-item__name">{{ skill.meta.name }}</h3>
        <div v-if="isCenterSkill" class="skill-item__editor-toggles">
          <button
            v-for="ed in installedEditors"
            :key="ed.id"
            class="skill-item__editor-btn"
            :class="{ 'skill-item__editor-btn--on': isEditorLinked(ed.id) }"
            :title="ed.display_name"
            :disabled="toggling"
            @click="handleEditorToggle($event, ed.id)"
          >{{ editorLabel(ed) }}</button>
        </div>
      </div>
      <p v-if="skill.meta.description" class="skill-item__desc">
        {{ truncate(skill.meta.description, 80) }}
      </p>
      <div class="skill-item__tags">
        <span
          v-if="skill.is_debug"
          class="skill-item__tag skill-item__tag--debug"
        >{{ t('debug.localDebug') }}</span>
        <span
          v-if="skill.is_debug && skill.debug_status === 'abnormal'"
          class="skill-item__tag skill-item__tag--abnormal"
        >{{ t('debug.abnormal') }}</span>
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

.skill-item__editor-toggles {
  display: flex;
  gap: 0.3rem;
  flex-shrink: 0;
}

.skill-item__editor-btn {
  width: 24px;
  height: 20px;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  font-size: 0.6rem;
  font-weight: 700;
  font-family: inherit;
  border: none;
  border-radius: 5px;
  cursor: pointer;
  background: var(--neu-bg);
  color: var(--neu-text-muted);
  box-shadow: var(--neu-shadow-in-sm);
  transition: all 200ms ease;
  padding: 0;
  line-height: 1;
}

.skill-item__editor-btn--on {
  background: var(--neu-accent);
  color: #fff;
  box-shadow: 0 1px 3px rgba(232, 115, 74, 0.3);
}

.skill-item__editor-btn:hover:not(:disabled) {
  opacity: 0.85;
}

.skill-item__editor-btn:disabled {
  opacity: 0.4;
  cursor: not-allowed;
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

.skill-item__tag--version {
  background: rgba(72, 187, 120, 0.14);
  color: #2f855a;
}

.skill-item__tag--debug {
  background: rgba(124, 58, 237, 0.12);
  color: #6d28d9;
}

.skill-item__tag--abnormal {
  background: rgba(245, 158, 11, 0.14);
  color: #b45309;
}
</style>
