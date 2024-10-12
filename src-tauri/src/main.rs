// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use audio::boot_player;
use database::get_connection;
use tauri::Manager;

pub(crate) mod audio;
pub(crate) mod commands;
pub(crate) mod database;
pub(crate) mod event;
pub(crate) mod library;

fn main() -> anyhow::Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();

    let db_connection = get_connection()?;
    let library = library::Library::new(db_connection);
    tauri::Builder::default()
        .setup(move |app| {
            let app_handle = app.handle();
            let player_controller = boot_player(tx.clone(), rx, app_handle)?;
            app.manage(player_controller);
            Ok(())
        })
        .manage(library)
        .invoke_handler(tauri::generate_handler![
            commands::play_audio,
            commands::queue,
            commands::skip,
            commands::pause,
            commands::resume,
            commands::seek,
            commands::change_volume,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
