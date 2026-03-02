import { ref, type Ref } from "vue"
import { invoke } from "@tauri-apps/api/core"
import type { Skill } from "@/types"

export function useSkillToggle(
  selectedEditorIds: Ref<string[]>,
  onToggled?: () => void
) {
  const togglingSkills = ref<Set<string>>(new Set())

  async function toggleSkill(skill: Skill) {
    const id = skill.id
    if (togglingSkills.value.has(id)) return

    togglingSkills.value = new Set([...togglingSkills.value, id])
    try {
      const installMode =
        typeof skill.install_mode === "string"
          ? skill.install_mode.toLowerCase()
          : "symlink"
      const sourcePath =
        typeof skill.source_path === "string"
          ? skill.source_path
          : String(skill.source_path)

      const editorIds = skill.enabled ? skill.editors : selectedEditorIds.value

      await invoke("toggle_skill", {
        skillId: id,
        dirName: skill.dir_name,
        enabled: !skill.enabled,
        installMode: installMode,
        sourcePath: sourcePath,
        editorIds: editorIds,
      })
      onToggled?.()
    } catch (err) {
      console.error("Failed to toggle skill:", err)
    } finally {
      togglingSkills.value = new Set([...togglingSkills.value].filter((i) => i !== id))
    }
  }

  return {
    toggleSkill,
    togglingSkills,
  }
}
