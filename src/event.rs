use rusqlite::{params, Connection, Result};

#[derive(Debug)]
pub struct Event {
    pub id: Option<i32>,
    pub summary: String,
    pub done: bool,
}

impl Event {
    pub fn new(summary: String) -> Self {
        Event {
            id: None,
            summary: summary,
            done: false,
        }
    }
    pub fn new_from_db(id: Option<i32>, summary: String, done: bool) -> Self {
        Event {
            id: id,
            summary: summary,
            done: done,
        }
    }
    pub fn into_database(&self, conn: &Connection) -> Result<()> {
        conn.execute(
            "INSERT INTO events (summary, done) values (?1, ?2)",
            params![self.summary, self.done],
        )?;
        Ok(())
    }
}
