use crate::editor::EditorRegistry;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;

fn remove_dir_or_symlink(path: &Path) -> std::io::Result<()> {
    let meta = std::fs::symlink_metadata(path)?;

    #[cfg(target_os = "windows")]
    {
        use std::os::windows::fs::MetadataExt;
        const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x400;
        // On Windows, both symlinks and junctions have the reparse point attribute.
        // Use remove_dir (not remove_dir_all) to remove only the link, not its target.
        if meta.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0 {
            return std::fs::remove_dir(path);
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        if meta.is_symlink() {
            return std::fs::remove_file(path);
        }
    }

    if meta.is_dir() {
        std::fs::remove_dir_all(path)
    } else {
        std::fs::remove_file(path)
    }
}

pub struct SkillLockManager {
    locks: Arc<Mutex<HashMap<String, Arc<Mutex<()>>>>>,
}

impl SkillLockManager {
    pub fn new() -> Self {
        Self {
            locks: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn with_lock<R, F>(&self, skill_id: &str, f: F) -> R
    where
        F: FnOnce() -> R,
    {
        let mutex = {
            let mut locks = self.locks.lock().await;
            locks
                .entry(skill_id.to_string())
                .or_insert_with(|| Arc::new(Mutex::new(())))
                .clone()
        };
        let _guard = mutex.lock().await;
        f()
    }
}

/// Disable a center skill (.agents/skills/ → .agents/.disabled-skills/)
/// and remove any references (symlink or copy) in editor skill directories.
pub fn disable_skill_center(
    dir_name: &str,
    home: &Path,
    registry: &EditorRegistry,
) -> Result<(), String> {
    let center_skills = home.join(".agents").join("skills");
    let disabled_dir = home.join(".agents").join(".disabled-skills");
    let skill_path = center_skills.join(dir_name);

    if !skill_path.exists() {
        return Err(format!("Skill '{}' not found in center skills", dir_name));
    }

    // Remove from ALL editor skill dirs (whether symlink or copy)
    for editor in registry.all() {
        let editor_skills = editor.skills_dir(home);
        let link_path = editor_skills.join(dir_name);
        if link_path.exists() || std::fs::symlink_metadata(&link_path).is_ok() {
            remove_dir_or_symlink(&link_path)
                .map_err(|e| format!("Failed to remove '{}' from editor '{}': {}", dir_name, editor.id(), e))?;
        }
    }

    if !disabled_dir.exists() {
        std::fs::create_dir_all(&disabled_dir)
            .map_err(|e| format!("Failed to create disabled dir: {}", e))?;
    }

    let dest = disabled_dir.join(dir_name);
    if dest.exists() {
        std::fs::remove_dir_all(&dest)
            .map_err(|e| format!("Failed to clean existing disabled dir: {}", e))?;
    }

    std::fs::rename(&skill_path, &dest)
        .map_err(|e| format!("Failed to move skill to disabled: {}", e))?;

    Ok(())
}

/// Enable a center skill (.agents/.disabled-skills/ → .agents/skills/)
/// and create symlinks in specified editor skill directories.
pub fn enable_skill_center(
    dir_name: &str,
    home: &Path,
    registry: &EditorRegistry,
    editor_ids: &[String],
) -> Result<(), String> {
    let center_skills = home.join(".agents").join("skills");
    let disabled_dir = home.join(".agents").join(".disabled-skills");
    let source = disabled_dir.join(dir_name);

    if !source.exists() {
        return Err(format!("Skill '{}' not found in disabled center skills", dir_name));
    }

    if !center_skills.exists() {
        std::fs::create_dir_all(&center_skills)
            .map_err(|e| format!("Failed to create center skills dir: {}", e))?;
    }

    let dest = center_skills.join(dir_name);
    std::fs::rename(&source, &dest)
        .map_err(|e| format!("Failed to move skill from disabled: {}", e))?;

    for editor_id in editor_ids {
        if let Some(editor) = registry.get(editor_id) {
            let editor_skills = editor.skills_dir(home);
            if !editor_skills.exists() {
                std::fs::create_dir_all(&editor_skills)
                    .map_err(|e| format!("Failed to create editor skills dir: {}", e))?;
            }
            let link_path = editor_skills.join(dir_name);
            if link_path.exists() || std::fs::symlink_metadata(&link_path).is_ok() {
                continue;
            }

            #[cfg(target_os = "windows")]
            {
                std::os::windows::fs::symlink_dir(&dest, &link_path)
                    .map_err(|e| format!("Failed to create symlink for editor '{}': {}", editor_id, e))?;
            }

            #[cfg(unix)]
            {
                std::os::unix::fs::symlink(&dest, &link_path)
                    .map_err(|e| format!("Failed to create symlink for editor '{}': {}", editor_id, e))?;
            }
        }
    }

    Ok(())
}

/// Disable an editor-local (copy) skill by moving it to .disabled subdirectory.
pub fn disable_skill_copy(dir_name: &str, editor_skills_dir: &Path) -> Result<(), String> {
    let skill_path = editor_skills_dir.join(dir_name);
    let disabled_dir = editor_skills_dir.join(".disabled");

    if !skill_path.exists() {
        return Err(format!("Skill '{}' not found in {}", dir_name, editor_skills_dir.display()));
    }

    if !disabled_dir.exists() {
        std::fs::create_dir_all(&disabled_dir)
            .map_err(|e| format!("Failed to create .disabled dir: {}", e))?;
    }

    let dest = disabled_dir.join(dir_name);
    if dest.exists() {
        std::fs::remove_dir_all(&dest)
            .map_err(|e| format!("Failed to clean existing disabled entry: {}", e))?;
    }

    std::fs::rename(&skill_path, &dest)
        .map_err(|e| format!("Failed to move skill to disabled: {}", e))?;

    Ok(())
}

/// Permanently delete a center skill and all its references in editor directories.
pub fn uninstall_skill_center(
    dir_name: &str,
    home: &Path,
    registry: &EditorRegistry,
) -> Result<(), String> {
    for editor in registry.all() {
        let editor_skills = editor.skills_dir(home);
        let link_path = editor_skills.join(dir_name);
        if link_path.exists() || std::fs::symlink_metadata(&link_path).is_ok() {
            remove_dir_or_symlink(&link_path)
                .map_err(|e| format!("Failed to remove from editor '{}': {}", editor.id(), e))?;
        }
    }

    let center_skills = home.join(".agents").join("skills");
    let skill_path = center_skills.join(dir_name);
    if skill_path.exists() {
        std::fs::remove_dir_all(&skill_path)
            .map_err(|e| format!("Failed to delete skill: {}", e))?;
    }

    let disabled_dir = home.join(".agents").join(".disabled-skills");
    let disabled_path = disabled_dir.join(dir_name);
    if disabled_path.exists() {
        std::fs::remove_dir_all(&disabled_path)
            .map_err(|e| format!("Failed to delete disabled skill: {}", e))?;
    }

    Ok(())
}

/// Permanently delete an editor-local (copy) skill.
pub fn uninstall_skill_copy(_dir_name: &str, source_path: &Path) -> Result<(), String> {
    if source_path.exists() {
        std::fs::remove_dir_all(source_path)
            .map_err(|e| format!("Failed to delete skill: {}", e))?;
    }
    Ok(())
}

/// Enable an editor-local (copy) skill by moving it back from .disabled subdirectory.
pub fn enable_skill_copy(dir_name: &str, editor_skills_dir: &Path) -> Result<(), String> {
    let disabled_dir = editor_skills_dir.join(".disabled");
    let source = disabled_dir.join(dir_name);
    let dest = editor_skills_dir.join(dir_name);

    if !source.exists() {
        return Err(format!("Skill '{}' not found in disabled", dir_name));
    }

    std::fs::rename(&source, &dest)
        .map_err(|e| format!("Failed to move skill from disabled: {}", e))?;

    Ok(())
}
