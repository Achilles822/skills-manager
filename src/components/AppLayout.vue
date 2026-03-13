<script setup lang="ts">
import { ref, computed } from "vue"
import { useI18n } from "vue-i18n"
import { invoke } from "@tauri-apps/api/core"
import EditorFilter from "@/components/EditorFilter.vue"
import StatusFilter from "@/components/StatusFilter.vue"
import type { StatusFilterValue } from "@/components/StatusFilter.vue"
import SkillList from "@/components/SkillList.vue"
import SkillDetail from "@/components/SkillDetail.vue"
import SettingsDialog from "@/components/SettingsDialog.vue"
import SkillSelectDialog from "@/components/SkillSelectDialog.vue"
import { useEditorFilter } from "@/composables/useEditorFilter"
import { useSkills } from "@/composables/useSkills"
import { useDebugSkills } from "@/composables/useDebugSkills"
import type { LinkRequest, Skill } from "@/types"

const { t } = useI18n()

const editorFilter = useEditorFilter()
const { editors, selectedEditorIds } = editorFilter
const statusFilter = ref<StatusFilterValue>("all")
const { skills, loading, selectedSkill, selectSkill, refresh } = useSkills(selectedEditorIds)

const installedEditors = computed(() => editors.value.filter((e) => e.installed))

const {
  scanning,
  scannedSkills,
  showSelectDialog,
  linking,
  pickFolder,
  linkSelected,
} = useDebugSkills(skills, refresh)

const showSettings = ref(false)
const searchQuery = ref("")
const filtersExpanded = ref(false)

const filteredSkills = computed(() => {
  let list = skills.value
  if (searchQuery.value.trim()) {
    const q = searchQuery.value.trim().toLowerCase()
    list = list.filter((s) => s.meta.name.toLowerCase().includes(q))
  }
  if (statusFilter.value !== "all") {
    const wantEnabled = statusFilter.value === "enabled"
    list = list.filter((s) => s.enabled === wantEnabled)
  }
  return list
})

const togglingSkills = ref<Set<string>>(new Set())

async function handleToggleEditor(skill: Skill, editorId: string) {
  if (togglingSkills.value.has(skill.id)) return
  togglingSkills.value = new Set([...togglingSkills.value, skill.id])
  try {
    const current = [...skill.editors]
    const idx = current.indexOf(editorId)
    if (idx >= 0) {
      current.splice(idx, 1)
    } else {
      current.push(editorId)
    }
    // #region agent log
    fetch('http://127.0.0.1:7514/ingest/7dab8989-90cd-4e22-9ee6-2f01dd4903b1',{method:'POST',headers:{'Content-Type':'application/json','X-Debug-Session-Id':'1221d1'},body:JSON.stringify({sessionId:'1221d1',location:'AppLayout.vue:handleToggleEditor',message:'before invoke',data:{skillId:skill.id,dirName:skill.dir_name,editorId,currentEditors:skill.editors,newEditors:current,enabled:skill.enabled,isDebug:skill.is_debug,debugStatus:skill.debug_status,sourcePath:skill.source_path},timestamp:Date.now()})}).catch(()=>{});
    // #endregion
    await invoke("update_skill_editors", {
      dirName: skill.dir_name,
      editorIds: current,
    })
    // #region agent log
    fetch('http://127.0.0.1:7514/ingest/7dab8989-90cd-4e22-9ee6-2f01dd4903b1',{method:'POST',headers:{'Content-Type':'application/json','X-Debug-Session-Id':'1221d1'},body:JSON.stringify({sessionId:'1221d1',location:'AppLayout.vue:handleToggleEditor',message:'invoke succeeded',data:{dirName:skill.dir_name,newEditors:current},timestamp:Date.now()})}).catch(()=>{});
    // #endregion
    await refresh()
  } catch (err) {
    // #region agent log
    fetch('http://127.0.0.1:7514/ingest/7dab8989-90cd-4e22-9ee6-2f01dd4903b1',{method:'POST',headers:{'Content-Type':'application/json','X-Debug-Session-Id':'1221d1'},body:JSON.stringify({sessionId:'1221d1',location:'AppLayout.vue:handleToggleEditor',message:'invoke FAILED',data:{dirName:skill.dir_name,error:String(err)},timestamp:Date.now()})}).catch(()=>{});
    // #endregion
    console.error("Failed to toggle editor:", err)
  } finally {
    togglingSkills.value = new Set([...togglingSkills.value].filter((i) => i !== skill.id))
  }
}

function handleUninstall() {
  selectSkill(null)
  refresh()
}

async function handleAddFolder() {
  await pickFolder()
}

async function handleLinkConfirm(requests: LinkRequest[], editorIds: string[]) {
  await linkSelected(requests, editorIds)
}
</script>

