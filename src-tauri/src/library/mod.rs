use std::hash::{Hash, Hasher};

use rusqlite::Connection;

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
        scanner::scan_directory();
    }
}

#[derive(Debug)]
pub struct Track {
    path: String,
    name: String,
    album_order: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Album {
    name: String,
    cover_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Artist {
    name: String,
}
