use crate::track::{Track, TrackDataHandle};
use crate::TestState;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, FromSample, Sample, StreamConfig};
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
    let config = get_config(&device)?;
    stream_from_path(&path, &device, config)?;
    Ok(())
}

fn stream_from_path(path: &String, device: &Device, config: StreamConfig) -> anyhow::Result<()> {
    let track = Track::new(path.clone());
    let track_handle = track.get_data()?;
    let config = if let Ok(mut handle) = track_handle.lock() {
        handle.skip_to(180);
        StreamConfig {
            buffer_size: config.buffer_size,
            channels: handle.channel_count as u16,
            sample_rate: cpal::SampleRate(handle.sample_rate),
        }
    } else {
        config
    };

    let handle_clone = track_handle.clone();
    let stream = device.build_output_stream(
        &config,
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            write_data(data, handle_clone.clone())
        },
        |err| eprintln!("an error occurred on the output audio stream: {}", err),
        None, // None=blocking, Some(Duration)=timeout
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

fn get_config(device: &Device) -> anyhow::Result<StreamConfig> {
    let mut supported_configs_range = device.supported_output_configs()?;
    match supported_configs_range.next() {
        Some(config) => Ok(config.with_sample_rate(cpal::SampleRate(44100)).into()),
        None => anyhow::bail!("no supported config?!"),
    }
}

fn get_channels_and_sample_rate(config: &StreamConfig) -> (f32, usize) {
    (config.sample_rate.0 as f32, config.channels as usize)
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
