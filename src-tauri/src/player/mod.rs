use crate::track::Track;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

use crate::audio::{get_device, stream_track};
use crate::track::TrackHandle;
use cpal::{traits::StreamTrait, Device, Stream};

pub enum PlayerCommand {
    PlayNow(Track),
    Resume,
    NextTrack,
    Pause,
    Seek(usize),
}

pub fn boot_player(
    tx: Sender<PlayerCommand>,
    rx: Receiver<PlayerCommand>,
) -> anyhow::Result<(PlayerController, JoinHandle<anyhow::Result<()>>)> {
    let device = get_device()?;
    let player_handle = Arc::new(Mutex::new(PlayerHandle::new(device, tx.clone())));
    let player_handle_clone = player_handle.clone();

    Ok((
        PlayerController {
            player_handle,
            player_command_tx: tx,
        },
        std::thread::spawn(move || run_player(player_handle_clone, rx)),
    ))
}

fn run_player_observer(player_handle: Arc<Mutex<PlayerHandle>>) -> JoinHandle<anyhow::Result<()>> {
    std::thread::spawn(move || loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
        let Ok(player_handle_guard) = player_handle.lock() else {
            continue;
        };
        match player_handle_guard.get_track_handle() {
            Some(track_handle) => {
                let percentage = (100_f64 * track_handle.get_percentage()).floor();
                println!("played: {percentage}%",);
                println!("status: {:?}", track_handle.get_status());
            }
            None => {}
        }
    })
}

fn run_player(
    player_handle: Arc<Mutex<PlayerHandle>>,
    rx: Receiver<PlayerCommand>,
) -> anyhow::Result<()> {
    let mut stream: Option<Stream> = None;
    run_player_observer(player_handle.clone());
    while let Ok(command) = rx.recv() {
        match command {
            PlayerCommand::PlayNow(track) => {
                println!("will play track {:?}", track);
                if stream.is_some() {
                    stream = None;
                }
                {
                    let Ok(mut player_handle_guard) = player_handle.lock() else {
                        continue;
                    };
                    player_handle_guard.clear_queue()?;
                    player_handle_guard.enqueue_track(track)?;
                    player_handle_guard.next_track()?;
                };
                handle_play_command(&mut stream, &player_handle)?;
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
                if let Ok(mut player_handle_guard) = player_handle.lock() {
                    if let Some(track_handle) = player_handle_guard.get_mut_track_handle() {
                        track_handle.seek(seconds)?;
                    }
                }
            }
            PlayerCommand::NextTrack => {
                if let Ok(mut player_handle_guard) = player_handle.lock() {
                    player_handle_guard.next_track()?;
                    handle_play_command(&mut stream, &player_handle)?;
                }
            }
        };
    }
    Ok(())
}

fn handle_play_command(
    stream: &mut Option<Stream>,
    player_handle: &Arc<Mutex<PlayerHandle>>,
) -> anyhow::Result<()> {
    if stream.is_some() {
        *stream = None;
    }
    let track_stream = stream_track(player_handle)?;
    track_stream.play()?;
    *stream = Some(track_stream);
    Ok(())
}

pub struct PlayerHandle {
    device: Device,
    player_tx: Sender<PlayerCommand>,
    current_track: Option<TrackHandle>,
    track_queue: Vec<Track>,
    volume: f64,
}

impl PlayerHandle {
    pub fn new(device: Device, player_tx: Sender<PlayerCommand>) -> Self {
        PlayerHandle {
            device,
            player_tx,
            current_track: None,
            track_queue: Vec::new(),
            volume: 1.0,
        }
    }

    pub fn get_device(&self) -> &Device {
        &self.device
    }

    pub fn get_track_handle(&self) -> Option<&TrackHandle> {
        self.current_track.as_ref()
    }

    pub fn get_mut_track_handle(&mut self) -> Option<&mut TrackHandle> {
        self.current_track.as_mut()
    }

    pub fn clear_queue(&mut self) -> anyhow::Result<()> {
        Ok(self.track_queue.clear())
    }

    pub fn enqueue_track(&mut self, track: Track) -> anyhow::Result<()> {
        Ok(self.track_queue.push(track))
    }

    pub fn trigger_next_track(&self) -> anyhow::Result<()> {
        Ok(self.player_tx.send(PlayerCommand::NextTrack)?)
    }

    pub fn change_volume(&mut self, volume: f64) -> anyhow::Result<()> {
        self.volume = volume;
        if let Some(track_handle) = self.get_mut_track_handle() {
            track_handle.update_volume(volume);
        }
        Ok(())
    }

    pub fn next_track(&mut self) -> anyhow::Result<bool> {
        let next_track = self.track_queue.pop();
        match next_track {
            Some(track) => {
                self.current_track = Some(track.get_track_handle(self.volume)?);
                Ok(true)
            }
            None => {
                self.current_track = None;
                Ok(false)
            }
        }
    }
}

pub struct PlayerController {
    player_handle: Arc<Mutex<PlayerHandle>>,
    player_command_tx: Sender<PlayerCommand>,
}

impl PlayerController {
    pub fn play_now(&self, path: String) -> anyhow::Result<()> {
        let track = Track::new(path.to_owned());
        self.player_command_tx
            .send(PlayerCommand::PlayNow(track))
            .expect("Could not play track");
        Ok(())
    }

    pub fn pause(&self) -> anyhow::Result<()> {
        self.player_command_tx
            .send(PlayerCommand::Pause)
            .expect("Could not pause track");
        Ok(())
    }

    pub fn resume(&self) -> anyhow::Result<()> {
        self.player_command_tx
            .send(PlayerCommand::Resume)
            .expect("Could not pause track");
        Ok(())
    }

    pub fn seek(&self, seconds: usize) -> anyhow::Result<()> {
        self.player_command_tx
            .send(PlayerCommand::Seek(seconds))
            .expect(&format!("Could not skip to {seconds} seconds"));
        Ok(())
    }

    pub fn change_volume(&self, volume: f64) -> anyhow::Result<()> {
        let Ok(mut player_handle) = self.player_handle.lock() else {
            anyhow::bail!("Could not change volume")
        };
        player_handle.change_volume(volume)?;
        Ok(())
    }

    pub fn get_volume(&self) -> anyhow::Result<f64> {
        let Ok(player_handle) = self.player_handle.lock() else {
            anyhow::bail!("Could not get volume")
        };
        Ok(player_handle.volume)
    }
}
