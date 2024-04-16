use tauri::State;

use crate::audio::PlayerController;

#[tauri::command]
pub async fn play_audio(path: String, state: State<'_, PlayerController>) -> Result<(), String> {
    let result = state.play_now(path);
    convert_anyhow_result(result)
}

#[tauri::command]
pub async fn queue(path: String, state: State<'_, PlayerController>) -> Result<(), String> {
    let result = state.queue(path);
    convert_anyhow_result(result)
}

#[tauri::command]
pub async fn skip(state: State<'_, PlayerController>) -> Result<(), String> {
    let result = state.skip();
    convert_anyhow_result(result)
}

#[tauri::command]
pub async fn pause(state: State<'_, PlayerController>) -> Result<(), String> {
    let result = state.pause();
    convert_anyhow_result(result)
}

#[tauri::command]
pub async fn resume(state: State<'_, PlayerController>) -> Result<(), String> {
    let result = state.resume();
    convert_anyhow_result(result)
}
#[tauri::command]
pub async fn seek(seconds: usize, state: State<'_, PlayerController>) -> Result<(), String> {
    let result = state.seek(seconds);
    convert_anyhow_result(result)
}

#[tauri::command]
pub async fn change_volume(volume: f64, state: State<'_, PlayerController>) -> Result<(), String> {
    let result = state.change_volume(volume);
    convert_anyhow_result(result)
}

fn convert_anyhow_result(result: anyhow::Result<()>) -> Result<(), String> {
    result.map_err(|e| e.to_string())
}
