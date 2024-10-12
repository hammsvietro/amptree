use rusqlite::Connection;
use tokio::sync::Mutex as TokioMutex;

pub(super) struct LibraryRepository {
    connection: TokioMutex<Connection>,
}

impl LibraryRepository {
    pub(super) fn new(connection: Connection) -> Self {
        let connection = TokioMutex::new(connection);
        Self { connection }
    }
}
