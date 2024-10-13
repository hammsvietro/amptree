use std::collections::HashMap;

use super::{Album, Artist, Track};

type ScanResult = anyhow::Result<HashMap<Artist, HashMap<Album, Vec<Track>>>>;
pub async fn scan_directory(path: String) -> ScanResult {
    let mut result = HashMap::new();
    let mut dir = tokio::fs::read_dir(path).await?;
    Ok(result)
}
