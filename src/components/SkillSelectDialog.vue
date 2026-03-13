<script setup lang="ts">
import { ref, computed, watch } from "vue"
import { useI18n } from "vue-i18n"
import type { ExternalSkillInfo, LinkRequest, EditorInfo } from "@/types"

const { t } = useI18n()

const props = defineProps<{
  visible: boolean
  skills: ExternalSkillInfo[]
  editors: EditorInfo[]
  loading?: boolean
}>()

const emit = defineEmits<{
  confirm: [requests: LinkRequest[], editorIds: string[]]
  cancel: []
}>()

const selected = ref<Set<string>>(new Set())
const selectedEditors = ref<string[]>([])
const showOverwriteConfirm = ref(false)

const installedEditors = computed(() => props.editors.filter((e) => e.installed))

watch(() => props.visible, (v) => {
  if (v) {
    selected.value = new Set(props.skills.map((s) => s.dir_name))
    selectedEditors.value = installedEditors.value.map((e) => e.id)
    showOverwriteConfirm.value = false
  }
})

const hasConflicts = computed(() =>
  props.skills.some((s) => s.has_conflict && selected.value.has(s.dir_name))
)

const conflictNames = computed(() =>
  props.skills
    .filter((s) => s.has_conflict && selected.value.has(s.dir_name))
    .map((s) => s.name)
)

function toggleSelection(dirName: string) {
  const next = new Set(selected.value)
  if (next.has(dirName)) {
    next.delete(dirName)
  } else {
    next.add(dirName)
  }
  selected.value = next
}

function handleConfirm() {
  if (hasConflicts.value) {
    showOverwriteConfirm.value = true
    return
  }
  doConfirm()
}

function toggleEditorSelection(id: string) {
  const idx = selectedEditors.value.indexOf(id)
  if (idx >= 0) {
    selectedEditors.value = selectedEditors.value.filter((i) => i !== id)
  } else {
    selectedEditors.value = [...selectedEditors.value, id]
  }
}

function doConfirm() {
  const requests: LinkRequest[] = props.skills
    .filter((s) => selected.value.has(s.dir_name))
    .map((s) => ({
      dir_name: s.dir_name,
      overwrite: s.has_conflict,
    }))
  emit("confirm", requests, selectedEditors.value)
}
</script>

<template>
  <Teleport to="body">
    <Transition name="dialog-fade">
      <div v-if="visible" class="dialog-anchor">
        <div class="select-dialog" @click.stop>
          <h3 class="select-dialog__title">{{ t('debug.selectSkills') }}</h3>

          <div v-if="skills.length === 0" class="select-dialog__empty">
            {{ t('debug.noSkillsInFolder') }}
          </div>

          <div v-else class="select-dialog__list">
            <label
              v-for="skill in skills"
              :key="skill.dir_name"
              class="select-dialog__item"
            >
              <input
                type="checkbox"
                :checked="selected.has(skill.dir_name)"
                @change="toggleSelection(skill.dir_name)"
                class="select-dialog__checkbox"
              />
              <span class="select-dialog__name">{{ skill.name }}</span>
              <span v-if="skill.has_conflict" class="select-dialog__conflict-tag">
                {{ t('debug.conflict') }}
              </span>
            </label>
          </div>

          <div v-if="installedEditors.length > 0" class="select-dialog__editors">
            <h4 class="select-dialog__editors-title">{{ t('debug.installTo') }}</h4>
            <div class="select-dialog__editors-list">
              <label
                v-for="ed in installedEditors"
                :key="ed.id"
                class="select-dialog__editor-item"
              >
                <input
                  type="checkbox"
                  :checked="selectedEditors.includes(ed.id)"
                  @change="toggleEditorSelection(ed.id)"
                />
                <span>{{ ed.display_name }}</span>
              </label>
            </div>
          </div>

          <div v-if="showOverwriteConfirm" class="select-dialog__warning">
            <p>{{ t('debug.overwriteConfirm') }}</p>
            <ul>
              <li v-for="name in conflictNames" :key="name">{{ name }}</li>
            </ul>
          </div>

          <div class="select-dialog__actions">
            <button
              class="select-dialog__btn select-dialog__btn--cancel"
              @click="emit('cancel')"
              :disabled="loading"
            >{{ t('confirm.cancel') }}</button>
            <button
              v-if="showOverwriteConfirm"
              class="select-dialog__btn select-dialog__btn--danger"
              @click="doConfirm"
              :disabled="loading || selected.size === 0"
            >{{ loading ? t('debug.importing') : t('confirm.confirm') }}</button>
            <button
              v-else
              class="select-dialog__btn select-dialog__btn--confirm"
              @click="handleConfirm"
              :disabled="loading || selected.size === 0"
            >{{ loading ? t('debug.importing') : t('confirm.confirm') }}</button>
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<style scoped>
.dialog-anchor {
  position: fixed;
  inset: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  pointer-events: none;
}

