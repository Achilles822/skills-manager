import { ref, watch, type Ref } from "vue"
import { invoke } from "@tauri-apps/api/core"
import type { Skill } from "@/types"

export function useSkills(selectedEditorIds: Ref<string[]>) {
  const skills = ref<Skill[]>([])
  const loading = ref(false)
  const selectedSkill = ref<Skill | null>(null)
  const error = ref<string | null>(null)

  async function fetchSkills() {
    loading.value = true
    error.value = null
    try {
      // editors 为空数组时传 null，让后端返回全部 skills（不按编辑器过滤）
      const editorsParam =
        selectedEditorIds.value.length > 0 ? selectedEditorIds.value : null
      const result = (await invoke("list_skills", {
        editors: editorsParam,
      })) as Skill[]
      skills.value = result

      if (selectedSkill.value) {
        const updated = result.find((s) => s.id === selectedSkill.value!.id)
        selectedSkill.value = updated ?? null
      }
    } catch (err) {
      console.error("Failed to list skills:", err)
      error.value = err instanceof Error ? err.message : "Failed to load skills"
      skills.value = []
    } finally {
      loading.value = false
    }
  }

  function selectSkill(skill: Skill | null) {
    selectedSkill.value = skill
  }

  function refresh() {
    return fetchSkills()
  }

  watch(
    selectedEditorIds,
    () => {
      fetchSkills()
    },
    { immediate: true }
  )

  return {
    skills,
    loading,
    selectedSkill,
    error,
    selectSkill,
    refresh,
  }
}
