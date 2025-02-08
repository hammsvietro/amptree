use std::hash::{Hash, Hasher};

use rusqlite::Connection;

use crate::audio::AudioFile;

mod repository;
pub mod scanner;

pub struct Library {
    repository: repository::LibraryRepository,
}

impl Library {
    pub fn new(connection: Connection) -> Library {
        Library {
            repository: repository::LibraryRepository::new(connection),
        }
    }

    pub fn scan(&self, path: &str) {
        scanner::scan_directory(path);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Artist {
    name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Album {
    name: String,
    cover_path: String,
}

#[derive(Debug)]
pub struct Track {
    path: String,
    name: String,
    album_order: i32,
}

impl Into<AudioFile> for &Track {
    fn into(self) -> AudioFile {
        AudioFile::new(self.path.clone())
    }
}
