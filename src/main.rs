pub mod config;
pub mod database;
pub mod event;

use clap::{App, Arg, Subcommand};
use database::*;
use event::Event;
use rusqlite::{params, Connection, Result, NO_PARAMS};
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
    println!(
        "delete event with id: {{}} and summary: {{}}?",
        // id_to_delete, event.summary
    );
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
    // down(&conn)?;
    up(&conn)?;
    let spanish = Event::new(String::from("spanish quiz"));
    spanish.into_database(&conn)?;
    println!("{:?}", get_event_by_id(&conn, 0)?);
    Ok(())
}
