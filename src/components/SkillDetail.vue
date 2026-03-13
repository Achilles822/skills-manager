<script setup lang="ts">
import { ref, watch, computed, toRef } from "vue"
import { useI18n } from "vue-i18n"
import { invoke } from "@tauri-apps/api/core"
import NeuTag from "@/components/NeuTag.vue"
import NeuToggle from "@/components/NeuToggle.vue"
import ConfirmDialog from "@/components/ConfirmDialog.vue"
import SkillFileTree from "@/components/SkillFileTree.vue"
import FileViewer from "@/components/FileViewer.vue"
import { useSkillFiles } from "@/composables/useSkillFiles"
import type { FileEntry } from "@/composables/useSkillFiles"
import type { Skill, EditorInfo } from "@/types"

const { t } = useI18n()

const props = defineProps<{
  skill: Skill | null
  editors: EditorInfo[]
  toggling: boolean
}>()

const emit = defineEmits<{
  toggleEditor: [editorId: string]
  refresh: []
  uninstall: [skill: Skill]
}>()

const skillRef = toRef(props, "skill")
const {
  fileTree,
  selectedFile,
  fileContent,
  loadingFile,
  hasMultipleFiles,
  selectFile,
  saveFile,
} = useSkillFiles(skillRef)

const isEditing = ref(false)
const treeCollapsed = ref(false)

const isSymlink = computed(() =>
  props.skill
    ? String(props.skill.install_mode).toLowerCase() === "symlink"
    : false
)

function handleEditToggle(v: boolean) {
  isEditing.value = v
}

async function handleFileSave(filePath: string, content: string) {
  const ok = await saveFile(filePath, content)
  if (ok) {
    if (selectedFile.value) {
      selectFile(selectedFile.value)
    }
    emit("refresh")
  }
}

function handleCancelEdit() {
  isEditing.value = false
}

function handleFileSelect(entry: FileEntry) {
  selectFile(entry)
}

watch(() => props.skill?.id, () => {
  isEditing.value = false
  treeCollapsed.value = false
})

const showUninstallConfirm = ref(false)
const uninstalling = ref(false)

function requestUninstall() {
  showUninstallConfirm.value = true
}

const uninstallMessage = computed(() => {
  if (!props.skill) return ""
  if (props.skill.is_debug) {
    return props.skill.debug_status === "abnormal"
      ? t("skill.uninstallDebugAbnormalMessage", { name: props.skill.meta.name })
      : t("skill.uninstallDebugMessage", { name: props.skill.meta.name })
  }
  return t("skill.uninstallMessage", { name: props.skill.meta.name })
})

async function confirmUninstall() {
  if (!props.skill) return
  showUninstallConfirm.value = false
  uninstalling.value = true
  try {
    if (props.skill.is_debug) {
      await invoke("uninstall_debug_skill", {
        dirName: props.skill.dir_name,
        debugStatus: props.skill.debug_status || "abnormal",
      })
    } else {
      await invoke("uninstall_skill", {
        skillId: props.skill.id,
        dirName: props.skill.dir_name,
        sourcePath: typeof props.skill.source_path === "string"
          ? props.skill.source_path
          : String(props.skill.source_path),
      })
    }
    emit("uninstall", props.skill)
  } catch (err) {
    console.error("Failed to uninstall skill:", err)
  } finally {
    uninstalling.value = false
  }
}

async function openInExplorer() {
  if (!props.skill) return
  const path = typeof props.skill.source_path === "string"
    ? props.skill.source_path
    : String(props.skill.source_path)
  try {
    await invoke("open_in_explorer", { path })
  } catch (err) {
    console.error("Failed to open in explorer:", err)
  }
}

const showTree = computed(() => hasMultipleFiles.value && !treeCollapsed.value)
const isCenterSkill = computed(() => props.skill?.id.startsWith("center:") ?? false)
const installedEditors = computed(() => props.editors.filter((e) => e.installed))
</script>

