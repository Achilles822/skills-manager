mod commands;
mod debug_log;
mod debug_store;
mod editor;
mod platform;
mod skill;
mod toggle;

use commands::AppState;
use editor::EditorRegistry;
use tauri::Manager;
use toggle::SkillLockManager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let registry = EditorRegistry::new();
    let lock_manager = SkillLockManager::new();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(move |app| {
            let app_data_dir = app.path().app_data_dir()?;
            let app_state = AppState::new(registry, lock_manager, app_data_dir);
            app.manage(app_state);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::detect_editors,
            commands::list_skills,
            commands::get_skill_detail,
            commands::toggle_skill,
            commands::uninstall_skill,
            commands::save_skill_content,
            commands::open_in_explorer,
            commands::list_skill_files,
            commands::read_file_content,
            commands::save_file_content,
            commands::scan_external_folder,
            commands::link_debug_skills,
            commands::uninstall_debug_skill,
            commands::get_debug_store,
            commands::update_skill_editors,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
