use crate::audio::track::Track;

pub async fn scan_directory(path: String) -> Vec<Track> {
    let mut tracks = Vec::new();
    let mut dir = tokio::fs::read_dir(path).await;
    tracks
}
