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
}
