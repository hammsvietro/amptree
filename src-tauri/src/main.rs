// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::mpsc::{self, Receiver, Sender};

use audio::{get_device, stream_track};
use cpal::{traits::StreamTrait, Device, Stream};
use player::PlayerCommand;
use track::Track;

pub(crate) mod audio;
pub(crate) mod player;
pub(crate) mod track;

pub struct PlayerHandle {
    tx: Sender<PlayerCommand>,
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let _ = std::thread::spawn(move || run_player_thread(rx));
    tauri::Builder::default()
        .manage(PlayerHandle { tx })
        .invoke_handler(tauri::generate_handler![
            audio::play_audio,
            audio::pause,
            audio::resume
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn run_player_thread(rx: Receiver<PlayerCommand>) -> anyhow::Result<()> {
    let device = get_device()?;
    let mut stream: Option<Stream> = None;
    while let Ok(command) = rx.recv() {
        match command {
            PlayerCommand::Play(track) => {
                handle_play_command(&mut stream, &device, &track)?;
            }
            PlayerCommand::Pause => {
                if let Some(stream) = &stream {
                    stream.pause()?;
                }
            }
            PlayerCommand::Resume => {
                if let Some(stream) = &stream {
                    stream.play()?;
                }
            }
            _ => todo!(),
        };
    }
    Ok(())
}

fn handle_play_command(
    stream: &mut Option<Stream>,
    device: &Device,
    track: &Track,
) -> anyhow::Result<()> {
    if stream.is_some() {
        *stream = None;
    }
    let track_stream = stream_track(track, &device)?;
    track_stream.play()?;
    *stream = Some(track_stream);
    Ok(())
}
