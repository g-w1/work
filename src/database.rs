use crate::config::DATABASE_URL;
use crate::event::Event;
use rusqlite::{params, Connection, Result};

pub fn up(conn: &Connection) -> Result<()> {
    conn.execute(
        "
CREATE TABLE IF NOT EXISTS events (
    id      INTEGER PRIMARY KEY AUTOINCREMENT,
    summary TEXT NOT NULL
    done    BOOL NOT NULL
    )",
        params![],
    )?;
    Ok(())
}

pub fn drop(conn: &Connection) -> Result<()> {
    let conn = Connection::open(DATABASE_URL)?;
    conn.execute("DROP TABLE events;", params![])?;
    Ok(())
}

pub fn get_event_by_id(conn: &Connection, id: i32) -> Result<()> {
    let conn = Connection::open(DATABASE_URL)?;
    let mut stmt = conn.prepare("SELECT id, summary, done FROM person where id = ?1")?;
    let event_iter = stmt.query_map(params![id], |row| {
        Ok(Event {
            id: row.get(0)?,
            summary: row.get(1)?,
            done: row.get(2)?,
        })
    })?;
    Ok(())
}
