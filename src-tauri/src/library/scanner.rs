use std::{collections::HashMap, future::Future, io::Read, path::PathBuf, pin::Pin};

use tokio::io::AsyncReadExt;

use crate::audio::{AudioFile, AudioMetadata, AudioSource};

use super::{Album, Artist, ScanResult, Track};

pub async fn scan_directory(base_path: &str) -> ScanResult {
    let mut scan_result: HashMap<Artist, HashMap<Album, Vec<Track>>> = HashMap::new();
    let file_paths = get_file_paths(base_path).await?;
    let found_file_metadata = get_audio_metadata_for_paths(file_paths).await?;

    for file_metadata in found_file_metadata {
        let artist_name = file_metadata
            .artist
            .unwrap_or(String::from("Unknown Artist"));
        let artist = Artist::new(artist_name);

        let artist_albums: &mut HashMap<Album, Vec<Track>> = scan_result.entry(artist).or_default();

        let album_name = file_metadata.album.unwrap_or(String::from("Unknown Album"));
        let album = Album::new(album_name, None);

        let album_tracks: &mut Vec<Track> = artist_albums.entry(album).or_default();

        album_tracks.push(Track::new(
            file_metadata.file_path.clone(),
            file_metadata.title,
            file_metadata.track_number,
        ));
    }

    Ok(scan_result)
}

async fn get_file_paths(path: impl Into<PathBuf>) -> anyhow::Result<Vec<String>> {
    let mut file_paths: Vec<String> = Vec::new();
    let mut dir = tokio::fs::read_dir(path.into()).await?;
    while let Some(child) = dir.next_entry().await? {
        if child.file_type().await?.is_file() {
            let file_path = child.path().to_string_lossy().to_string();
            file_paths.push(file_path);
        } else {
            let mut folder_file_paths = get_file_paths_boxed(child.path()).await?;
            file_paths.append(&mut folder_file_paths);
        }
    }
    Ok(file_paths)
}

fn get_file_paths_boxed(
    path: PathBuf,
) -> Pin<Box<dyn Future<Output = anyhow::Result<Vec<String>>> + Send>> {
    Box::pin(get_file_paths(path))
}

async fn is_audio_file(path: &str) -> anyhow::Result<bool> {
    let Ok(mut file) = tokio::fs::File::open(path).await else {
        return Ok(false);
    };
    let mut buf = [0u8; 16];
    let magic_number_bytes_result = file.read_exact(&mut buf).await;
    let Ok(magic_number_bytes) = magic_number_bytes_result else {
        return Ok(false);
    };
    Ok(matches!(
        infer::get(&buf[..magic_number_bytes]),
        Some(kind) if kind.mime_type().starts_with("audio/")
    ))
}

async fn get_audio_metadata_for_paths(paths: Vec<String>) -> anyhow::Result<Vec<AudioMetadata>> {
    let mut found_audio_metadata: Vec<AudioMetadata> = Vec::new();
    for file_path in paths {
        if !is_audio_file(&file_path).await? {
            continue;
        }
        let audio_file = AudioFile::new(file_path);
        if let Ok(current_audio_metadata) = audio_file.get_metadata() {
            found_audio_metadata.push(current_audio_metadata)
        }
    }
    Ok(found_audio_metadata)
}
