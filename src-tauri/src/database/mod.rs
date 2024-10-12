use anyhow::Result;
use rusqlite::Connection;

mod migrations;

pub fn get_connection() -> anyhow::Result<Connection> {
    let conn = init_db()?;
    Ok(conn)
}

fn init_db() -> Result<Connection> {
    let database_dir = dirs::data_dir().unwrap().join("amptree/db.sqlite");
    let mut conn = Connection::open(database_dir)?;
    configure_db(&mut conn)?;
    migrations::migrate(&mut conn)?;
    Ok(conn)
}

fn configure_db(conn: &mut Connection) -> Result<()> {
    conn.pragma_update(None, "journal_mode", "WAL")?;
    conn.pragma_update(None, "foreign_keys", "ON")?;
    Ok(())
}
