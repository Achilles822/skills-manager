export interface EditorInfo {
  id: string
  display_name: string
  icon: string
  skills_dir: string
  installed: boolean
}

export interface SkillMeta {
  name: string
  description: string | null
  version: string | null
  author: string | null
  license: string | null
}

export interface Skill {
  id: string
  dir_name: string
  meta: SkillMeta
  install_mode: "Symlink" | "Copy" | "symlink" | "copy"
  source_path: string
  editors: string[]
  enabled: boolean
  raw_content: string
}

export interface SkillDetail {
  meta: SkillMeta
  body: string
  raw_content: string
  source_path: string
  install_mode: "Symlink" | "Copy"
  editors: string[]
  enabled: boolean
}
