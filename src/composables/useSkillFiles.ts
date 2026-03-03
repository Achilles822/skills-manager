import { ref, watch, type Ref } from "vue"
import { invoke } from "@tauri-apps/api/core"
import type { Skill } from "@/types"

export interface FileEntry {
  name: string
  path: string
  is_dir: boolean
  children?: FileEntry[]
}

export interface FileContent {
  content: string
  is_binary: boolean
}

export function useSkillFiles(skill: Ref<Skill | null>) {
  const fileTree = ref<FileEntry[]>([])
  const selectedFile = ref<FileEntry | null>(null)
  const fileContent = ref<FileContent | null>(null)
  const loadingTree = ref(false)
  const loadingFile = ref(false)
  const hasMultipleFiles = ref(false)

  function checkMultipleFiles(entries: FileEntry[]): boolean {
    if (entries.length === 0) return false
    if (entries.length > 1) return true
    if (entries.length === 1 && entries[0].is_dir) return true
    return false
  }

  async function loadTree() {
    if (!skill.value) {
      fileTree.value = []
      hasMultipleFiles.value = false
      return
    }
    loadingTree.value = true
    try {
      const sourcePath = typeof skill.value.source_path === "string"
        ? skill.value.source_path
        : String(skill.value.source_path)
      const entries = await invoke<FileEntry[]>("list_skill_files", { skillDir: sourcePath })
      fileTree.value = entries
      hasMultipleFiles.value = checkMultipleFiles(entries)

      const skillMd = entries.find((e) => e.name === "SKILL.md")
      if (skillMd) {
        selectFile(skillMd)
      } else if (entries.length > 0 && !entries[0].is_dir) {
        selectFile(entries[0])
      }
    } catch (err) {
      console.error("Failed to load skill files:", err)
      fileTree.value = []
      hasMultipleFiles.value = false
    } finally {
      loadingTree.value = false
    }
  }

  async function selectFile(entry: FileEntry) {
    if (entry.is_dir) return
    selectedFile.value = entry
    loadingFile.value = true
    try {
      const result = await invoke<FileContent>("read_file_content", { filePath: entry.path })
      fileContent.value = result
    } catch (err) {
      console.error("Failed to read file:", err)
      fileContent.value = null
    } finally {
      loadingFile.value = false
    }
  }

  async function saveFile(filePath: string, content: string): Promise<boolean> {
    try {
      await invoke("save_file_content", { filePath, content })
      return true
    } catch (err) {
      console.error("Failed to save file:", err)
      return false
    }
  }

  watch(() => skill.value?.id, () => {
    selectedFile.value = null
    fileContent.value = null
    loadTree()
  })

  return {
    fileTree,
    selectedFile,
    fileContent,
    loadingTree,
    loadingFile,
    hasMultipleFiles,
    selectFile,
    saveFile,
    loadTree,
  }
}
