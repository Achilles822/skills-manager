use crate::platform::home_dir;
use serde::Serialize;
use std::path::{Path, PathBuf};

pub trait EditorDefinition: Send + Sync {
    fn id(&self) -> &str;
    fn display_name(&self) -> &str;
    fn icon(&self) -> &str;
    fn skills_dir(&self, home: &Path) -> PathBuf;
    fn config_dir(&self, home: &Path) -> PathBuf;
}

pub struct CursorEditor;

impl EditorDefinition for CursorEditor {
    fn id(&self) -> &str {
        "cursor"
    }

    fn display_name(&self) -> &str {
        "Cursor"
    }

    fn icon(&self) -> &str {
        "cursor"
    }

    fn skills_dir(&self, home: &Path) -> PathBuf {
        home.join(".cursor").join("skills")
    }

    fn config_dir(&self, home: &Path) -> PathBuf {
        home.join(".cursor")
    }
}

pub struct ClaudeCodeEditor;

impl EditorDefinition for ClaudeCodeEditor {
    fn id(&self) -> &str {
        "claude-code"
    }

    fn display_name(&self) -> &str {
        "Claude Code"
    }

    fn icon(&self) -> &str {
        "claude-code"
    }

    fn skills_dir(&self, home: &Path) -> PathBuf {
        home.join(".claude").join("skills")
    }

    fn config_dir(&self, home: &Path) -> PathBuf {
        home.join(".claude")
    }
}

pub struct EditorRegistry {
    editors: Vec<Box<dyn EditorDefinition>>,
}

mod path_serde {
    use serde::Serializer;
    use std::path::PathBuf;

    pub fn serialize<S>(path: &PathBuf, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&path.to_string_lossy())
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct EditorInfo {
    pub id: String,
    pub display_name: String,
    pub icon: String,
    #[serde(serialize_with = "path_serde::serialize")]
    pub skills_dir: PathBuf,
    pub installed: bool,
}

impl EditorRegistry {
    pub fn new() -> Self {
        let editors: Vec<Box<dyn EditorDefinition>> = vec![
            Box::new(CursorEditor),
            Box::new(ClaudeCodeEditor),
        ];
        Self { editors }
    }

    pub fn get(&self, id: &str) -> Option<&dyn EditorDefinition> {
        self.editors.iter().find(|e| e.id() == id).map(|e| e.as_ref())
    }

    pub fn all(&self) -> &[Box<dyn EditorDefinition>] {
        &self.editors
    }

    pub fn detect_installed(&self) -> Vec<EditorInfo> {
        let home = home_dir();
        if home.is_none() {
            return vec![];
        }
        let home = home.unwrap();

        self.editors
            .iter()
            .map(|e| {
                let skills_dir = e.skills_dir(&home);
                let config_dir = e.config_dir(&home);
                let installed = config_dir.exists() || skills_dir.exists();

                EditorInfo {
                    id: e.id().to_string(),
                    display_name: e.display_name().to_string(),
                    icon: e.icon().to_string(),
                    skills_dir,
                    installed,
                }
            })
            .collect()
    }
}

