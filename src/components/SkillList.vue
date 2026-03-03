<script setup lang="ts">
import { computed } from "vue"
import { useI18n } from "vue-i18n"
import SkillListItem from "@/components/SkillListItem.vue"
import type { Skill } from "@/types"

const { t } = useI18n()

const props = defineProps<{
  skills: Skill[]
  selectedSkill: Skill | null
  loading: boolean
  togglingSkillIds: Set<string>
}>()

const emit = defineEmits<{
  select: [skill: Skill]
  toggle: [skill: Skill]
}>()

const isEmpty = computed(() => !props.loading && props.skills.length === 0)
</script>

<template>
  <div class="skill-list">
    <div v-if="loading" class="skill-list__status">
      <span class="skill-list__spinner" aria-hidden="true" />
      <span>{{ t('skill.loadingSkills') }}</span>
    </div>
    <div v-else-if="isEmpty" class="skill-list__status">
      <p>{{ t('skill.noSkillsFound') }}</p>
    </div>
    <div v-else class="skill-list__scroll">
      <SkillListItem
        v-for="skill in skills"
        :key="skill.id"
        :skill="skill"
        :selected="selectedSkill?.id === skill.id"
        :toggling="togglingSkillIds.has(skill.id)"
        @select="emit('select', skill)"
        @toggle="emit('toggle', skill)"
      />
    </div>
  </div>
</template>

<style scoped>
.skill-list {
  flex: 1;
  min-height: 0;
  display: flex;
  flex-direction: column;
}

.skill-list__status {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 0.75rem;
  padding: 2rem 1rem;
  color: var(--neu-text-muted);
  font-size: 0.9rem;
}

.skill-list__spinner {
  width: 18px;
  height: 18px;
  border: 2px solid var(--neu-dark);
  border-top-color: var(--neu-accent);
  border-radius: 50%;
  animation: spin 0.8s linear infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

.skill-list__scroll {
  flex: 1;
  overflow-y: auto;
  overflow-x: hidden;
  display: flex;
  flex-direction: column;
  gap: 0.625rem;
  /* 左右各留 8px 让卡片阴影不被裁切，右侧多留给滚动条 */
  padding: 4px 10px 8px 8px;
}
</style>
