pub mod cli;
pub mod config;
pub mod database;
pub mod event;
pub mod frontend;
pub mod fuzzy;
#[cfg(test)]
pub mod tests;

use database::*;
use rusqlite::{Connection, Result};
use std::env::var;
use std::path::PathBuf;

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
        println!("initalized empty database at ~/.local/share/worktodo/work.db");
    }
    let conn = Connection::open(path)?;
    up(&conn)?;
    let cfg: Option<config::Config> = match config::load_config() {
        Ok(x) => Some(x),
        Err(_) => {
            println!("error sourcing config");
            None
        }
    };
    cli::parse(&conn, cfg)?;
    Ok(())
}
