import { ref, computed, onMounted } from "vue"
import { invoke } from "@tauri-apps/api/core"
import type { EditorInfo } from "@/types"

export function useEditorFilter() {
  const editors = ref<EditorInfo[]>([])
  const selectedEditorIds = ref<string[]>([])

  const isAllSelected = computed(() => {
    const installed = editors.value.filter((e) => e.installed)
    if (installed.length === 0) return false
    return installed.every((e) => selectedEditorIds.value.includes(e.id))
  })

  function toggleEditor(id: string) {
    const idx = selectedEditorIds.value.indexOf(id)
    if (idx >= 0) {
      selectedEditorIds.value = selectedEditorIds.value.filter((i) => i !== id)
    } else {
      selectedEditorIds.value = [...selectedEditorIds.value, id]
    }
  }

  function toggleAll() {
    const installed = editors.value.filter((e) => e.installed)
    if (isAllSelected.value) {
      selectedEditorIds.value = []
    } else {
      selectedEditorIds.value = installed.map((e) => e.id)
    }
  }

  onMounted(async () => {
    try {
      const result = (await invoke("detect_editors")) as EditorInfo[]
      editors.value = result
      const installed = result.filter((e) => e.installed)
      selectedEditorIds.value = installed.map((e) => e.id)
    } catch (err) {
      console.error("Failed to detect editors:", err)
      editors.value = []
    }
  })

  return {
    editors,
    selectedEditorIds,
    isAllSelected,
    toggleEditor,
    toggleAll,
  }
}
