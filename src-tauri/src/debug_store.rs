use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkedSkill {
    pub dir_name: String,
    pub linked_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugFolder {
    pub path: String,
    pub linked_skills: Vec<LinkedSkill>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DebugSkillsStore {
    pub folders: Vec<DebugFolder>,
}

impl DebugSkillsStore {
    pub fn load(app_data_dir: &Path) -> Self {
        let file = app_data_dir.join("debug-skills.json");
        if !file.exists() {
            return Self::default();
        }
        match std::fs::read_to_string(&file) {
            Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
            Err(_) => Self::default(),
        }
    }

    pub fn save(&self, app_data_dir: &Path) -> Result<(), String> {
        if !app_data_dir.exists() {
            std::fs::create_dir_all(app_data_dir)
                .map_err(|e| format!("Failed to create app data dir: {}", e))?;
        }
        let file = app_data_dir.join("debug-skills.json");
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize: {}", e))?;
        std::fs::write(&file, json)
            .map_err(|e| format!("Failed to write debug-skills.json: {}", e))?;
        Ok(())
    }

    pub fn add_linked_skills(
        &mut self,
        folder_path: &str,
        dir_names: &[String],
    ) {
        let now = chrono::Utc::now().to_rfc3339();
        if let Some(folder) = self.folders.iter_mut().find(|f| f.path == folder_path) {
            for dn in dir_names {
                if !folder.linked_skills.iter().any(|s| s.dir_name == *dn) {
                    folder.linked_skills.push(LinkedSkill {
                        dir_name: dn.clone(),
                        linked_at: now.clone(),
                    });
                }
            }
        } else {
            self.folders.push(DebugFolder {
                path: folder_path.to_string(),
                linked_skills: dir_names
                    .iter()
                    .map(|dn| LinkedSkill {
                        dir_name: dn.clone(),
                        linked_at: now.clone(),
                    })
                    .collect(),
            });
        }
    }

    pub fn remove_skill(&mut self, dir_name: &str) {
        for folder in &mut self.folders {
            folder.linked_skills.retain(|s| s.dir_name != dir_name);
        }
        self.folders.retain(|f| !f.linked_skills.is_empty());
    }

    pub fn is_debug_skill(&self, dir_name: &str) -> bool {
        self.folders
            .iter()
            .any(|f| f.linked_skills.iter().any(|s| s.dir_name == dir_name))
    }

    pub fn get_source_folder(&self, dir_name: &str) -> Option<String> {
        for folder in &self.folders {
            if folder.linked_skills.iter().any(|s| s.dir_name == dir_name) {
                return Some(folder.path.clone());
            }
        }
        None
    }

    pub fn all_debug_dir_names(&self) -> Vec<String> {
        self.folders
            .iter()
            .flat_map(|f| f.linked_skills.iter().map(|s| s.dir_name.clone()))
            .collect()
    }
}
