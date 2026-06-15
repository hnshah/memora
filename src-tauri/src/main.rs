#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;

fn main() {
    tauri::Builder::default()
        .manage(commands::AppState::default())
        .setup(|_app| {
            println!("Memora backend starting...");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::initialize_memory,
            commands::index_conversation,
            commands::search_memory,
            commands::get_memory_stats,
            commands::get_smart_suggestions,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
