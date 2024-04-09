// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use player::{boot_player, PlayerCommand, PlayerController};

pub(crate) mod audio;
pub(crate) mod commands;
pub(crate) mod player;
pub(crate) mod track;

fn main() -> anyhow::Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();

    let (player_controller, thread_handle) = boot_player(tx.clone(), rx)?;

    tauri::Builder::default()
        .manage(player_controller)
        .invoke_handler(tauri::generate_handler![
            commands::play_audio,
            commands::pause,
            commands::resume,
            commands::seek
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    tx.send(PlayerCommand::Shutdown)?;
    let _ = thread_handle.join();
    Ok(())
}
