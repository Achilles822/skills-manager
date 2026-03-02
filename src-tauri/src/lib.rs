mod editor;
mod platform;
mod skill;
mod toggle;
mod commands;

use commands::AppState;
use editor::EditorRegistry;
use toggle::SkillLockManager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let registry = EditorRegistry::new();
    let lock_manager = SkillLockManager::new();
    let app_state = AppState::new(registry, lock_manager);

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            commands::detect_editors,
            commands::list_skills,
            commands::get_skill_detail,
            commands::toggle_skill,
            commands::uninstall_skill,
            commands::save_skill_content,
            commands::open_in_explorer,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
