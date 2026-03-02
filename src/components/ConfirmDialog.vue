<script setup lang="ts">
defineProps<{
  visible: boolean
  title: string
  message: string
  confirmText?: string
  cancelText?: string
  danger?: boolean
}>()

const emit = defineEmits<{
  confirm: []
  cancel: []
}>()
</script>

<template>
  <Teleport to="body">
    <Transition name="dialog-fade">
      <div v-if="visible" class="dialog-anchor">
        <div class="dialog-panel" @click.stop>
          <h3 class="dialog-panel__title">{{ title }}</h3>
          <p class="dialog-panel__message">{{ message }}</p>
          <div class="dialog-panel__actions">
            <button
              class="dialog-panel__btn dialog-panel__btn--cancel"
              @click="emit('cancel')"
            >{{ cancelText || '取消' }}</button>
            <button
              class="dialog-panel__btn"
              :class="danger ? 'dialog-panel__btn--danger' : 'dialog-panel__btn--confirm'"
              @click="emit('confirm')"
            >{{ confirmText || '确认' }}</button>
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

.dialog-panel {
  pointer-events: auto;
  background: var(--neu-bg);
  border-radius: var(--neu-radius-lg);
  box-shadow:
    0 8px 32px rgba(0, 0, 0, 0.18),
    0 2px 8px rgba(0, 0, 0, 0.08);
  padding: 1.5rem 2rem;
  min-width: 300px;
  max-width: 400px;
}

.dialog-panel__title {
  margin: 0 0 0.5rem;
  font-size: 1rem;
  font-weight: 700;
  color: var(--neu-text);
}

.dialog-panel__message {
  margin: 0 0 1.25rem;
  font-size: 0.88rem;
  color: var(--neu-text-muted);
  line-height: 1.5;
}

.dialog-panel__actions {
  display: flex;
  justify-content: flex-end;
  gap: 0.625rem;
}

.dialog-panel__btn {
  padding: 0.45rem 1.2rem;
  font-size: 0.85rem;
  font-weight: 500;
  font-family: inherit;
  border: none;
  border-radius: var(--neu-radius-sm);
  cursor: pointer;
  transition: box-shadow var(--neu-transition), opacity var(--neu-transition);
}

.dialog-panel__btn--cancel {
  background: var(--neu-bg);
  color: var(--neu-text);
  box-shadow: var(--neu-shadow-out-sm);
}

.dialog-panel__btn--cancel:hover {
  box-shadow: var(--neu-shadow-out);
}

.dialog-panel__btn--cancel:active {
  box-shadow: var(--neu-shadow-in-sm);
}

.dialog-panel__btn--confirm {
  background: var(--neu-accent);
  color: #fff;
  box-shadow: 0 2px 6px rgba(232, 115, 74, 0.3);
}

.dialog-panel__btn--confirm:hover {
  opacity: 0.9;
}

.dialog-panel__btn--danger {
  background: #e53e3e;
  color: #fff;
  box-shadow: 0 2px 6px rgba(229, 62, 62, 0.3);
}

.dialog-panel__btn--danger:hover {
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
