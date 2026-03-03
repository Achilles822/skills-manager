<script setup lang="ts">
import { ref, watch, computed } from "vue"
import { useI18n } from "vue-i18n"
import EditorFilter from "@/components/EditorFilter.vue"
import StatusFilter from "@/components/StatusFilter.vue"
import type { StatusFilterValue } from "@/components/StatusFilter.vue"
import SkillList from "@/components/SkillList.vue"
import SkillDetail from "@/components/SkillDetail.vue"
import AboutDialog from "@/components/AboutDialog.vue"
import SettingsDialog from "@/components/SettingsDialog.vue"
import { useEditorFilter } from "@/composables/useEditorFilter"
import { useSkills } from "@/composables/useSkills"
import { useSkillToggle } from "@/composables/useSkillToggle"

const { t } = useI18n()

const editorFilter = useEditorFilter()
const { editors, selectedEditorIds } = editorFilter
const statusFilter = ref<StatusFilterValue>("all")
const { skills, loading, selectedSkill, selectSkill, refresh } = useSkills(selectedEditorIds)
const { toggleSkill, togglingSkills } = useSkillToggle(selectedEditorIds, refresh)

const showAbout = ref(false)
const showSettings = ref(false)

const filteredSkills = computed(() => {
  if (statusFilter.value === "all") return skills.value
  const wantEnabled = statusFilter.value === "enabled"
  return skills.value.filter((s) => s.enabled === wantEnabled)
})

function handleUninstall() {
  selectSkill(null)
  refresh()
}

function updateSelectedEditors(v: string[]) {
  selectedEditorIds.value = v
}

watch(selectedEditorIds, () => {
  selectSkill(null)
})
</script>

<template>
  <div class="app-layout">
    <aside class="app-layout__left">
      <div class="app-layout__top-bar">
        <span class="app-layout__brand" @click="showAbout = true" style="cursor: pointer;">{{ t('app.brand') }}</span>
        <div class="app-layout__top-actions">
        <button class="app-layout__icon-btn" :title="t('filter.refresh')" @click="refresh">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <polyline points="23 4 23 10 17 10" />
            <polyline points="1 20 1 14 7 14" />
            <path d="M3.51 9a9 9 0 0 1 14.85-3.36L23 10M1 14l4.64 4.36A9 9 0 0 0 20.49 15" />
          </svg>
        </button>
        <button class="app-layout__icon-btn" :title="t('settings.title')" @click="showSettings = true">
          <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <circle cx="12" cy="12" r="3" />
            <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06A1.65 1.65 0 0 0 4.68 15a1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06A1.65 1.65 0 0 0 9 4.68a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06A1.65 1.65 0 0 0 19.4 9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z" />
          </svg>
        </button>
        </div>
      </div>
      <div class="app-layout__filters">
        <EditorFilter
          :editors="editors"
          :selected-editor-ids="selectedEditorIds"
          @update:selected-editor-ids="updateSelectedEditors"
        />
        <StatusFilter
          v-model="statusFilter"
        />
      </div>
      <div class="app-layout__list">
        <SkillList
          :skills="filteredSkills"
          :selected-skill="selectedSkill"
          :loading="loading"
          :toggling-skill-ids="togglingSkills"
          @select="selectSkill"
          @toggle="toggleSkill"
        />
      </div>
    </aside>
    <main class="app-layout__right">
      <SkillDetail
        :skill="selectedSkill"
        :toggling-skill-ids="togglingSkills"
        :selected-editor-ids="selectedEditorIds"
        @toggle="toggleSkill"
        @refresh="refresh"
        @uninstall="handleUninstall"
      />
    </main>

    <AboutDialog :visible="showAbout" @close="showAbout = false" />
    <SettingsDialog :visible="showSettings" @close="showSettings = false" />
  </div>
</template>

<style scoped>
.app-layout {
  display: flex;
  height: 100vh;
  overflow: hidden;
}

.app-layout__left {
  width: 30%;
  min-width: 280px;
  max-width: 400px;
  display: flex;
  flex-direction: column;
  padding: 1rem;
  gap: 0.75rem;
  border-right: 1px solid rgba(209, 217, 230, 0.5);
}

.app-layout__top-bar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  flex-shrink: 0;
}

.app-layout__brand {
  font-size: 1rem;
  font-weight: 700;
  color: var(--neu-text);
}

.app-layout__top-actions {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.app-layout__icon-btn {
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--neu-bg);
  border: none;
  border-radius: 10px;
  box-shadow: var(--neu-shadow-out-sm);
  cursor: pointer;
  color: var(--neu-text-muted);
  transition: box-shadow var(--neu-transition), color var(--neu-transition);
}

.app-layout__icon-btn:hover {
  color: var(--neu-accent);
  box-shadow: var(--neu-shadow-out);
}

.app-layout__icon-btn:active {
  box-shadow: var(--neu-shadow-in-sm);
}

.app-layout__filters {
  flex-shrink: 0;
  display: flex;
  flex-direction: column;
  gap: 0.625rem;
}

.app-layout__list {
  flex: 1;
  min-height: 0;
  overflow: visible;
  display: flex;
  flex-direction: column;
}

.app-layout__right {
  flex: 1;
  padding: 1.5rem;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}
</style>
