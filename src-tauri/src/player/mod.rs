use crate::track::Track;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

use crate::audio::{get_device, stream_track};
use crate::track::TrackHandle;
use cpal::{traits::StreamTrait, Device, Stream};

pub struct PlayerController {
    pub tx: Sender<PlayerCommand>,
}

pub enum PlayerCommand {
    Play(Track),
    Resume,
    Pause,
    Seek(usize),
}

pub fn boot_player(rx: Receiver<PlayerCommand>) -> JoinHandle<anyhow::Result<()>> {
    std::thread::spawn(move || run_player(rx))
}

fn run_player_observer(
    track_handle: Arc<Mutex<Option<TrackHandle>>>,
) -> JoinHandle<anyhow::Result<()>> {
    std::thread::spawn(move || loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
        let Ok(mut track_handle_guard) = track_handle.lock() else {
            continue;
        };
        match &mut *track_handle_guard {
            Some(track_handle) => {
                let percentage = (100_f64 * track_handle.get_percentage()).floor();
                println!("played: {percentage}%",);
                println!("status: {:?}", track_handle.get_status());
            }
            None => {}
        }
    })
}

fn run_player(rx: Receiver<PlayerCommand>) -> anyhow::Result<()> {
    let device = get_device()?;
    let mut stream: Option<Stream> = None;
    let current_track_handle: Arc<Mutex<Option<TrackHandle>>> = Arc::new(Mutex::new(None));
    run_player_observer(current_track_handle.clone());
    while let Ok(command) = rx.recv() {
        match command {
            PlayerCommand::Play(track) => {
                if let Ok(mut track_handle_guard) = current_track_handle.lock() {
                    *track_handle_guard = Some(track.get_track_handle()?);
                }
                handle_play_command(&mut stream, &device, &current_track_handle)?;
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
            PlayerCommand::Seek(seconds) => {
                if let Ok(mut track_handle_guard) = current_track_handle.lock() {
                    match &mut *track_handle_guard {
                        Some(track_handle) => track_handle.seek(seconds)?,
                        None => {}
                    };
                }
            }
        };
    }
    Ok(())
}

fn handle_play_command(
    stream: &mut Option<Stream>,
    device: &Device,
    track_handle: &Arc<Mutex<Option<TrackHandle>>>,
) -> anyhow::Result<()> {
    if stream.is_some() {
        *stream = None;
    }
    let track_stream = stream_track(track_handle, device)?;
    track_stream.play()?;
    *stream = Some(track_stream);
    Ok(())
}

// pub struct PlayerHandle {
//     stream: Stream,
//     device: Device,
//     current_track: Option<TrackHandle>,
//     track_queue: Vec<Track>,
// }

// impl PlayerHandle {
//    pub fn new(device: &Device) -> Self {
//    }
// }
