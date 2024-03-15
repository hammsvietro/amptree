use crate::track::{Track, TrackData, TrackDataHandle};
use crate::TestState;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, FromSample, Sample, SampleRate, StreamConfig};
use tauri::State;

const VOLUME: f64 = 0.06;

#[tauri::command]
pub fn play_audio(path: String, _state: State<TestState>) -> Result<(), String> {
    if let Err(error) = play(path) {
        return Err(error.to_string());
    }
    Ok(())
}

fn play(path: String) -> anyhow::Result<()> {
    let device = get_device()?;
    Ok(stream_from_path(&path, &device)?)
}

fn stream_from_path(path: &String, device: &Device) -> anyhow::Result<()> {
    let track = Track::new(path.clone());
    let track_handle = track.get_data()?;
    let Ok(handle) = track_handle.lock() else {
        anyhow::bail!("Couldn't acquire handle lock")
    };

    let config = get_config(device, &handle)?;
    drop(handle);
    let handle_clone = track_handle.clone();
    let stream = device.build_output_stream(
        &config,
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            write_data(data, handle_clone.clone())
        },
        |err| eprintln!("an error occurred on the output audio stream: {}", err),
        None,
    )?;
    stream.play()?;
    std::thread::sleep(std::time::Duration::from_secs(200));
    Ok(())
}

fn get_device() -> anyhow::Result<Device> {
    match cpal::default_host().default_output_device() {
        Some(device) => Ok(device),
        None => anyhow::bail!("No device available."),
    }
}

fn get_config(device: &Device, track_data: &TrackData) -> anyhow::Result<StreamConfig> {
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

fn write_data<T>(output: &mut [T], track_handle: TrackDataHandle)
where
    T: Sample + FromSample<f64>,
{
    if let Ok(mut track_handle) = track_handle.lock() {
        for frame in output.chunks_mut(track_handle.channel_count) {
            let samples = track_handle.get_sample_vec();
            for (channel, sample) in frame.iter_mut().enumerate() {
                let value: T = T::from_sample(samples[channel] * VOLUME);
                *sample = value;
            }
            track_handle.time += 1;
        }
    }
}
