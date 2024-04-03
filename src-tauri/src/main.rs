// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use player::{boot_player, PlayerController};

pub(crate) mod audio;
pub(crate) mod commands;
pub(crate) mod player;
pub(crate) mod track;

fn main() {
    let (tx, rx) = std::sync::mpsc::channel();

    let _ = boot_player(tx.clone(), rx);

    tauri::Builder::default()
        .manage(PlayerController { tx })
        .invoke_handler(tauri::generate_handler![
            commands::play_audio,
            commands::pause,
            commands::resume,
            commands::seek
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
