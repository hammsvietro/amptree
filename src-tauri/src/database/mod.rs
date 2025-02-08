use anyhow::Result;
use rusqlite::Connection;

mod migrations;

pub fn get_connection() -> anyhow::Result<Connection> {
    let conn = init_db()?;
    Ok(conn)
}

fn init_db() -> Result<Connection> {
    let database_dir = dirs::data_dir().unwrap().join("amptree");
    std::fs::create_dir_all(&database_dir)?;
    let database_file = database_dir.join("db.sqlite");
    let mut conn = Connection::open(database_file)?;
    configure_db(&mut conn)?;
    migrations::migrate(&mut conn)?;
    Ok(conn)
}

fn configure_db(conn: &mut Connection) -> Result<()> {
    conn.pragma_update(None, "journal_mode", "WAL")?;
    conn.pragma_update(None, "foreign_keys", "ON")?;
    Ok(())
}

#[cfg(test)]
fn init_test_db() -> Result<Connection> {
    let mut conn = Connection::open_in_memory()?;
    configure_db(&mut conn)?;
    migrations::migrate(&mut conn)?;
    Ok(conn)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_db() {
        let conn = init_test_db().unwrap();
        let mut stmt = conn
            .prepare("SELECT name FROM sqlite_master WHERE type='table'")
            .unwrap();
        let tables = stmt.query_map([], |row| row.get(0)).unwrap();
        let tables: Vec<String> = tables.map(|r| r.unwrap()).collect();
        assert!(tables.contains(&"Artists".to_string()));
        assert!(tables.contains(&"Albums".to_string()));
        assert!(tables.contains(&"Tracks".to_string()));
    }
}
