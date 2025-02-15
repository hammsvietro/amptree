use cpal::traits::HostTrait;
use cpal::Device;

mod decoder;
mod player;
mod stream;

pub use decoder::{AudioFile, AudioMetadata, AudioSource};
pub use player::{boot_player, PlayerController};

pub fn get_device() -> anyhow::Result<Device> {
    match cpal::default_host().default_output_device() {
        Some(device) => Ok(device),
        None => anyhow::bail!("No device available."),
    }
}
