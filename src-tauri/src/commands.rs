use crate::debug_store::DebugSkillsStore;
use crate::editor::{EditorInfo, EditorRegistry};
use crate::skill::{parse_skill_md, scan_skills, strip_verbatim_prefix, Skill, SkillDetail};
use crate::toggle::{
    disable_skill_center, disable_skill_copy, enable_skill_center, enable_skill_copy,
    remove_dir_or_symlink, uninstall_skill_center, uninstall_skill_copy, SkillLockManager,
};
use crate::platform::home_dir;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tauri::State;

pub struct AppState {
    pub registry: Arc<EditorRegistry>,
    pub lock_manager: Arc<SkillLockManager>,
    pub app_data_dir: PathBuf,
}

impl AppState {
    pub fn new(registry: EditorRegistry, lock_manager: SkillLockManager, app_data_dir: PathBuf) -> Self {
        Self {
            registry: Arc::new(registry),
            lock_manager: Arc::new(lock_manager),
            app_data_dir,
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
    let skills = scan_skills(&state.registry, &state.app_data_dir);
    Ok(match editors {
        Some(ids) => skills
            .into_iter()
            .filter(|s| {
                if !s.enabled {
                    return true;
                }
                if s.id.starts_with("center:") && s.editors.is_empty() {
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
    let raw_content = crate::skill::read_file_lossy(&path)?;
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
    let path_buf = strip_verbatim_prefix(&PathBuf::from(&path));

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

#[derive(Debug, Clone, Serialize)]
pub struct FileEntry {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
    pub children: Option<Vec<FileEntry>>,
}

fn scan_dir_recursive(dir: &Path) -> Vec<FileEntry> {
    let mut entries = Vec::new();
    let Ok(read_dir) = std::fs::read_dir(dir) else {
        return entries;
    };

    let mut items: Vec<_> = read_dir.flatten().collect();
    items.sort_by(|a, b| {
        let a_is_dir = a.path().is_dir();
        let b_is_dir = b.path().is_dir();
        b_is_dir.cmp(&a_is_dir).then_with(|| {
            a.file_name()
                .to_string_lossy()
                .to_lowercase()
                .cmp(&b.file_name().to_string_lossy().to_lowercase())
        })
    });

    for entry in items {
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();

        if name.starts_with('.') || name == "node_modules" {
            continue;
        }

        let is_dir = path.is_dir();
        let children = if is_dir {
            Some(scan_dir_recursive(&path))
        } else {
            None
        };

        entries.push(FileEntry {
            name,
            path: strip_verbatim_prefix(&path).to_string_lossy().to_string(),
            is_dir,
            children,
        });
    }

    entries
}

#[tauri::command]
pub async fn list_skill_files(skill_dir: String) -> Result<Vec<FileEntry>, String> {
    let dir = PathBuf::from(&skill_dir);
    if !dir.exists() || !dir.is_dir() {
        return Err("Skill directory does not exist".to_string());
    }
    Ok(scan_dir_recursive(&dir))
}

#[derive(Debug, Clone, Serialize)]
pub struct FileContent {
    pub content: String,
    pub is_binary: bool,
}

fn is_binary_content(bytes: &[u8]) -> bool {
    let check_len = bytes.len().min(8192);
    bytes[..check_len].iter().any(|&b| b == 0)
}

#[tauri::command]
pub async fn read_file_content(file_path: String) -> Result<FileContent, String> {
    let path = PathBuf::from(&file_path);
    let bytes = std::fs::read(&path)
        .map_err(|e| format!("Failed to read file: {}", e))?;

    if is_binary_content(&bytes) {
        return Ok(FileContent {
            content: String::new(),
            is_binary: true,
        });
    }

    let content = String::from_utf8_lossy(&bytes).to_string();
    Ok(FileContent {
        content,
        is_binary: false,
    })
}

#[tauri::command]
pub async fn save_file_content(file_path: String, content: String) -> Result<(), String> {
    let path = PathBuf::from(&file_path);
    let parent = path.parent().ok_or("Invalid path")?;
    let temp_path = parent.join(".file_temp");

    std::fs::write(&temp_path, &content)
        .map_err(|e| format!("Failed to write content: {}", e))?;

    std::fs::rename(&temp_path, &path).map_err(|e| {
        let _ = std::fs::remove_file(&temp_path);
        format!("Failed to save file: {}", e)
    })?;

    Ok(())
}

#[derive(Debug, Clone, Serialize)]
pub struct ExternalSkillInfo {
    pub dir_name: String,
    pub name: String,
    pub description: Option<String>,
    pub has_conflict: bool,
}

#[tauri::command]
pub async fn scan_external_folder(folder_path: String) -> Result<Vec<ExternalSkillInfo>, String> {
    let folder = strip_verbatim_prefix(&PathBuf::from(&folder_path));
    if !folder.exists() || !folder.is_dir() {
        return Err("Folder does not exist".to_string());
    }

    let home = home_dir().ok_or("Could not determine home directory")?;
    let center_skills_dir = home.join(".agents").join("skills");

    let mut results = Vec::new();
    let entries = std::fs::read_dir(&folder)
        .map_err(|e| format!("Failed to read folder: {}", e))?;

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            let skill_md = path.join("SKILL.md");
            if skill_md.exists() {
                let dir_name = path.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown")
                    .to_string();

                let (name, description) = match parse_skill_md(&skill_md) {
                    Ok((meta, _)) => (meta.name, meta.description),
                    Err(_) => (dir_name.clone(), None),
                };

                let has_conflict = center_skills_dir.join(&dir_name).exists()
                    || std::fs::symlink_metadata(&center_skills_dir.join(&dir_name)).is_ok();

                results.push(ExternalSkillInfo {
                    dir_name,
                    name,
                    description,
                    has_conflict,
                });
            }
        }
    }

    results.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    Ok(results)
}

#[derive(Debug, Deserialize)]
pub struct LinkRequest {
    pub dir_name: String,
    pub overwrite: bool,
}

fn create_symlink_dir(source: &Path, target: &Path) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        std::os::windows::fs::symlink_dir(source, target)
            .map_err(|e| format!("Failed to create symlink: {}", e))?;
    }

    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(source, target)
            .map_err(|e| format!("Failed to create symlink: {}", e))?;
    }

    Ok(())
}

#[tauri::command]
pub async fn link_debug_skills(
    state: State<'_, AppState>,
    folder_path: String,
    skills: Vec<LinkRequest>,
    editor_ids: Vec<String>,
) -> Result<(), String> {
    let home = home_dir().ok_or("Could not determine home directory")?;
    let center_skills_dir = home.join(".agents").join("skills");
    let folder_raw = PathBuf::from(&folder_path);
    let folder = strip_verbatim_prefix(&folder_raw);
    let folder_path_clean = folder.to_string_lossy().to_string();

    if !center_skills_dir.exists() {
        std::fs::create_dir_all(&center_skills_dir)
            .map_err(|e| format!("Failed to create center skills dir: {}", e))?;
    }

    let mut linked_names = Vec::new();

    for req in &skills {
        let source = folder.join(&req.dir_name);
        let target = center_skills_dir.join(&req.dir_name);

        // #region agent log
        crate::debug_log::debug_log("commands.rs:link_debug_skills", "processing skill", serde_json::json!({"dir_name": &req.dir_name, "overwrite": req.overwrite, "source": source.to_string_lossy(), "target": target.to_string_lossy(), "target_exists": target.exists(), "target_meta_ok": std::fs::symlink_metadata(&target).is_ok(), "folder_path_clean": &folder_path_clean}), "A");
        // #endregion

        if target.exists() || std::fs::symlink_metadata(&target).is_ok() {
            if req.overwrite {
                for editor in state.registry.all() {
                    let editor_skills = editor.skills_dir(&home);
                    let old_link = editor_skills.join(&req.dir_name);
                    let old_exists = old_link.exists() || std::fs::symlink_metadata(&old_link).is_ok();
                    // #region agent log
                    crate::debug_log::debug_log("commands.rs:link_debug_skills", "overwrite: removing editor link", serde_json::json!({"editor": editor.id(), "old_link": old_link.to_string_lossy(), "old_exists": old_exists}), "A");
                    // #endregion
                    if old_exists {
                        let _ = remove_dir_or_symlink(&old_link);
                    }
                }
                remove_dir_or_symlink(&target)
                    .map_err(|e| format!("Failed to remove existing '{}': {}", req.dir_name, e))?;
            } else {
                continue;
            }
        }

        create_symlink_dir(&source, &target)?;
        linked_names.push(req.dir_name.clone());

        for editor_id in &editor_ids {
            if let Some(editor) = state.registry.get(editor_id) {
                let editor_skills = editor.skills_dir(&home);
                if !editor_skills.exists() {
                    std::fs::create_dir_all(&editor_skills)
                        .map_err(|e| format!("Failed to create editor skills dir: {}", e))?;
                }
                let link_path = editor_skills.join(&req.dir_name);
                let editor_link_exists = link_path.exists() || std::fs::symlink_metadata(&link_path).is_ok();
                // #region agent log
                crate::debug_log::debug_log("commands.rs:link_debug_skills", "creating editor link", serde_json::json!({"editor_id": editor_id, "link_path": link_path.to_string_lossy(), "already_exists": editor_link_exists, "target": target.to_string_lossy()}), "A");
                // #endregion
                if editor_link_exists {
                    continue;
                }
                create_symlink_dir(&target, &link_path)?;
            }
        }
    }

    if !linked_names.is_empty() {
        let mut store = DebugSkillsStore::load(&state.app_data_dir);
        store.add_linked_skills(&folder_path_clean, &linked_names);
        store.save(&state.app_data_dir)?;
    }

    Ok(())
}

#[tauri::command]
pub async fn update_skill_editors(
    state: State<'_, AppState>,
    dir_name: String,
    editor_ids: Vec<String>,
) -> Result<(), String> {
    let home = home_dir().ok_or("Could not determine home directory")?;
    let center_skills_dir = home.join(".agents").join("skills");
    let center_path = center_skills_dir.join(&dir_name);

    let center_exists = center_path.exists();
    let center_meta_ok = std::fs::symlink_metadata(&center_path).is_ok();
    // #region agent log
    crate::debug_log::debug_log("commands.rs:update_skill_editors", "entry", serde_json::json!({"dir_name": &dir_name, "editor_ids": &editor_ids, "center_path": center_path.to_string_lossy(), "center_exists": center_exists, "center_meta_ok": center_meta_ok}), "E");
    // #endregion
    if !center_exists && !center_meta_ok {
        return Err(format!("Skill '{}' not found in center skills", dir_name));
    }

    for editor in state.registry.all() {
        let editor_skills = editor.skills_dir(&home);
        let link_path = editor_skills.join(&dir_name);
        let should_have = editor_ids.contains(&editor.id().to_string());
        let link_meta_ok = std::fs::symlink_metadata(&link_path).is_ok();
        let exists = link_path.exists() || link_meta_ok;

        // #region agent log
        crate::debug_log::debug_log("commands.rs:update_skill_editors", "editor", serde_json::json!({"editor": editor.id(), "link_path": link_path.to_string_lossy(), "should_have": should_have, "exists": exists, "link_meta_ok": link_meta_ok, "link_exists": link_path.exists()}), "E");
        // #endregion

        if should_have && !exists {
            if !editor_skills.exists() {
                std::fs::create_dir_all(&editor_skills)
                    .map_err(|e| format!("Failed to create editor skills dir: {}", e))?;
            }
            create_symlink_dir(&center_path, &link_path)?;
        } else if should_have && link_meta_ok && !link_path.exists() {
            remove_dir_or_symlink(&link_path)
                .map_err(|e| format!("Failed to remove stale link: {}", e))?;
            if !editor_skills.exists() {
                std::fs::create_dir_all(&editor_skills)
                    .map_err(|e| format!("Failed to create editor skills dir: {}", e))?;
            }
            create_symlink_dir(&center_path, &link_path)?;
        } else if !should_have && exists {
            remove_dir_or_symlink(&link_path)
                .map_err(|e| format!("Failed to remove from editor '{}': {}", editor.id(), e))?;
        }
    }

    Ok(())
}

#[tauri::command]
pub async fn uninstall_debug_skill(
    state: State<'_, AppState>,
    dir_name: String,
    debug_status: String,
) -> Result<(), String> {
    if debug_status == "normal" {
        let home = home_dir().ok_or("Could not determine home directory")?;
        let center_skills_dir = home.join(".agents").join("skills");
        let skill_path = center_skills_dir.join(&dir_name);

        for editor in state.registry.all() {
            let editor_skills = editor.skills_dir(&home);
            let link_path = editor_skills.join(&dir_name);
            if link_path.exists() || std::fs::symlink_metadata(&link_path).is_ok() {
                remove_dir_or_symlink(&link_path)
                    .map_err(|e| format!("Failed to remove from editor '{}': {}", editor.id(), e))?;
            }
        }

        if std::fs::symlink_metadata(&skill_path).is_ok() {
            remove_dir_or_symlink(&skill_path)
                .map_err(|e| format!("Failed to remove symlink: {}", e))?;
        }

        let disabled_dir = home.join(".agents").join(".disabled-skills");
        let disabled_path = disabled_dir.join(&dir_name);
        if std::fs::symlink_metadata(&disabled_path).is_ok() {
            remove_dir_or_symlink(&disabled_path)
                .map_err(|e| format!("Failed to remove disabled symlink: {}", e))?;
        }
    }

    let mut store = DebugSkillsStore::load(&state.app_data_dir);
    store.remove_skill(&dir_name);
    store.save(&state.app_data_dir)?;

    Ok(())
}

#[tauri::command]
pub async fn get_debug_store(
    state: State<'_, AppState>,
) -> Result<DebugSkillsStore, String> {
    Ok(DebugSkillsStore::load(&state.app_data_dir))
}
