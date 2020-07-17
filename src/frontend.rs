use std::io::stdin;
use crate::database::*;
use crate::event::Event;
use rusqlite::{Connection, Result};

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

pub fn delete_event(conn: &Connection, eventresult: Result<Event>) -> Result<()> {
    let event = match eventresult {
        Ok(x) => x,
        Err(e) => {
            println!("Error: event not found\nDid not delete.\nPlease try again.");
            return Ok(());
        }
    };
    println!("Delete event with id: {} and summary: {}?\nThis is not undoable.",event.id.unwrap(), event.summary);
    if are_u_sure() {
        delete_event_by_id(&conn, event.id.unwrap())?;
        println!("Deleted event with id {}", event.id.unwrap());
    } else {
        println!("Canceled. Did not delete anything.");
    }
    Ok(())
}
