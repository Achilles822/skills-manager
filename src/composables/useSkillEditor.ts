import { ref } from "vue"
import { invoke } from "@tauri-apps/api/core"

export function useSkillEditor() {
  const isEditing = ref(false)
  const editContent = ref("")

  function startEdit(rawContent: string) {
    editContent.value = rawContent
    isEditing.value = true
  }

  function cancelEdit() {
    isEditing.value = false
    editContent.value = ""
  }

  async function saveEdit(skillPath: string) {
    try {
      await invoke("save_skill_content", {
        skillPath,
        content: editContent.value,
      })
      isEditing.value = false
      editContent.value = ""
      return true
    } catch (err) {
      console.error("Failed to save skill:", err)
      return false
    }
  }

  return {
    isEditing,
    editContent,
    startEdit,
    cancelEdit,
    saveEdit,
  }
}
