use crate::database::*;
use crate::event::Event;

use rusqlite::{Connection, Result};
use std::env::var;
use std::path::PathBuf;

fn get_conn() -> Result<Connection> {
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
    let conn = Connection::open(path).unwrap();
    down(&conn).unwrap();
    up(&conn).unwrap();
    Ok(conn)
}
#[test]
fn clean_slate() {
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
    let conn = Connection::open(path).unwrap();
    down(&conn).unwrap();
    up(&conn).unwrap();
}
#[test]
fn insert_and_get_back_event() {
    // clearing the database
    let conn = get_conn().unwrap();
    down(&conn).unwrap();
    up(&conn).unwrap();
    // making the events
    let name = String::from("spanish test\ntest");
    let othername = String::from("math test\n🇨🇼🇰🇵\"");
    let event = Event::new(name.clone());
    let other_event = Event::new(othername.clone());
    // putting the events into the database
    event.into_database(&conn).unwrap();
    other_event.into_database(&conn).unwrap();
    // getting them back
    let events_good = get_all_events(&conn).expect("didnt get a vec of events, ooooooof");
    // the actual tests
    let mut events = Vec::new();
    for i in events_good {
        events.push(i.unwrap());
    }
    assert_eq!(events[0], Event::new(name));
    assert_eq!(events[1], Event::new(othername));
}