<template>
  <div class="detail">
    <div v-if="!skill" class="detail__placeholder">
      <p>{{ t('skill.selectPrompt') }}</p>
    </div>

    <template v-else>
      <header class="detail__header">
        <div class="detail__title-left">
          <h2 class="detail__name">{{ skill.meta.name }}</h2>
          <NeuTag :variant="isSymlink ? 'symlink' : 'copy'" />
        </div>
        <div class="detail__toolbar">
          <button class="detail__tool-btn" :title="t('skill.openFolder')" @click="openInExplorer">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/>
            </svg>
          </button>
          <button class="detail__tool-btn detail__tool-btn--danger" :title="t('skill.uninstall')" :disabled="uninstalling" @click="requestUninstall">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <polyline points="3 6 5 6 21 6"/>
              <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"/>
            </svg>
          </button>
          <span class="detail__toolbar-sep" />
          <span class="detail__edit-label">{{ t('skill.edit') }}</span>
          <NeuToggle :model-value="isEditing" @update:model-value="handleEditToggle" />
        </div>
      </header>

      <div class="detail__info">
        <dl class="detail__grid">
          <template v-if="skill.meta.version">
            <dt>{{ t('skill.version') }}</dt>
            <dd>{{ skill.meta.version }}</dd>
          </template>
          <template v-if="skill.meta.author">
            <dt>{{ t('skill.author') }}</dt>
            <dd>{{ skill.meta.author }}</dd>
          </template>
          <template v-if="skill.meta.license">
            <dt>{{ t('skill.license') }}</dt>
            <dd>{{ skill.meta.license }}</dd>
          </template>
          <dt>{{ t('skill.editors') }}</dt>
          <dd v-if="isCenterSkill" class="detail__editors">
            <label
              v-for="ed in installedEditors"
              :key="ed.id"
              class="detail__editor-check"
            >
              <input
                type="checkbox"
                :checked="skill.editors.includes(ed.id)"
                :disabled="toggling"
                @change="emit('toggleEditor', ed.id)"
              />
              <span>{{ ed.display_name }}</span>
            </label>
          </dd>
          <dd v-else>{{ skill.editors.join(", ") || "—" }}</dd>
          <dt>{{ t('skill.path') }}</dt>
          <dd class="detail__path">{{ skill.source_path }}</dd>
          <template v-if="skill.is_debug && skill.debug_source_path">
            <dt>{{ t('skill.actualPath') }}</dt>
            <dd class="detail__path">{{ skill.debug_source_path }}</dd>
          </template>
        </dl>
      </div>

      <ConfirmDialog
        :visible="showUninstallConfirm"
        :title="t('skill.confirmUninstall')"
        :message="uninstallMessage"
        :confirm-text="t('skill.uninstall')"
        :cancel-text="t('skill.cancel')"
        danger
        @confirm="confirmUninstall"
        @cancel="showUninstallConfirm = false"
      />

      <div v-if="skill.is_debug && skill.debug_status === 'abnormal'" class="detail__warning detail__warning--abnormal">
        {{ t('debug.abnormalWarning') }}
      </div>

      <div v-if="isSymlink && isEditing" class="detail__warning">
        {{ t('skill.symlinkWarning') }}
      </div>

      <div class="detail__body">
        <div v-if="showTree" class="detail__tree-panel">
          <SkillFileTree
            :entries="fileTree"
            :selected-path="selectedFile?.path || null"
            @select="handleFileSelect"
          />
        </div>

        <div v-if="hasMultipleFiles" class="detail__divider-col">
          <div class="detail__divider-line" />
          <button
            class="detail__collapse-btn"
            :title="treeCollapsed ? t('skill.expandTree') : t('skill.collapseTree')"
            @click="treeCollapsed = !treeCollapsed"
          >
            <svg width="12" height="12" viewBox="0 0 12 12" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
              <path v-if="treeCollapsed" d="M4.5 2l4 4-4 4" />
              <path v-else d="M7.5 2L3.5 6l4 4" />
            </svg>
          </button>
          <div class="detail__divider-line" />
        </div>

        <div class="detail__viewer-panel">
          <div v-if="selectedFile" class="detail__file-name">
            {{ selectedFile.name }}
          </div>
          <FileViewer
            :file-name="selectedFile?.name || null"
            :file-path="selectedFile?.path || null"
            :file-content="fileContent"
            :loading="loadingFile"
            :is-editing="isEditing"
            @save="handleFileSave"
            @cancel-edit="handleCancelEdit"
          />
        </div>
      </div>
    </template>
  </div>
</template>

<style scoped>
.detail {
  display: flex;
  flex-direction: column;
  height: 100%;
  overflow: hidden;
}

.detail__placeholder {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--neu-text-muted);
  font-size: 0.95rem;
}

