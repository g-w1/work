use crate::database::*;
use crate::event::Event;
use rusqlite::{Connection, Result};
use std::io::stdin;
use termion::terminal_size;

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
        Err(_) => {
            eprintln!("Error: event not found\nDid not delete.\nPlease try again.");
            return Ok(());
        }
    };
    println!(
        "Delete event with id: '{}' and summary: `{}'?\nThis is not undoable.",
        event.id.unwrap(),
        event.summary
    );
    if are_u_sure() {
        delete_event_by_id(&conn, event.id.unwrap())?;
        println!("Deleted event with id {}", event.id.unwrap());
    } else {
        println!("Canceled. Did not delete anything.");
    }
    Ok(())
}

pub fn list_all_events(conn: &Connection) -> Result<()> {
    let events_special_stuff = get_all_events(&conn);
    let events = match events_special_stuff {
        Ok(x) => x,
        Err(_) => {
            eprintln!("An Error Occured.\nTry adding some events to list first.\nWhoops");
            return Ok(());
        }
    };
    line('_');
    println!("DONE| ID| SUMMARY");
    line('-');
    for event_res in events {
        let event = match event_res {
            Ok(x) => x,
            Err(_) => {
                eprintln!("Error Occured. Whoops.");
                return Ok(());
            }
        };
        println!(
            "{} | {} | {}",
            format_finished(event.done, false),
            event.id.unwrap(),
            event.summary
        );
    }
    line('-');
    Ok(())
}
pub fn list_range_events(conn: &Connection, start: u32, end: u32) -> Result<()> {
    let events_special_stuff = get_range_events(&conn, start, end);
    let events = match events_special_stuff {
        Ok(x) => x,
        Err(_) => {
            eprintln!("An Error Occured.\nTry selecting a range with events in it or adding some events to list first.\nWhoops");
            return Ok(());
        }
    };
    line('_');
    println!("DONE| ID| SUMMARY");
    line('-');
    for event_res in events {
        let event = match event_res {
            Ok(x) => x,
            Err(_) => {
                eprintln!("Error Occured. Whoops.");
                return Ok(());
            }
        };
        println!(
            "{} | {} | {}",
            format_finished(event.done, false),
            event.id.unwrap(),
            event.summary
        );
    }
    line('-');
    Ok(())
}

pub fn list_event_by_id(conn: &Connection, id: u32) -> Result<()> {
    let event_result = get_event_by_id(&conn, id);
    let event = match event_result {
        Ok(x) => x,
        Err(_) => {
            eprintln!("Error. Try another id that is valid");
            return Ok(());
        }
    };
    line('_');
    println!("DONE| ID| SUMMARY");
    line('-');
    println!(
        "{} | {} | {}",
        format_finished(event.done, false),
        event.id.unwrap(),
        event.summary
    );
    line('-');
    Ok(())
}

fn line(char_to_repeat: char) {
    let size = terminal_size().unwrap();
    let mut size_str = String::new();
    for _ in 0..size.0 {
        size_str.push(char_to_repeat);
    }
    println!("{}", size_str);
}
