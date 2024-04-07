use std::sync::{Arc, Mutex};

use crate::player::PlayerHandle;
use cpal::traits::{DeviceTrait, HostTrait};
use cpal::{Device, FromSample, Sample, SampleRate, Stream, StreamConfig};

const VOLUME: f64 = 0.06;

pub fn stream_track(track_handle: &Arc<Mutex<PlayerHandle>>) -> anyhow::Result<Stream> {
    let Ok(handle) = track_handle.lock() else {
        anyhow::bail!("Couldn't acquire handle lock")
    };

    let (device, config) = get_config(&handle)?;
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

pub fn get_config(player_handle: &PlayerHandle) -> anyhow::Result<(&Device, StreamConfig)> {
    let Some(track_data) = player_handle.get_track_handle() else {
        anyhow::bail!("No track to play.");
    };
    let device = player_handle.get_device();
    let channel_count = track_data.channel_count as u16;
    let supported_configs = device.supported_output_configs()?;
    for supported_config in supported_configs {
        if channel_count == supported_config.channels() {
            return Ok((
                device.clone(),
                supported_config
                    .with_sample_rate(SampleRate(track_data.sample_rate))
                    .into(),
            ));
        }
    }
    anyhow::bail!("Couldn't build configuration")
}

fn write_data<T>(output: &mut [T], player_handle: Arc<Mutex<PlayerHandle>>)
where
    T: Sample + FromSample<f64>,
{
    let Ok(mut player_handle_guard) = player_handle.lock() else {
        return;
    };

    let track_handle = player_handle_guard.get_mut_track_handle();

    match track_handle {
        Some(track_handle) => {
            if track_handle.has_finished() {
                player_handle_guard
                    .trigger_next_track()
                    .expect("Couldn't trigger next track");
                return;
            }
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
        None => {}
    }
}