.detail__header {
  flex-shrink: 0;
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 0.75rem;
  margin-bottom: 0.75rem;
}

.detail__title-left {
  display: flex;
  align-items: center;
  gap: 0.6rem;
  min-width: 0;
}

.detail__name {
  margin: 0;
  font-size: 1.2rem;
  font-weight: 700;
  color: var(--neu-text);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.detail__toolbar {
  display: flex;
  align-items: center;
  gap: 0.4rem;
  flex-shrink: 0;
}

.detail__tool-btn {
  width: 30px;
  height: 30px;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  background: var(--neu-bg);
  border: none;
  border-radius: 8px;
  box-shadow: var(--neu-shadow-out-sm);
  cursor: pointer;
  color: var(--neu-text-muted);
  transition: box-shadow var(--neu-transition), color var(--neu-transition);
}

.detail__tool-btn:hover:not(:disabled) {
  color: var(--neu-accent);
  box-shadow: var(--neu-shadow-out);
}

.detail__tool-btn:active:not(:disabled) {
  box-shadow: var(--neu-shadow-in-sm);
}

.detail__tool-btn--danger:hover:not(:disabled) {
  color: #e53e3e;
}

.detail__tool-btn:disabled {
  opacity: 0.4;
  cursor: not-allowed;
}

.detail__toolbar-sep {
  width: 1px;
  height: 18px;
  background: var(--neu-dark);
  opacity: 0.5;
  margin: 0 0.2rem;
}

.detail__edit-label {
  font-size: 0.8rem;
  font-weight: 500;
  color: var(--neu-text-muted);
  white-space: nowrap;
}

.detail__info {
  flex-shrink: 0;
  margin-bottom: 0.75rem;
}

.detail__grid {
  display: grid;
  grid-template-columns: auto 1fr;
  gap: 0.2rem 1.25rem;
  font-size: 0.85rem;
  margin: 0;
}

.detail__grid dt {
  color: var(--neu-text-muted);
  font-weight: 500;
}

.detail__grid dd {
  margin: 0;
}

.detail__path {
  word-break: break-all;
  font-size: 0.8rem;
}

.detail__warning {
  padding: 0.6rem 0.9rem;
  background: rgba(245, 158, 11, 0.12);
  border-radius: var(--neu-radius-sm);
  font-size: 0.82rem;
  color: #b45309;
  margin-bottom: 0.75rem;
  flex-shrink: 0;
}

.detail__warning--abnormal {
  background: rgba(229, 62, 62, 0.1);
  color: #c53030;
}

.detail__body {
  flex: 1;
  display: flex;
  min-height: 0;
  overflow: hidden;
  gap: 0;
}

.detail__tree-panel {
  width: 200px;
  min-width: 160px;
  flex-shrink: 0;
  overflow-y: auto;
  overflow-x: hidden;
  padding: 4px 6px 4px 4px;
}

.detail__divider-col {
  display: flex;
  flex-direction: column;
  align-items: center;
  flex-shrink: 0;
  width: 22px;
}

.detail__divider-line {
  flex: 1;
  width: 1px;
  background: rgba(209, 217, 230, 0.5);
}

.detail__collapse-btn {
  width: 20px;
  height: 20px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--neu-bg);
  border: none;
  border-radius: 6px;
  box-shadow: var(--neu-shadow-out-sm);
  cursor: pointer;
  color: var(--neu-text-muted);
  transition: box-shadow var(--neu-transition), color var(--neu-transition);
  flex-shrink: 0;
  margin: 4px 0;
}

.detail__collapse-btn:hover {
  color: var(--neu-accent);
  box-shadow: var(--neu-shadow-out);
}

.detail__collapse-btn:active {
  box-shadow: var(--neu-shadow-in-sm);
}

.detail__viewer-panel {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.detail__file-name {
  font-size: 0.78rem;
  font-weight: 600;
  color: var(--neu-text-muted);
  padding: 2px 0 6px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  flex-shrink: 0;
}

.detail__editors {
  display: flex;
  gap: 0.75rem;
  margin: 0;
}

.detail__editor-check {
  display: inline-flex;
  align-items: center;
  gap: 0.3rem;
  font-size: 0.85rem;
  cursor: pointer;
}

.detail__editor-check input {
  accent-color: var(--neu-accent);
}
</style>
