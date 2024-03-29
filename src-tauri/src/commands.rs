use crate::player::PlayerCommand;
use crate::track::Track;
use crate::PlayerController;
use tauri::State;

#[tauri::command]
pub async fn play_audio(path: String, state: State<'_, PlayerController>) -> Result<(), String> {
    if let Err(error) = queue(path, state).await {
        return Err(error.to_string());
    }
    Ok(())
}

#[tauri::command]
pub async fn pause(state: State<'_, PlayerController>) -> Result<(), String> {
    state
        .tx
        .send(PlayerCommand::Pause)
        .expect("Could not pause track");
    Ok(())
}

#[tauri::command]
pub async fn resume(state: State<'_, PlayerController>) -> Result<(), String> {
    state
        .tx
        .send(PlayerCommand::Resume)
        .expect("Could not pause track");
    Ok(())
}

#[tauri::command]
pub async fn seek(seconds: usize, state: State<'_, PlayerController>) -> Result<(), String> {
    if state.tx.send(PlayerCommand::Seek(seconds)).is_err() {
        return Err(format!("Could not skip to {seconds} seconds"));
    }
    Ok(())
}

async fn queue(path: String, state: State<'_, PlayerController>) -> anyhow::Result<()> {
    let track = Track::new(path.to_owned());
    state.tx.send(PlayerCommand::Play(track))?;
    Ok(())
}
