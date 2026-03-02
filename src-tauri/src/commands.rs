use crate::editor::{EditorInfo, EditorRegistry};
use crate::skill::{parse_skill_md, scan_skills, Skill, SkillDetail};
use crate::toggle::{
    disable_skill_center, disable_skill_copy, enable_skill_center, enable_skill_copy,
    uninstall_skill_center, uninstall_skill_copy, SkillLockManager,
};
use crate::platform::home_dir;
use std::path::PathBuf;
use std::sync::Arc;
use tauri::State;

pub struct AppState {
    pub registry: Arc<EditorRegistry>,
    pub lock_manager: Arc<SkillLockManager>,
}

impl AppState {
    pub fn new(registry: EditorRegistry, lock_manager: SkillLockManager) -> Self {
        Self {
            registry: Arc::new(registry),
            lock_manager: Arc::new(lock_manager),
        }
    }
}

#[tauri::command]
pub async fn detect_editors(state: State<'_, AppState>) -> Result<Vec<EditorInfo>, String> {
    Ok(state.registry.detect_installed())
}

#[tauri::command]
pub async fn list_skills(
    state: State<'_, AppState>,
    editors: Option<Vec<String>>,
) -> Result<Vec<Skill>, String> {
    let skills = scan_skills(&state.registry);
    Ok(match editors {
        Some(ids) => skills
            .into_iter()
            .filter(|s| {
                if !s.enabled {
                    return true;
                }
                s.editors.iter().any(|e| ids.contains(e))
            })
            .collect(),
        None => skills,
    })
}

#[tauri::command]
pub async fn get_skill_detail(skill_path: String) -> Result<SkillDetail, String> {
    let path = PathBuf::from(&skill_path);
    let (meta, body) = parse_skill_md(&path)?;
    let raw_content =
        std::fs::read_to_string(&path).map_err(|e| format!("Failed to read skill file: {}", e))?;
    Ok(SkillDetail {
        meta,
        body,
        raw_content,
    })
}

#[tauri::command]
#[allow(unused_variables)]
pub async fn toggle_skill(
    state: State<'_, AppState>,
    skill_id: String,
    dir_name: String,
    enabled: bool,
    install_mode: String,
    source_path: String,
    editor_ids: Vec<String>,
) -> Result<(), String> {
    state
        .lock_manager
        .with_lock(&skill_id, || {
            let home = home_dir().ok_or("Could not determine home directory".to_string())?;
            let is_center = skill_id.starts_with("center:");

            if is_center {
                if enabled {
                    enable_skill_center(&dir_name, &home, &state.registry, &editor_ids)
                } else {
                    disable_skill_center(&dir_name, &home, &state.registry)
                }
            } else {
                let path = PathBuf::from(&source_path);
                let parent = path
                    .parent()
                    .ok_or("Invalid source path".to_string())?;

                let editor_skills_dir = if parent
                    .file_name()
                    .map(|n| n == ".disabled")
                    .unwrap_or(false)
                {
                    parent
                        .parent()
                        .ok_or("Invalid disabled path".to_string())?
                } else {
                    parent
                };

                if enabled {
                    enable_skill_copy(&dir_name, editor_skills_dir)
                } else {
                    disable_skill_copy(&dir_name, editor_skills_dir)
                }
            }
        })
        .await
}

#[tauri::command]
pub async fn uninstall_skill(
    state: State<'_, AppState>,
    skill_id: String,
    dir_name: String,
    source_path: String,
) -> Result<(), String> {
    state
        .lock_manager
        .with_lock(&skill_id, || {
            let home = home_dir().ok_or("Could not determine home directory".to_string())?;
            let is_center = skill_id.starts_with("center:");

            if is_center {
                uninstall_skill_center(&dir_name, &home, &state.registry)
            } else {
                let path = PathBuf::from(&source_path);
                uninstall_skill_copy(&dir_name, &path)
            }
        })
        .await
}

#[tauri::command]
pub async fn save_skill_content(skill_path: String, content: String) -> Result<(), String> {
    let path = PathBuf::from(&skill_path);
    let parent = path.parent().ok_or("Invalid path")?;
    let temp_path = parent.join(".skill_temp.md");

    std::fs::write(&temp_path, &content)
        .map_err(|e| format!("Failed to write content: {}", e))?;

    std::fs::rename(&temp_path, &path).map_err(|e| {
        let _ = std::fs::remove_file(&temp_path);
        format!("Failed to save skill file: {}", e)
    })?;

    Ok(())
}

#[tauri::command]
pub async fn open_in_explorer(path: String) -> Result<(), String> {
    let path_buf = PathBuf::from(&path);

    #[cfg(target_os = "windows")]
    {
        let path_str = path_buf.to_string_lossy().replace('/', "\\");
        let select_arg = format!("/select,{}", path_str);
        let status = std::process::Command::new("explorer")
            .arg(&select_arg)
            .status()
            .map_err(|e| format!("Failed to open explorer: {}", e))?;
        if !status.success() {
            std::process::Command::new("explorer")
                .arg(&path_str)
                .status()
                .map_err(|e| format!("Failed to open explorer: {}", e))?;
        }
    }

    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg("-R")
            .arg(&path_buf)
            .status()
            .map_err(|e| format!("Failed to open Finder: {}", e))?;
    }

    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(path_buf.parent().unwrap_or(&path_buf))
            .status()
            .map_err(|e| format!("Failed to open file manager: {}", e))?;
    }

    Ok(())
}