<template>
  <div class="app-layout">
    <aside class="app-layout__left">
      <div class="app-layout__top-bar">
        <h1 class="app-layout__brand">Skills Manager</h1>
        <div class="app-layout__top-actions">
          <button class="app-layout__icon-btn" :title="t('debug.addFolder')" :disabled="scanning" @click="handleAddFolder">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <line x1="12" y1="5" x2="12" y2="19" /><line x1="5" y1="12" x2="19" y2="12" />
            </svg>
          </button>
          <button class="app-layout__icon-btn" :title="t('filter.refresh')" @click="refresh">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <polyline points="23 4 23 10 17 10" /><polyline points="1 20 1 14 7 14" />
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

      <div class="app-layout__search-row">
        <div class="app-layout__search-box">
          <svg class="app-layout__search-icon" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <circle cx="11" cy="11" r="8" /><line x1="21" y1="21" x2="16.65" y2="16.65" />
          </svg>
          <input
            v-model="searchQuery"
            class="app-layout__search-input"
            type="text"
            :placeholder="t('filter.search')"
          />
        </div>
        <button
          class="app-layout__filter-toggle"
          :class="{ 'app-layout__filter-toggle--active': filtersExpanded }"
          :title="t('filter.filters')"
          @click="filtersExpanded = !filtersExpanded"
        >
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <polygon points="22 3 2 3 10 12.46 10 19 14 21 14 12.46 22 3" />
          </svg>
        </button>
      </div>

      <div v-if="filtersExpanded" class="app-layout__filters">
        <EditorFilter
          :editors="editors"
          :selected-editor-ids="selectedEditorIds"
          @update:selected-editor-ids="(v: string[]) => selectedEditorIds = v"
        />
        <StatusFilter v-model="statusFilter" />
      </div>

      <div class="app-layout__list">
        <SkillList
          :skills="filteredSkills"
          :selected-skill="selectedSkill"
          :loading="loading"
          :toggling-skill-ids="togglingSkills"
          :installed-editors="installedEditors"
          @select="selectSkill"
          @toggle-editor="handleToggleEditor"
        />
      </div>
    </aside>

    <main class="app-layout__right">
      <SkillDetail
        :skill="selectedSkill"
        :editors="editors"
        :toggling="selectedSkill ? togglingSkills.has(selectedSkill.id) : false"
        @toggle-editor="(editorId: string) => selectedSkill && handleToggleEditor(selectedSkill, editorId)"
        @refresh="refresh"
        @uninstall="handleUninstall"
      />
    </main>

    <SettingsDialog :visible="showSettings" @close="showSettings = false" />
    <SkillSelectDialog
      :visible="showSelectDialog"
      :skills="scannedSkills"
      :editors="editors"
      :loading="linking"
      @confirm="handleLinkConfirm"
      @cancel="showSelectDialog = false"
    />
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
  margin: 0;
  font-size: 0.92rem;
  font-weight: 700;
  color: var(--neu-text);
  letter-spacing: 0.01em;
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

.app-layout__icon-btn:hover:not(:disabled) {
  color: var(--neu-accent);
  box-shadow: var(--neu-shadow-out);
}

.app-layout__icon-btn:active:not(:disabled) {
  box-shadow: var(--neu-shadow-in-sm);
}

.app-layout__icon-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.app-layout__search-row {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  flex-shrink: 0;
}

.app-layout__search-box {
  flex: 1;
  display: flex;
  align-items: center;
  gap: 0.4rem;
  padding: 0.35rem 0.65rem;
  background: var(--neu-bg);
  border-radius: 10px;
  box-shadow: var(--neu-shadow-in-sm);
}

.app-layout__search-icon {
  color: var(--neu-text-muted);
  flex-shrink: 0;
}

.app-layout__search-input {
  flex: 1;
  border: none;
  outline: none;
  background: transparent;
  font-size: 0.82rem;
  font-family: inherit;
  color: var(--neu-text);
  min-width: 0;
}

.app-layout__search-input::placeholder {
  color: var(--neu-text-muted);
  opacity: 0.7;
}

.app-layout__filter-toggle {
  width: 30px;
  height: 30px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--neu-bg);
  border: none;
  border-radius: 8px;
  box-shadow: var(--neu-shadow-out-sm);
  cursor: pointer;
  color: var(--neu-text-muted);
  transition: box-shadow var(--neu-transition), color var(--neu-transition);
  flex-shrink: 0;
}

.app-layout__filter-toggle:hover {
  color: var(--neu-accent);
  box-shadow: var(--neu-shadow-out);
}

.app-layout__filter-toggle--active {
  color: var(--neu-accent);
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
