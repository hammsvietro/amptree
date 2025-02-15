use tauri::State;

use crate::{audio::PlayerController, library::Library};

#[tauri::command]
pub async fn play_audio(
    path: String,
    controller: State<'_, PlayerController>,
    library: State<'_, Library>,
) -> Result<(), String> {
    let result = controller.play_now(path);
    let a = library.scan("/Users/pedrovietro/Downloads").await;
    convert_anyhow_result(result)
}

#[tauri::command]
pub async fn queue(path: String, controller: State<'_, PlayerController>) -> Result<(), String> {
    let result = controller.queue(path);
    convert_anyhow_result(result)
}

#[tauri::command]
pub async fn skip(controller: State<'_, PlayerController>) -> Result<(), String> {
    let result = controller.skip();
    convert_anyhow_result(result)
}

#[tauri::command]
pub async fn pause(controller: State<'_, PlayerController>) -> Result<(), String> {
    let result = controller.pause();
    convert_anyhow_result(result)
}

#[tauri::command]
pub async fn resume(controller: State<'_, PlayerController>) -> Result<(), String> {
    let result = controller.resume();
    convert_anyhow_result(result)
}
#[tauri::command]
pub async fn seek(seconds: usize, controller: State<'_, PlayerController>) -> Result<(), String> {
    let result = controller.seek(seconds);
    convert_anyhow_result(result)
}

#[tauri::command]
pub async fn change_volume(
    volume: f64,
    controller: State<'_, PlayerController>,
) -> Result<(), String> {
    let result = controller.change_volume(volume);
    convert_anyhow_result(result)
}

#[tauri::command]
pub async fn scan_folder(path: String, state: State<'_, PlayerController>) -> Result<(), String> {
    // let result = state.scan_folder(path);
    // convert_anyhow_result(result)
    Ok(())
}

fn convert_anyhow_result(result: anyhow::Result<()>) -> Result<(), String> {
    result.map_err(|e| e.to_string())
}
