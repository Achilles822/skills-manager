<script setup lang="ts">
defineProps<{
  visible: boolean
}>()

const emit = defineEmits<{
  close: []
}>()

function onOuterClick(e: MouseEvent) {
  if ((e.target as HTMLElement).classList.contains("about-anchor")) {
    emit("close")
  }
}
</script>

<template>
  <Teleport to="body">
    <Transition name="about-fade">
      <div v-if="visible" class="about-anchor" @click="onOuterClick">
        <div class="about-dialog" @click.stop>
          <div class="about-dialog__icon">
            <img src="@/assets/app-icon.png" alt="Skills Manager" width="56" height="56" />
          </div>
          <h2 class="about-dialog__title">Skills Manager</h2>
          <p class="about-dialog__version">v0.1.0</p>
          <p class="about-dialog__desc">AI 编辑器 Skills 可视化管理工具</p>
          <div class="about-dialog__divider" />
          <dl class="about-dialog__info">
            <dt>作者</dt>
            <dd>Leo Yan</dd>
            <dt>许可证</dt>
            <dd>MIT</dd>
          </dl>
          <button class="about-dialog__close" @click="emit('close')">关闭</button>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<style scoped>
.about-anchor {
  position: fixed;
  inset: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  pointer-events: none;
}

.about-dialog {
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
  align-items: center;
  text-align: center;
}

.about-dialog__icon {
  margin-bottom: 0.75rem;
}

.about-dialog__icon img {
  border-radius: 12px;
  display: block;
}

.about-dialog__title {
  margin: 0;
  font-size: 1.25rem;
  font-weight: 700;
  color: var(--neu-text);
}

.about-dialog__version {
  margin: 0.25rem 0 0;
  font-size: 0.85rem;
  color: var(--neu-text-muted);
}

.about-dialog__desc {
  margin: 0.5rem 0 0;
  font-size: 0.85rem;
  color: var(--neu-text-muted);
}

.about-dialog__divider {
  width: 60%;
  height: 1px;
  background: var(--neu-dark);
  margin: 1rem 0;
  opacity: 0.5;
}

.about-dialog__info {
  display: grid;
  grid-template-columns: auto 1fr;
  gap: 0.3rem 1.25rem;
  font-size: 0.85rem;
  text-align: left;
  width: 100%;
  margin: 0;
}

.about-dialog__info dt {
  color: var(--neu-text-muted);
  font-weight: 500;
}

.about-dialog__info dd {
  margin: 0;
  color: var(--neu-text);
}

.about-dialog__close {
  margin-top: 1.25rem;
  padding: 0.5rem 1.5rem;
  font-size: 0.85rem;
  font-weight: 500;
  font-family: inherit;
  color: var(--neu-text);
  background: var(--neu-bg);
  border: none;
  border-radius: var(--neu-radius-sm);
  box-shadow: var(--neu-shadow-out-sm);
  cursor: pointer;
  transition: box-shadow var(--neu-transition);
}

.about-dialog__close:hover {
  box-shadow: var(--neu-shadow-out);
}

.about-dialog__close:active {
  box-shadow: var(--neu-shadow-in-sm);
}

.about-fade-enter-active,
.about-fade-leave-active {
  transition: opacity 200ms ease;
}

.about-fade-enter-from,
.about-fade-leave-to {
  opacity: 0;
}
</style>
