pub mod config;
pub mod database;
pub mod event;
pub mod frontend;
pub mod fuzzy;

use database::*;
use rusqlite::{Connection, Result};
use std::env::var;
use std::path::{PathBuf};

fn main() -> Result<()> {
    // opening the connection to the database
    let home = var("HOME").expect("you must be on a unix like system");
    let path: PathBuf = [home.as_str(), ".local", "share", "worktodo", "work.db"]
        .iter()
        .collect();
    let path_dir: PathBuf = [home.as_str(), ".local", "share", "worktodo"]
        .iter()
        .collect();
    if !path.exists() {
        std::fs::create_dir_all(path_dir).unwrap();
    }
    let conn = Connection::open(path)?;
    // clear it for testing purposes
    // down(&conn)?;
    // make sure it works
    up(&conn)?;
    // testing
    // use frontend::*;
    // use event::Event;
    // this is where the magic happens!
    config::parse(&conn)?;
    Ok(())
}
