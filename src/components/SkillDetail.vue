<script setup lang="ts">
import { ref, watch, computed } from "vue"
import { invoke } from "@tauri-apps/api/core"
import MarkdownIt from "markdown-it"
import NeuTag from "@/components/NeuTag.vue"
import NeuToggle from "@/components/NeuToggle.vue"
import NeuButton from "@/components/NeuButton.vue"
import ConfirmDialog from "@/components/ConfirmDialog.vue"
import { useSkillEditor } from "@/composables/useSkillEditor"
import type { Skill } from "@/types"

const props = defineProps<{
  skill: Skill | null
  togglingSkillIds: Set<string>
  selectedEditorIds: string[]
}>()

const emit = defineEmits<{
  toggle: [skill: Skill]
  refresh: []
  uninstall: [skill: Skill]
}>()

const md = new MarkdownIt()
const detailBody = ref("")
const detailLoading = ref(false)

const { isEditing, editContent, startEdit, cancelEdit, saveEdit } = useSkillEditor()

const isSymlink = computed(() =>
  props.skill
    ? String(props.skill.install_mode).toLowerCase() === "symlink"
    : false
)

function getSkillMdPath(skill: Skill): string {
  const p = typeof skill.source_path === "string" ? skill.source_path : String(skill.source_path)
  return p.endsWith("SKILL.md") ? p : `${p.replace(/\\/g, "/")}/SKILL.md`
}

async function fetchDetail() {
  if (!props.skill) return
  detailLoading.value = true
  try {
    const result = (await invoke("get_skill_detail", {
      skillPath: getSkillMdPath(props.skill),
    })) as { body: string }
    detailBody.value = result.body
  } catch (err) {
    console.error("Failed to fetch skill detail:", err)
    detailBody.value = props.skill.raw_content
  } finally {
    detailLoading.value = false
  }
}

watch(
  () => props.skill?.id,
  (id) => {
    if (id) fetchDetail()
    else detailBody.value = ""
    cancelEdit()
  }
)

const renderedMarkdown = computed(() => md.render(detailBody.value || ""))

async function handleSave() {
  if (!props.skill) return
  const ok = await saveEdit(getSkillMdPath(props.skill))
  if (ok) {
    await fetchDetail()
    emit("refresh")
  }
}

function handleEditToggle(v: boolean) {
  if (v && props.skill) {
    startEdit(props.skill.raw_content)
  } else {
    cancelEdit()
  }
}

const showUninstallConfirm = ref(false)
const uninstalling = ref(false)

function requestUninstall() {
  showUninstallConfirm.value = true
}

