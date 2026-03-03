import { ref, watch, type Ref } from "vue"
import { invoke } from "@tauri-apps/api/core"
import type { Skill } from "@/types"

export function useSkills(selectedEditorIds: Ref<string[]>) {
  const skills = ref<Skill[]>([])
  const loading = ref(false)
  const selectedSkill = ref<Skill | null>(null)
  const error = ref<string | null>(null)
  let fetchSeq = 0

  async function fetchSkills() {
    loading.value = true
    error.value = null
    const seq = ++fetchSeq
    try {
      const editorsParam =
        selectedEditorIds.value.length > 0 ? selectedEditorIds.value : null
      const result = (await invoke("list_skills", {
        editors: editorsParam,
      })) as Skill[]

      if (seq !== fetchSeq) return

      skills.value = result

      if (selectedSkill.value) {
        const updated = result.find((s) => s.id === selectedSkill.value!.id)
        selectedSkill.value = updated ?? null
      }
    } catch (err) {
      if (seq !== fetchSeq) return
      console.error("Failed to list skills:", err)
      error.value = err instanceof Error ? err.message : "Failed to load skills"
      skills.value = []
    } finally {
      if (seq === fetchSeq) {
        loading.value = false
      }
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
