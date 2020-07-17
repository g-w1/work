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
    conn.query_row(
        "SELECT id, summary, done FROM events WHERE id = ?1",
        params![Some(id_to_query)],
        |row| {
            Ok(Event {
                id: row.get(0)?,
                summary: row.get(1)?,
                done: row.get(2)?,
            })
        },
    )
}
pub fn update_event_by_id(conn: &Connection, event_to_update: Event) -> Result<()> {
    conn.execute(
        "update events set summary = ?1, done = ?2 where id = ?3",
        params![
            event_to_update.summary,
            event_to_update.done,
            event_to_update.id
        ],
    )?;
    Ok(())
}
