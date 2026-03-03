<script setup lang="ts">
import { ref, watch } from "vue"
import { useI18n } from "vue-i18n"
import { saveLocale } from "@/i18n"

const { t, locale } = useI18n()

const props = defineProps<{
  visible: boolean
}>()

const emit = defineEmits<{
  close: []
}>()

const selectedLocale = ref(locale.value)
const savedLocale = ref(locale.value)

watch(() => props.visible, (v) => {
  if (v) {
    savedLocale.value = locale.value
    selectedLocale.value = locale.value
  }
})

function onLocaleChange(val: string) {
  selectedLocale.value = val
  locale.value = val
}

function handleSave() {
  saveLocale(selectedLocale.value)
  savedLocale.value = selectedLocale.value
  emit("close")
}

function handleClose() {
  locale.value = savedLocale.value
  selectedLocale.value = savedLocale.value
  emit("close")
}

function onOuterClick(e: MouseEvent) {
  if ((e.target as HTMLElement).classList.contains("settings-anchor")) {
    handleClose()
  }
}

const languages = [
  { id: "zh-CN", label: "简体中文" },
  { id: "en", label: "English" },
]
</script>

<template>
  <Teleport to="body">
    <Transition name="settings-fade">
      <div v-if="visible" class="settings-anchor" @click="onOuterClick">
        <div class="settings-dialog" @click.stop>
          <div class="settings-dialog__brand">
            <img src="@/assets/app-icon.png" alt="Skills Manager" width="48" height="48" class="settings-dialog__icon" />
            <div class="settings-dialog__brand-text">
              <h2 class="settings-dialog__title">Skills Manager</h2>
              <span class="settings-dialog__version">v0.1.1</span>
            </div>
          </div>

          <div class="settings-dialog__divider" />

          <div class="settings-dialog__section">
            <label class="settings-dialog__label">{{ t('settings.displayLanguage') }}</label>
            <div class="settings-dialog__lang-pills">
              <button
                v-for="lang in languages"
                :key="lang.id"
                type="button"
                class="settings-dialog__lang-pill"
                :class="{ 'settings-dialog__lang-pill--selected': selectedLocale === lang.id }"
                @click="onLocaleChange(lang.id)"
              >
                {{ lang.label }}
              </button>
            </div>
          </div>

          <div class="settings-dialog__footer">
            <button class="settings-dialog__save" @click="handleSave">
              {{ t('settings.save') }}
            </button>
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<style scoped>
.settings-anchor {
  position: fixed;
  inset: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  pointer-events: none;
}

.settings-dialog {
  pointer-events: auto;
  background: var(--neu-bg);
  border-radius: var(--neu-radius-lg);
  box-shadow:
    0 8px 32px rgba(0, 0, 0, 0.18),
    0 2px 8px rgba(0, 0, 0, 0.08);
  padding: 2rem 2.5rem;
  min-width: 320px;
  max-width: 400px;
  display: flex;
  flex-direction: column;
  gap: 1.25rem;
}

.settings-dialog__brand {
  display: flex;
  align-items: center;
  gap: 0.875rem;
}

.settings-dialog__icon {
  border-radius: 10px;
  display: block;
  flex-shrink: 0;
}

.settings-dialog__brand-text {
  display: flex;
  flex-direction: column;
  gap: 0.1rem;
}

.settings-dialog__title {
  margin: 0;
  font-size: 1.1rem;
  font-weight: 700;
  color: var(--neu-text);
}

.settings-dialog__version {
  font-size: 0.8rem;
  color: var(--neu-text-muted);
}

.settings-dialog__divider {
  width: 100%;
  height: 1px;
  background: var(--neu-dark);
  opacity: 0.35;
}

.settings-dialog__section {
  display: flex;
  flex-direction: column;
  gap: 0.625rem;
}

.settings-dialog__label {
  font-size: 0.8rem;
  font-weight: 600;
  color: var(--neu-text-muted);
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.settings-dialog__lang-pills {
  display: flex;
  gap: 0.5rem;
}

.settings-dialog__lang-pill {
  flex: 1;
  padding: 0.5rem 1rem;
  font-size: 0.85rem;
  font-weight: 500;
  font-family: inherit;
  color: var(--neu-text);
  background: var(--neu-bg);
  border: none;
  border-radius: var(--neu-radius-lg);
  box-shadow: var(--neu-shadow-out-sm);
  cursor: pointer;
  transition: box-shadow var(--neu-transition), color var(--neu-transition);
}

.settings-dialog__lang-pill:hover {
  box-shadow: var(--neu-shadow-out);
}

.settings-dialog__lang-pill--selected {
  box-shadow: var(--neu-shadow-in-sm);
  color: var(--neu-accent);
  font-weight: 600;
}

.settings-dialog__lang-pill:active {
  transform: scale(0.97);
}

.settings-dialog__footer {
  display: flex;
  justify-content: center;
  padding-top: 0.25rem;
}

.settings-dialog__save {
  padding: 0.5rem 2rem;
  font-size: 0.85rem;
  font-weight: 600;
  font-family: inherit;
  color: #fff;
  background: var(--neu-accent);
  border: none;
  border-radius: var(--neu-radius-sm);
  box-shadow: 0 2px 6px rgba(232, 115, 74, 0.3);
  cursor: pointer;
  transition: opacity var(--neu-transition);
}

.settings-dialog__save:hover {
  opacity: 0.9;
}

.settings-dialog__save:active {
  opacity: 0.8;
}

.settings-fade-enter-active,
.settings-fade-leave-active {
  transition: opacity 200ms ease;
}

.settings-fade-enter-from,
.settings-fade-leave-to {
  opacity: 0;
}
</style>
