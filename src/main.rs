pub mod config;
pub mod database;
pub mod event;
pub mod frontend;

use database::*;
// use event::Event;
use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let conn = Connection::open("./test.db")?;
    up(&conn)?;
    // let spanish_event = Event::new(String::from("spanish quiz"));
    // let other_event = Event::new(String::from("test2"));
    // other_event.into_database(&conn)?;
    // spanish_event.into_database(&conn)?;
    // update_event_by_id(
    //     &conn,
    //     Event {
    //         id: Some(1),
    //         summary: String::from("testing"),
    //         done: true,
    //     },
    // )?;
    // let karate = Event::new(String::from("i need to practice karate"));
    // karate.into_database(&conn)?;
    config::parse(&conn)?;
    Ok(())
}
