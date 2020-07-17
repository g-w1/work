pub mod config;
pub mod database;
pub mod event;
pub mod frontend;

use clap::{App, Arg, Subcommand};
use database::*;
use event::Event;
use frontend::*;
// use crate::frontend::delete_event;
use rusqlite::{Connection, Result};


fn main() -> Result<()> {
    let conn = Connection::open("./test.db")?;
    down(&conn)?;
    up(&conn)?;
    let spanish_event = Event::new(String::from("spanish quiz"));
    spanish_event.into_database(&conn)?;
    update_event_by_id(
        &conn,
        Event {
            id: Some(1),
            summary: String::from("testing"),
            done: true,
        },
    )?;
    let karate = Event::new(String::from("i need to practice karate"));
    karate.into_database(&conn)?;
    println!("{:?}", get_all_events(&conn));
    delete_event(&conn, get_event_by_id(&conn, 1))?;
    println!("{:?}", get_all_events(&conn));
    Ok(())
}