.select-dialog {
  pointer-events: auto;
  background: var(--neu-bg);
  border-radius: var(--neu-radius-lg);
  box-shadow:
    0 8px 32px rgba(0, 0, 0, 0.18),
    0 2px 8px rgba(0, 0, 0, 0.08);
  padding: 1.5rem 2rem;
  min-width: 380px;
  max-width: 500px;
  max-height: 70vh;
  display: flex;
  flex-direction: column;
}

.select-dialog__title {
  margin: 0 0 1rem;
  font-size: 1rem;
  font-weight: 700;
  color: var(--neu-text);
}

.select-dialog__empty {
  padding: 1.5rem 0;
  text-align: center;
  color: var(--neu-text-muted);
  font-size: 0.88rem;
}

.select-dialog__list {
  overflow-y: auto;
  max-height: 40vh;
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  margin-bottom: 1rem;
}

.select-dialog__item {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.3rem 0;
  cursor: pointer;
}

.select-dialog__checkbox {
  accent-color: var(--neu-accent);
  flex-shrink: 0;
}

.select-dialog__name {
  font-size: 0.88rem;
  font-weight: 600;
  color: var(--neu-text);
}

.select-dialog__conflict-tag {
  display: inline-flex;
  align-items: center;
  padding: 0.05rem 0.4rem;
  font-size: 0.65rem;
  font-weight: 500;
  border-radius: 4px;
  background: rgba(229, 62, 62, 0.12);
  color: #e53e3e;
  flex-shrink: 0;
}

.select-dialog__editors {
  margin-bottom: 1rem;
}

.select-dialog__editors-title {
  margin: 0 0 0.5rem;
  font-size: 0.85rem;
  font-weight: 600;
  color: var(--neu-text);
}

.select-dialog__editors-list {
  display: flex;
  gap: 1rem;
}

.select-dialog__editor-item {
  display: inline-flex;
  align-items: center;
  gap: 0.35rem;
  font-size: 0.85rem;
  cursor: pointer;
}

.select-dialog__editor-item input {
  accent-color: var(--neu-accent);
}

.select-dialog__warning {
  padding: 0.6rem 0.9rem;
  background: rgba(245, 158, 11, 0.12);
  border-radius: var(--neu-radius-sm);
  font-size: 0.82rem;
  color: #b45309;
  margin-bottom: 1rem;
}

.select-dialog__warning p {
  margin: 0 0 0.3rem;
}

.select-dialog__warning ul {
  margin: 0;
  padding-left: 1.2rem;
}

.select-dialog__actions {
  display: flex;
  justify-content: flex-end;
  gap: 0.625rem;
}

.select-dialog__btn {
  padding: 0.45rem 1.2rem;
  font-size: 0.85rem;
  font-weight: 500;
  font-family: inherit;
  border: none;
  border-radius: var(--neu-radius-sm);
  cursor: pointer;
  transition: box-shadow var(--neu-transition), opacity var(--neu-transition);
}

.select-dialog__btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.select-dialog__btn--cancel {
  background: var(--neu-bg);
  color: var(--neu-text);
  box-shadow: var(--neu-shadow-out-sm);
}

.select-dialog__btn--cancel:hover:not(:disabled) {
  box-shadow: var(--neu-shadow-out);
}

.select-dialog__btn--confirm {
  background: var(--neu-accent);
  color: #fff;
  box-shadow: 0 2px 6px rgba(232, 115, 74, 0.3);
}

.select-dialog__btn--confirm:hover:not(:disabled) {
  opacity: 0.9;
}

.select-dialog__btn--danger {
  background: #e53e3e;
  color: #fff;
  box-shadow: 0 2px 6px rgba(229, 62, 62, 0.3);
}

.select-dialog__btn--danger:hover:not(:disabled) {
  opacity: 0.9;
}

.dialog-fade-enter-active,
.dialog-fade-leave-active {
  transition: opacity 150ms ease;
}

.dialog-fade-enter-from,
.dialog-fade-leave-to {
  opacity: 0;
}
</style>
