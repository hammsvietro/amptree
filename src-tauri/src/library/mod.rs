use std::{collections::HashMap, hash::Hash, path::Path};

use rusqlite::Connection;

use crate::audio::AudioFile;

mod repository;
pub mod scanner;

pub(super) type ScanResult = anyhow::Result<HashMap<Artist, HashMap<Album, Vec<Track>>>>;

pub struct Library {
    repository: repository::LibraryRepository,
}

impl Library {
    pub fn new(connection: Connection) -> Library {
        Library {
            repository: repository::LibraryRepository::new(connection),
        }
    }

    pub async fn scan(&self, path: &str) -> anyhow::Result<()> {
        let scanned = scanner::scan_directory(path).await?;
        println!("{scanned:?}");
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Artist {
    name: String,
}

impl Artist {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Album {
    name: String,
    cover_path: Option<String>,
}

impl Album {
    pub fn new(name: String, cover_path: Option<String>) -> Self {
        Self { name, cover_path }
    }
}

#[derive(Debug)]
pub struct Track {
    path: String,
    name: Option<String>,
    album_order: Option<usize>,
}

impl Track {
    pub fn new(path: String, name: Option<String>, album_order: Option<usize>) -> Self {
        Self {
            path,
            name,
            album_order,
        }
    }
}

impl Into<AudioFile> for &Track {
    fn into(self) -> AudioFile {
        AudioFile::new(self.path.clone())
    }
}
