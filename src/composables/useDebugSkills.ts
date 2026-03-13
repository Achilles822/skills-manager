import { ref, computed, type Ref } from "vue"
import { invoke } from "@tauri-apps/api/core"
import { open } from "@tauri-apps/plugin-dialog"
import type { Skill, ExternalSkillInfo, LinkRequest } from "@/types"

export function useDebugSkills(
  allSkills: Ref<Skill[]>,
  refreshAll: () => Promise<void>
) {
  const scanning = ref(false)
  const scannedSkills = ref<ExternalSkillInfo[]>([])
  const scannedFolder = ref<string | null>(null)
  const showSelectDialog = ref(false)
  const linking = ref(false)

  const debugSkills = computed(() =>
    allSkills.value.filter((s) => s.is_debug)
  )

  async function pickFolder() {
    const selected = await open({ directory: true, multiple: false })
    if (!selected) return

    const folderPath = typeof selected === "string" ? selected : selected
    scanning.value = true
    scannedFolder.value = folderPath
    try {
      const results = await invoke<ExternalSkillInfo[]>("scan_external_folder", {
        folderPath,
      })
      scannedSkills.value = results
      if (results.length > 0) {
        showSelectDialog.value = true
      }
      return results
    } catch (err) {
      console.error("Failed to scan folder:", err)
      return []
    } finally {
      scanning.value = false
    }
  }

  async function linkSelected(requests: LinkRequest[], editorIds: string[]) {
    if (!scannedFolder.value || requests.length === 0) return
    linking.value = true
    try {
      await invoke("link_debug_skills", {
        folderPath: scannedFolder.value,
        skills: requests,
        editorIds,
      })
      showSelectDialog.value = false
      await refreshAll()
    } catch (err) {
      console.error("Failed to link skills:", err)
      throw err
    } finally {
      linking.value = false
    }
  }

  async function uninstallDebug(skill: Skill) {
    try {
      await invoke("uninstall_debug_skill", {
        dirName: skill.dir_name,
        debugStatus: skill.debug_status || "abnormal",
      })
      await refreshAll()
    } catch (err) {
      console.error("Failed to uninstall debug skill:", err)
      throw err
    }
  }

  return {
    debugSkills,
    scanning,
    scannedSkills,
    scannedFolder,
    showSelectDialog,
    linking,
    pickFolder,
    linkSelected,
    uninstallDebug,
  }
}
