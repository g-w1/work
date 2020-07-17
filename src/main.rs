pub mod config;
pub mod database;
pub mod event;
pub mod frontend;

use clap::{App, Arg, Subcommand};
use database::*;
use event::Event;
use rusqlite::{params, Connection, Result};
use std::env::args;
use std::io::{stdin, Read};

fn are_u_sure() -> bool {
    println!("Are you sure? (Y/n) ");
    let mut sure: String = String::new();
    stdin().read_line(&mut sure).unwrap();
    let sure = sure.to_lowercase().trim().to_owned();
    if sure == String::from("y") || sure == String::from("yes") {
        true
    } else {
        false
    }
}
fn delete(id_to_delete: usize) {
    println!("delete event with id: {{}} and summary: {{}}?",);
    if are_u_sure() {
        println!("Deleted event with id {}", id_to_delete);
    } else {
        println!("Canceled. Did not delete anything.");
    }
}

fn format_finished(finished_or_not: bool, in_20th_century: bool) -> String {
    if finished_or_not && in_20th_century {
        String::from("✅")
    } else if in_20th_century {
        String::from("❌")
    } else if finished_or_not {
        String::from("[X]")
    } else {
        String::from("[ ]")
    }
}

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
    delete_event_by_id(&conn, 2)?;
    println!("{:?}", get_all_events(&conn));
    Ok(())
}
