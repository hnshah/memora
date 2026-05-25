// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use memora::AppState;
use tauri::Manager;

mod lib;

fn main() {
    tauri::Builder::default()
        .manage(AppState::default())
        .setup(|app| {
            println!("Memora backend starting... (LanceDB + Ollama ready)");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            memora::initialize_memory,
            memora::index_conversation,
            memora::search_memory,
            memora::get_memory_stats
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}