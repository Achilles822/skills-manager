use crate::debug_store::DebugSkillsStore;
use crate::editor::EditorRegistry;
use crate::platform::home_dir;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

pub fn strip_verbatim_prefix(p: &Path) -> PathBuf {
    let s = p.to_string_lossy();
    if let Some(stripped) = s.strip_prefix(r"\\?\") {
        PathBuf::from(stripped)
    } else {
        p.to_path_buf()
    }
}

fn is_reparse_point(meta: &std::fs::Metadata) -> bool {
    #[cfg(target_os = "windows")]
    {
        use std::os::windows::fs::MetadataExt;
        const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x400;
        meta.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0
    }
    #[cfg(not(target_os = "windows"))]
    {
        meta.is_symlink()
    }
}

fn path_starts_with_ignoring_verbatim(child: &Path, parent: &Path) -> bool {
    let child_s = child.to_string_lossy();
    let parent_s = parent.to_string_lossy();
    #[cfg(windows)]
    let (child_norm, parent_norm): (String, String) = {
        let c: String = child_s.strip_prefix(r"\\?\").unwrap_or(&*child_s).replace('/', "\\").to_lowercase();
        let p: String = parent_s.strip_prefix(r"\\?\").unwrap_or(&*parent_s).replace('/', "\\").to_lowercase();
        (c, p)
    };
    #[cfg(not(windows))]
    let (child_norm, parent_norm): (String, String) = {
        let c: String = child_s.strip_prefix(r"\\?\").unwrap_or(&*child_s).into();
        let p: String = parent_s.strip_prefix(r"\\?\").unwrap_or(&*parent_s).into();
        (c, p)
    };
    let sep_char = std::path::MAIN_SEPARATOR;
    let parent_trimmed = parent_norm.trim_end_matches(sep_char);
    child_norm == parent_trimmed || child_norm.starts_with(&format!("{}{}", parent_trimmed, std::path::MAIN_SEPARATOR))
}

fn link_points_to_center(link_path: &Path, center_raw: &Path, _center_canon: &Path) -> bool {
    let target = match std::fs::read_link(link_path) {
        Ok(t) => t,
        Err(e) => {
            // #region agent log
            crate::debug_log::debug_log("skill.rs:link_points_to_center", "read_link failed", serde_json::json!({"link_path": link_path.to_string_lossy(), "err": e.to_string()}), "C");
            // #endregion
            return false;
        }
    };
    let absolute = if target.is_absolute() {
        target
    } else {
        link_path.parent().unwrap_or(Path::new("")).join(&target)
    };
    let result = path_starts_with_ignoring_verbatim(&absolute, center_raw);
    // #region agent log
    crate::debug_log::debug_log("skill.rs:link_points_to_center", "check", serde_json::json!({"link_path": link_path.to_string_lossy(), "raw_target": std::fs::read_link(link_path).ok().map(|p| p.to_string_lossy().to_string()), "absolute": absolute.to_string_lossy(), "center_raw": center_raw.to_string_lossy(), "result": result}), "C");
    // #endregion
    result
}

mod path_serde {
    use serde::Serializer;
    use std::path::PathBuf;

    pub fn serialize<S>(path: &PathBuf, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = path.to_string_lossy();
        let clean = s.strip_prefix(r"\\?\").unwrap_or(&s);
        serializer.serialize_str(clean)
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
    pub is_debug: bool,
    pub debug_status: Option<String>,
    pub debug_source_path: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SkillDetail {
    pub meta: SkillMeta,
    pub body: String,
    pub raw_content: String,
}

pub fn read_file_lossy(path: &Path) -> Result<String, String> {
    let bytes = std::fs::read(path)
        .map_err(|e| format!("Failed to read file: {}", e))?;
    Ok(String::from_utf8_lossy(&bytes).into_owned())
}

pub fn parse_skill_md(path: &Path) -> Result<(SkillMeta, String), String> {
    let content = read_file_lossy(path)?;

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

fn paths_equivalent(a: &Path, b: &Path) -> bool {
    let a_clean = strip_verbatim_prefix(a);
    let b_clean = strip_verbatim_prefix(b);
    if a_clean == b_clean {
        return true;
    }
    #[cfg(windows)]
    {
        let a_s = a_clean.to_string_lossy().replace('/', "\\");
        let b_s = b_clean.to_string_lossy().replace('/', "\\");
        if a_s.eq_ignore_ascii_case(&b_s) {
            return true;
        }
    }
    if let (Ok(ca), Ok(cb)) = (a.canonicalize(), b.canonicalize()) {
        return strip_verbatim_prefix(&ca) == strip_verbatim_prefix(&cb);
    }
    false
}

fn check_debug_status(path: &Path, dir_name: &str, debug_store: &DebugSkillsStore) -> (bool, Option<String>, Option<String>) {
    if !debug_store.is_debug_skill(dir_name) {
        return (false, None, None);
    }

    let source_folder = debug_store.get_source_folder(dir_name);
    let expected_target = source_folder.as_ref().map(|f| {
        strip_verbatim_prefix(&PathBuf::from(f).join(dir_name))
    });

    let is_link = std::fs::symlink_metadata(path)
        .map(|m| is_reparse_point(&m))
        .unwrap_or(false);

    let status = if !path.exists() && !is_link {
        "abnormal".to_string()
    } else if !is_link {
        "abnormal".to_string()
    } else if let Some(expected) = &expected_target {
        let actual_target = std::fs::read_link(path).ok();
        let resolved = actual_target.map(|t| {
            let abs = if t.is_absolute() { t } else { path.parent().unwrap_or(Path::new("")).join(t) };
            strip_verbatim_prefix(&abs)
        });
        let matches = match &resolved {
            Some(r) => paths_equivalent(r, expected),
            None => false,
        };
        let status_val = if matches { "normal" } else { "abnormal" };
        // #region agent log
        crate::debug_log::debug_log("skill.rs:check_debug_status", "comparison", serde_json::json!({"path": path.to_string_lossy(), "expected": expected.to_string_lossy(), "resolved": resolved.as_ref().map(|p| p.to_string_lossy().to_string()), "matches": matches, "status": status_val}), "D");
        // #endregion
        status_val.to_string()
    } else {
        "normal".to_string()
    };

    let source_path = source_folder.map(|f| {
        let p = strip_verbatim_prefix(&PathBuf::from(&f).join(dir_name));
        p.to_string_lossy().to_string()
    });

    (true, Some(status), source_path)
}

pub fn scan_skills(registry: &EditorRegistry, app_data_dir: &Path) -> Vec<Skill> {
    let home = match home_dir() {
        Some(h) => h,
        None => return vec![],
    };

    let debug_store = DebugSkillsStore::load(app_data_dir);
    let center_skills_dir_raw = home.join(".agents").join("skills");
    let center_skills_dir = center_skills_dir_raw.canonicalize().unwrap_or_else(|_| center_skills_dir_raw.clone());
    let disabled_center_dir = home.join(".agents").join(".disabled-skills");

    let mut skills: std::collections::HashMap<String, Skill> = std::collections::HashMap::new();

    // #region agent log
    crate::debug_log::debug_log("skill.rs:scan_skills", "start", serde_json::json!({"center_raw": center_skills_dir_raw.to_string_lossy(), "center_canon": center_skills_dir.to_string_lossy(), "center_exists": center_skills_dir_raw.exists(), "debug_store_folders": debug_store.folders.len()}), "F");
    // #endregion
    if center_skills_dir_raw.exists() {
        if let Ok(entries) = std::fs::read_dir(&center_skills_dir_raw) {
            for entry in entries.flatten() {
                let path = entry.path();
                let is_dir = path.is_dir();
                let has_skill_md = path.join("SKILL.md").exists();
                let is_symlink_entry = std::fs::symlink_metadata(&path).map(|m| m.is_symlink()).unwrap_or(false);
                // #region agent log
                crate::debug_log::debug_log("skill.rs:scan_skills", "entry", serde_json::json!({"path": path.to_string_lossy(), "is_dir": is_dir, "has_skill_md": has_skill_md, "is_symlink": is_symlink_entry, "file_name": path.file_name().map(|n| n.to_string_lossy().to_string())}), "F");
                // #endregion
                if is_dir {
                    let skill_md = path.join("SKILL.md");
                    if skill_md.exists() {
                        let parse_result = parse_skill_md(&skill_md);
                        // #region agent log
                        if parse_result.is_err() {
                            crate::debug_log::debug_log("skill.rs:scan_skills", "parse_error", serde_json::json!({"path": skill_md.to_string_lossy(), "error": format!("{:?}", parse_result.as_ref().err())}), "F");
                        }
                        // #endregion
                        if let Ok((meta, _)) = parse_result {
                            let raw_content = read_file_lossy(&skill_md).unwrap_or_default();
                            let name = &meta.name;
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
                                    if is_reparse_point(&link_meta)
                                        && link_points_to_center(&link_path, &center_skills_dir_raw, &center_skills_dir)
                                    {
                                        editors.push(editor.id().to_string());
                                    }
                                }
                            }

                            let (is_debug, debug_status, debug_source_path) =
                                check_debug_status(&path, &dir_name, &debug_store);

                            // #region agent log
                            crate::debug_log::debug_log("skill.rs:scan_skills", "center skill", serde_json::json!({"dir_name": &dir_name, "source_path": path.to_string_lossy(), "editors": &editors, "is_debug": is_debug, "debug_status": &debug_status, "is_symlink": std::fs::symlink_metadata(&path).map(|m| is_reparse_point(&m)).unwrap_or(false)}), "B");
                            // #endregion

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
                                    is_debug,
                                    debug_status,
                                    debug_source_path,
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
                            let raw_content = read_file_lossy(&skill_md).unwrap_or_default();
                            let name = &meta.name;
                            let id = format!("center:{}", name);
                            let dir_name = path.file_name()
                                .and_then(|n| n.to_str())
                                .unwrap_or(name)
                                .to_string();

                            let is_debug = debug_store.is_debug_skill(&dir_name);
                            let debug_source_path = debug_store.get_source_folder(&dir_name).map(|f| {
                                let p = strip_verbatim_prefix(&PathBuf::from(&f).join(&dir_name));
                                p.to_string_lossy().to_string()
                            });

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
                                    is_debug,
                                    debug_status: if is_debug { Some("normal".to_string()) } else { None },
                                    debug_source_path,
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
                                    read_file_lossy(&skill_md).unwrap_or_default();
                                let name = &meta.name;

                                let is_symlink = std::fs::symlink_metadata(&path)
                                    .map(|m| is_reparse_point(&m))
                                    .unwrap_or(false);

                                let points_to_center = if is_symlink {
                                    link_points_to_center(&path, &center_skills_dir_raw, &center_skills_dir)
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
                                            is_debug: false,
                                            debug_status: None,
                                            debug_source_path: None,
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
                                            is_debug: false,
                                            debug_status: None,
                                            debug_source_path: None,
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

    // Add entries for debug skills that exist in the store but are missing from filesystem
    for dn in debug_store.all_debug_dir_names() {
        let has_entry = skills.values().any(|s| s.dir_name == dn && s.is_debug);
        if !has_entry {
            let source_folder = debug_store.get_source_folder(&dn);
            let source_skill_path = source_folder.as_ref().map(|f| PathBuf::from(f).join(&dn));
            let meta = source_skill_path
                .as_ref()
                .and_then(|p| parse_skill_md(&p.join("SKILL.md")).ok())
                .map(|(m, _)| m)
                .unwrap_or(SkillMeta {
                    name: dn.clone(),
                    description: None,
                    version: None,
                    author: None,
                    license: None,
                });
            let id = format!("center:{}", meta.name);
            if !skills.contains_key(&id) {
                let raw_content = source_skill_path
                    .as_ref()
                    .and_then(|p| read_file_lossy(&p.join("SKILL.md")).ok())
                    .unwrap_or_default();
                skills.insert(
                    id.clone(),
                    Skill {
                        id,
                        dir_name: dn.clone(),
                        meta,
                        install_mode: InstallMode::Symlink,
                        source_path: center_skills_dir_raw.join(&dn),
                        editors: vec![],
                        enabled: false,
                        raw_content,
                        is_debug: true,
                        debug_status: Some("abnormal".to_string()),
                        debug_source_path: source_folder.map(|f| {
                            let p = strip_verbatim_prefix(&PathBuf::from(&f).join(&dn));
                            p.to_string_lossy().to_string()
                        }),
                    },
                );
            }
        }
    }

    let mut result: Vec<Skill> = skills.into_values().collect();
    result.sort_by(|a, b| a.meta.name.to_lowercase().cmp(&b.meta.name.to_lowercase()));
    result
}
