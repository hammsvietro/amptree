// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use player::{boot_player, PlayerCommand, PlayerController};
use tauri::Manager;

pub(crate) mod audio;
pub(crate) mod commands;
pub(crate) mod player;
pub(crate) mod track;

fn main() -> anyhow::Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();

    let (player_controller, thread_handle) = boot_player(tx.clone(), rx)?;

    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle();
            tauri::async_runtime::spawn(async move {
                loop {
                    std::thread::sleep(std::time::Duration::from_millis(500));
                    app_handle.emit_all("tick", "testUpdate").unwrap();
                }
            });
            Ok(())
        })
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
