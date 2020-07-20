use crate::database::*;
use rusqlite::{Connection, Result};
use std::env::var;
use std::path::{PathBuf};

fn clean_slate() -> Result<()> {
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
    down(&conn)?;
    up(&conn)?;
    config::parse(&conn)?;


}
#[test]
fn dummy() {

}