async function confirmUninstall() {
  if (!props.skill) return
  showUninstallConfirm.value = false
  uninstalling.value = true
  try {
    await invoke("uninstall_skill", {
      skillId: props.skill.id,
      dirName: props.skill.dir_name,
      sourcePath: typeof props.skill.source_path === "string"
        ? props.skill.source_path
        : String(props.skill.source_path),
    })
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
</script>

<template>
  <div class="detail">
    <div v-if="!skill" class="detail__placeholder">
      <p>选择一个 Skill 查看详情</p>
    </div>

    <template v-else>
      <header class="detail__header">
        <div class="detail__title-left">
          <h2 class="detail__name">{{ skill.meta.name }}</h2>
          <NeuTag :variant="isSymlink ? 'symlink' : 'copy'" />
          <NeuToggle
            :model-value="skill.enabled"
            :loading="togglingSkillIds.has(skill.id)"
            @update:model-value="emit('toggle', skill)"
          />
        </div>
        <div class="detail__toolbar">
          <button class="detail__tool-btn" title="打开所在目录" @click="openInExplorer">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/>
            </svg>
          </button>
          <button class="detail__tool-btn detail__tool-btn--danger" title="卸载" :disabled="uninstalling" @click="requestUninstall">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <polyline points="3 6 5 6 21 6"/>
              <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"/>
            </svg>
          </button>
          <span class="detail__toolbar-sep" />
          <span class="detail__edit-label">编辑</span>
          <NeuToggle :model-value="isEditing" @update:model-value="handleEditToggle" />
        </div>
      </header>

      <div class="detail__info">
        <dl class="detail__grid">
          <template v-if="skill.meta.version">
            <dt>版本</dt>
            <dd>{{ skill.meta.version }}</dd>
          </template>
          <template v-if="skill.meta.author">
            <dt>作者</dt>
            <dd>{{ skill.meta.author }}</dd>
          </template>
          <template v-if="skill.meta.license">
            <dt>许可证</dt>
            <dd>{{ skill.meta.license }}</dd>
          </template>
          <dt>编辑器</dt>
          <dd>{{ skill.editors.join(", ") || "—" }}</dd>
          <dt>路径</dt>
          <dd class="detail__path">{{ skill.source_path }}</dd>
        </dl>
      </div>

      <ConfirmDialog
        :visible="showUninstallConfirm"
        title="确认卸载"
        :message="`确定要卸载 「${skill.meta.name}」 吗？此操作将永久删除该 Skill 的所有文件，无法撤销。`"
        confirm-text="卸载"
        cancel-text="取消"
        danger
        @confirm="confirmUninstall"
        @cancel="showUninstallConfirm = false"
      />

      <div v-if="isSymlink && isEditing" class="detail__warning">
        此 Skill 为 Symlink 模式，编辑将影响所有链接到此 Skill 的编辑器。
      </div>

      <div v-if="isEditing" class="detail__editor">
        <textarea
          v-model="editContent"
          class="detail__textarea"
          spellcheck="false"
          placeholder="SKILL.md 内容…"
        />
        <div class="detail__editor-actions">
          <NeuButton :disabled="!editContent" @click="handleSave">保存</NeuButton>
          <NeuButton @click="cancelEdit">取消</NeuButton>
        </div>
      </div>

      <div v-else class="detail__content">
        <div v-if="detailLoading" class="detail__loading">加载中…</div>
        <div
          v-else
          class="detail__markdown"
          v-html="renderedMarkdown"
        />
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
}

.detail__editor {
  flex: 1;
  display: flex;
  flex-direction: column;
  min-height: 0;
  overflow: hidden;
}

.detail__textarea {
  flex: 1;
  min-height: 200px;
  padding: 0.875rem;
  font-family: ui-monospace, "Cascadia Code", "Source Code Pro", Menlo, monospace;
  font-size: 0.85rem;
  line-height: 1.5;
  background: var(--neu-bg);
  color: var(--neu-text);
  border: 2px solid transparent;
  border-radius: var(--neu-radius-sm);
  box-shadow: var(--neu-shadow-in);
  resize: vertical;
  transition: border-color var(--neu-transition);
}

.detail__textarea:focus {
  outline: none;
  border-color: var(--neu-accent);
}

.detail__editor-actions {
  display: flex;
  gap: 0.5rem;
  margin-top: 0.5rem;
  padding: 2px;
}

.detail__content {
  flex: 1;
  overflow-y: auto;
  min-height: 0;
}

.detail__loading {
  color: var(--neu-text-muted);
  font-size: 0.9rem;
}

.detail__markdown {
  font-size: 0.88rem;
  line-height: 1.6;
}

.detail__markdown :deep(h1),
.detail__markdown :deep(h2),
.detail__markdown :deep(h3) {
  margin-top: 1.25em;
  margin-bottom: 0.5em;
}

.detail__markdown :deep(p) {
  margin: 0.5em 0;
}

.detail__markdown :deep(code) {
  background: rgba(209, 217, 230, 0.4);
  padding: 0.15em 0.4em;
  border-radius: 4px;
  font-size: 0.88em;
}

.detail__markdown :deep(pre) {
  background: rgba(209, 217, 230, 0.25);
  padding: 0.875rem;
  border-radius: var(--neu-radius-sm);
  overflow-x: auto;
}

.detail__markdown :deep(pre code) {
  background: none;
  padding: 0;
}
</style>
