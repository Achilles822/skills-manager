use crate::editor::EditorRegistry;
use crate::platform::home_dir;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum InstallMode {
    Symlink,
    Copy,
}

impl std::fmt::Display for InstallMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InstallMode::Symlink => write!(f, "symlink"),
            InstallMode::Copy => write!(f, "copy"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillMeta {
    pub name: String,
    pub description: Option<String>,
    pub version: Option<String>,
    pub author: Option<String>,
    pub license: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Skill {
    pub id: String,
    pub dir_name: String,
    pub meta: SkillMeta,
    pub install_mode: InstallMode,
    #[serde(serialize_with = "path_serde::serialize")]
    pub source_path: PathBuf,
    pub editors: Vec<String>,
    pub enabled: bool,
    pub raw_content: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct SkillDetail {
    pub meta: SkillMeta,
    pub body: String,
    pub raw_content: String,
}

pub fn parse_skill_md(path: &Path) -> Result<(SkillMeta, String), String> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| format!("Failed to read skill file: {}", e))?;

    let (meta, body) = if content.starts_with("---") {
        let rest = &content[3..];
        if let Some(end_idx) = rest.find("\n---") {
            let front_matter = &rest[..end_idx];
            let body_start = end_idx + 4;
            let body = if body_start < rest.len() {
                rest[body_start..].trim_start_matches('\n').to_string()
            } else {
                String::new()
            };
            let meta = parse_yaml_front_matter(front_matter)?;
            (meta, body)
        } else {
            return Err("Invalid front matter: missing closing ---".to_string());
        }
    } else {
        let meta = SkillMeta {
            name: path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown")
                .to_string(),
            description: None,
            version: None,
            author: None,
            license: None,
        };
        (meta, content)
    };

    Ok((meta, body))
}

fn parse_yaml_front_matter(content: &str) -> Result<SkillMeta, String> {
    let mut name = None;
    let mut description = None;
    let mut version = None;
    let mut author = None;
    let mut license = None;

    let mut metadata_name = None;
    let mut metadata_version = None;
    let mut metadata_author = None;

    let mut in_metadata = false;
    let mut metadata_indent = 0;

    for line in content.lines() {
        let trimmed = line.trim_end();
        if trimmed.is_empty() {
            continue;
        }

        let indent = line.len() - line.trim_start().len();
        let key_value = line.trim();

        if key_value.starts_with("metadata:") {
            in_metadata = true;
            metadata_indent = indent;
            continue;
        }

        if in_metadata && indent <= metadata_indent && !key_value.starts_with(" ") {
            in_metadata = false;
        }

        if let Some((k, v)) = parse_key_value(key_value) {
            let v = v.trim().trim_matches('"').trim_matches('\'').to_string();
            if v.is_empty() || v == "null" {
                continue;
            }

            if in_metadata {
                match k {
                    "name" => metadata_name = Some(v),
                    "version" => metadata_version = Some(v),
                    "author" => metadata_author = Some(v),
                    _ => {}
                }
            } else {
                match k {
                    "name" => name = Some(v),
                    "description" => description = Some(v),
                    "version" => version = Some(v),
                    "author" => author = Some(v),
                    "license" => license = Some(v),
                    _ => {}
                }
            }
        }
    }

    let name = name.or(metadata_name).unwrap_or_else(|| "unknown".to_string());
    let version = version.or(metadata_version);
    let author = author.or(metadata_author);

    Ok(SkillMeta {
        name,
        description,
        version,
        author,
        license,
    })
}

fn parse_key_value(line: &str) -> Option<(&str, &str)> {
    let colon_pos = line.find(':')?;
    let key = line[..colon_pos].trim();
    let value = line[colon_pos + 1..].trim();
    if key.is_empty() {
        None
    } else {
        Some((key, value))
    }
}

pub fn scan_skills(registry: &EditorRegistry) -> Vec<Skill> {
    let home = match home_dir() {
        Some(h) => h,
        None => return vec![],
    };

    let center_skills_dir = home.join(".agents").join("skills");
    let disabled_center_dir = home.join(".agents").join(".disabled-skills");

    let mut skills: std::collections::HashMap<String, Skill> = std::collections::HashMap::new();

    if center_skills_dir.exists() {
        if let Ok(entries) = std::fs::read_dir(&center_skills_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    let skill_md = path.join("SKILL.md");
                    if skill_md.exists() {
                        if let Ok((meta, _)) = parse_skill_md(&skill_md) {
                            let raw_content = std::fs::read_to_string(&skill_md).unwrap_or_default();
                            let name = &meta.name;
                            // Use actual filesystem dir name for symlink operations
                            let dir_name = path.file_name()
                                .and_then(|n| n.to_str())
                                .unwrap_or(name)
                                .to_string();
                            let id = format!("center:{}", name);
                            let mut editors = Vec::new();

                            for editor in registry.all() {
                                let editor_skills = editor.skills_dir(&home);
                                let link_path = editor_skills.join(&dir_name);
                                if let Ok(link_meta) = std::fs::symlink_metadata(&link_path) {
                                    if link_meta.is_symlink() {
                                        if let Ok(target) = std::fs::read_link(&link_path) {
                                            let resolved = link_path
                                                .parent()
                                                .unwrap()
                                                .join(&target)
                                                .canonicalize();
                                            if resolved
                                                .as_ref()
                                                .map(|p| p.starts_with(&center_skills_dir))
                                                .unwrap_or(false)
                                            {
                                                editors.push(editor.id().to_string());
                                            }
                                        }
                                    }
                                }
                            }

                            skills.insert(
                                id.clone(),
                                Skill {
                                    id: id.clone(),
                                    dir_name,
                                    meta,
                                    install_mode: InstallMode::Symlink,
                                    source_path: path.clone(),
                                    editors,
                                    enabled: true,
                                    raw_content,
                                },
                            );
                        }
                    }
                }
            }
        }
    }

    if disabled_center_dir.exists() {
        if let Ok(entries) = std::fs::read_dir(&disabled_center_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    let skill_md = path.join("SKILL.md");
                    if skill_md.exists() {
                        if let Ok((meta, _)) = parse_skill_md(&skill_md) {
                            let raw_content = std::fs::read_to_string(&skill_md).unwrap_or_default();
                            let name = &meta.name;
                            let id = format!("center:{}", name);
                            let dir_name = path.file_name()
                                .and_then(|n| n.to_str())
                                .unwrap_or(name)
                                .to_string();

                            skills.insert(
                                id.clone(),
                                Skill {
                                    id: id.clone(),
                                    dir_name,
                                    meta,
                                    install_mode: InstallMode::Symlink,
                                    source_path: path.clone(),
                                    editors: vec![],
                                    enabled: false,
                                    raw_content,
                                },
                            );
                        }
                    }
                }
            }
        }
    }

    for editor in registry.all() {
        let editor_skills_dir = editor.skills_dir(&home);
        let editor_disabled_dir = editor_skills_dir.join(".disabled");

        let mut scan_dir = |dir: &Path, enabled: bool| {
            if !dir.exists() {
                return;
            }
            if let Ok(entries) = std::fs::read_dir(dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        let skill_md = path.join("SKILL.md");
                        if skill_md.exists() {
                            if let Ok((meta, _)) = parse_skill_md(&skill_md) {
                                let raw_content =
                                    std::fs::read_to_string(&skill_md).unwrap_or_default();
                                let name = &meta.name;

                                let is_symlink = std::fs::symlink_metadata(&path)
                                    .map(|m| m.is_symlink())
                                    .unwrap_or(false);

                                let points_to_center = if is_symlink {
                                    std::fs::read_link(&path)
                                        .ok()
                                        .and_then(|target| {
                                            path.parent()
                                                .map(|p| p.join(&target).canonicalize())
                                        })
                                        .and_then(|r| r.ok())
                                        .map(|resolved| resolved.starts_with(&center_skills_dir))
                                        .unwrap_or(false)
                                } else {
                                    false
                                };

                                let d_name = path.file_name()
                                    .and_then(|n| n.to_str())
                                    .unwrap_or(name)
                                    .to_string();

                                if is_symlink && points_to_center {
                                    let center_id = format!("center:{}", name);
                                    if let Some(skill) = skills.get_mut(&center_id) {
                                        if enabled
                                            && !skill.editors.contains(&editor.id().to_string())
                                        {
                                            skill.editors.push(editor.id().to_string());
                                        }
                                    }
                                } else if is_symlink {
                                    let id = format!("{}:{}", editor.id(), name);
                                    skills.insert(
                                        id.clone(),
                                        Skill {
                                            id,
                                            dir_name: d_name,
                                            meta,
                                            install_mode: InstallMode::Symlink,
                                            source_path: path.clone(),
                                            editors: if enabled {
                                                vec![editor.id().to_string()]
                                            } else {
                                                vec![]
                                            },
                                            enabled,
                                            raw_content,
                                        },
                                    );
                                } else {
                                    let install_mode = InstallMode::Copy;
                                    let id = format!("{}:{}:copy", editor.id(), name);
                                    skills.insert(
                                        id.clone(),
                                        Skill {
                                            id,
                                            dir_name: d_name,
                                            meta,
                                            install_mode,
                                            source_path: path.clone(),
                                            editors: vec![editor.id().to_string()],
                                            enabled,
                                            raw_content,
                                        },
                                    );
                                }
                            }
                        }
                    }
                }
            }
        };

        scan_dir(&editor_skills_dir, true);
        scan_dir(&editor_disabled_dir, false);
    }

    let mut result: Vec<Skill> = skills.into_values().collect();
    result.sort_by(|a, b| a.meta.name.to_lowercase().cmp(&b.meta.name.to_lowercase()));
    result
}
