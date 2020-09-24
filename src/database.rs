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

pub fn get_event_by_id(conn: &Connection, id_to_query: u32) -> Result<Event> {
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
pub fn delete_event_by_id(conn: &Connection, id_of_event_to_delete: i32) -> Result<()> {
    conn.execute(
        "DELETE FROM events WHERE id = ?1;",
        params![Some(id_of_event_to_delete)],
    )?;
    Ok(())
}

pub fn get_all_events(conn: &Connection) -> Result<Vec<Result<Event>>> {
    let mut stmt = conn.prepare("SELECT id, summary, done FROM events")?;
    let rows = stmt.query_map(NO_PARAMS, |row| {
        Ok(Event {
            id: Some(row.get(0)?),
            summary: row.get(1)?,
            done: row.get(2)?,
        })
    })?;
    let mut events = Vec::new();
    for event in rows {
        events.push(event);
    }
    Ok(events)
}
pub fn get_range_events(conn: &Connection, start: u32, end: u32) -> Result<Vec<Result<Event>>> {
    let mut stmt =
        conn.prepare("SELECT id, summary, done FROM events WHERE id >= ?1 AND id<= ?2")?;
    let rows = stmt.query_map(params![start, end], |row| {
        Ok(Event {
            id: Some(row.get(0)?),
            summary: row.get(1)?,
            done: row.get(2)?,
        })
    })?;
    let mut events = Vec::new();
    for event in rows {
        events.push(event);
    }
    Ok(events)
}

pub fn make_done_from_id(conn: &Connection, id: u32) -> Result<()> {
    let item = get_event_by_id(&conn, id)?;
    if !item.done {
        conn.execute(
            "update events set done = ?1 where id = ?2",
            params![true, id],
        )?;
    } else {
        conn.execute(
            "update events set done = ?1 where id = ?2",
            params![false, id],
        )?;
    }
    Ok(())
}

impl Event {
    pub fn into_database(&self, conn: &Connection) -> Result<()> {
        conn.execute(
            "INSERT INTO events (summary, done) values (?1, ?2)",
            params![self.summary, self.done],
        )?;
        Ok(())
    }
}
