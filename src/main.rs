pub mod config;
pub mod database;
pub mod event;
pub mod frontend;
pub mod fuzzy;

use database::*;
use fuzzy::*;
use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    // opening the connection to the database
    let conn = Connection::open("./test.db")?;
    // clear it for testing purposes
    // down(&conn)?;
    // make sure it works
    up(&conn)?;
    // testing
    // use frontend::*;
    // use event::Event;
    sk_all_events(&conn);
    // this is where the magic happens!
    config::parse(&conn)?;
    Ok(())
}
