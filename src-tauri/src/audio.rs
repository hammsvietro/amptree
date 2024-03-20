use std::sync::{Arc, Mutex};

use crate::player::PlayerCommand;
use crate::track::{Track, TrackHandle};
use crate::PlayerHandle;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, FromSample, Sample, SampleRate, StreamConfig, Stream};
use tauri::State;

const VOLUME: f64 = 0.06;

#[tauri::command]
pub async fn play_audio(path: String, state: State<'_,PlayerHandle>) -> Result<(), String> {
    if let Err(error) = queue(path, state).await {
        return Err(error.to_string());
    }
    Ok(())
}

#[tauri::command]
pub async fn pause(state: State<'_, PlayerHandle>) -> Result<(), String> {
    state.tx.send(PlayerCommand::Pause).expect("Could not pause track");
    Ok(())
}

#[tauri::command]
pub async fn resume(state: State<'_, PlayerHandle>) -> Result<(), String> {
    state.tx.send(PlayerCommand::Resume).expect("Could not pause track");
    Ok(())
}

async fn queue(path: String, state: State<'_, PlayerHandle>) -> anyhow::Result<()> {
    let track = Track::new(path.to_owned());
    state.tx.send(PlayerCommand::Play(track))?;
    Ok(())
}


pub fn stream_track(track: &Track, device: &Device) -> anyhow::Result<Stream> {
    let track_handle = track.get_track_handle()?;
    let Ok(handle) = track_handle.lock() else {
        anyhow::bail!("Couldn't acquire handle lock")
    };

    let config = get_config(device, &handle)?;
    drop(handle);

    let handle_clone = track_handle.clone();
    Ok(device.build_output_stream(
        &config,
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            write_data(data, handle_clone.clone())
        },
        |err| eprintln!("an error occurred on the output audio stream: {}", err),
        None,
    )?)
}

pub fn get_device() -> anyhow::Result<Device> {
    match cpal::default_host().default_output_device() {
        Some(device) => Ok(device),
        None => anyhow::bail!("No device available."),
    }
}

pub fn get_config(device: &Device, track_data: &TrackHandle) -> anyhow::Result<StreamConfig> {
    let channel_count = track_data.channel_count as u16;
    let supported_configs = device.supported_output_configs()?;
    for supported_config in supported_configs {
        if channel_count == supported_config.channels() {
            return Ok(supported_config
                .with_sample_rate(SampleRate(track_data.sample_rate))
                .into());
        }
    }
    anyhow::bail!("Couldn't build configuration")
}

fn write_data<T>(output: &mut [T], track_handle: Arc<Mutex<TrackHandle>>)
where
    T: Sample + FromSample<f64>,
{
    if let Ok(mut track_handle) = track_handle.lock() {
        for frame in output.chunks_mut(track_handle.channel_count) {
            let Ok(samples) = track_handle.get_sample_buffer() else {
                panic!("coulnd't fetch sample buffer");
            };
            if samples.len() == track_handle.channel_count {
                for (channel, sample) in frame.iter_mut().enumerate() {
                    let value: T = T::from_sample(samples[channel] * VOLUME);
                    *sample = value;
                }
            }
            track_handle.increment_time();
        }
    }
}
