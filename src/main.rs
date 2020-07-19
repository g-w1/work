pub mod config;
pub mod database;
pub mod event;
pub mod frontend;

use database::*;
use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    // opening the connection to the database
    let conn = Connection::open("./test.db")?;
    // clear it for testing purposes
    // down(&conn)?;
    // make sure it works
    up(&conn)?;
    // testing
    // use event::Event;
    // use frontend::*;
    // let spanish_event = Event::new(String::from("spanish quiz"));
    // spanish_event.into_database(&conn)?;
    // update_event_from_id(&conn, 1)?;
    // this is where the magic happens!
    config::parse(&conn)?;
    Ok(())
}
