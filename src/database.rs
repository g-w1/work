use crate::event::Event;
use rusqlite::NO_PARAMS;
use rusqlite::{params, Connection, Result};

pub fn up(conn: &Connection) -> Result<()> {
    conn.execute(
        "create table if not exists events (
     id integer primary key autoincrement,
    summary text not null,
    done bool not null
         )",
        NO_PARAMS,
    )?;
    Ok(())
}

pub fn down(conn: &Connection) -> Result<()> {
    conn.execute("DROP TABLE events;", NO_PARAMS)?;
    Ok(())
}

pub fn get_event_by_id(conn: &Connection, id_to_query: i32) -> Result<Event> {
    let mut stmt = conn.prepare("SELECT id, summary, done FROM events")?;
    let mut event_iter = stmt.query_map(params![], |row| {
        Ok(Event {
            id: row.get(0)?,
            summary: row.get(1)?,
            done: row.get(2)?,
        })
    })?;
    let event_to_return = event_iter.nth(10).unwrap();
    event_to_return
}
