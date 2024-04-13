// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use player::{boot_player, PlayerController};
use tauri::Manager;

pub(crate) mod audio;
pub(crate) mod commands;
pub(crate) mod player;
pub(crate) mod track;

fn main() -> anyhow::Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();

    tauri::Builder::default()
        .setup(move |app| {
            let app_handle = app.handle();
            let player_controller = boot_player(tx.clone(), rx, app_handle)?;
            app.manage(player_controller);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::play_audio,
            commands::pause,
            commands::resume,
            commands::seek
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
