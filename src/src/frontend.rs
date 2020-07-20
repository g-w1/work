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

fn print_vec_events(event_vec: &Vec<Event>) {
    line('_');
    println!("DONE| ID| SUMMARY");
    line('-');
    for event in event_vec {
        println!(
            "{} | {} | {}",
            format_finished(event.done, true),
            event.id.unwrap(),
            event.summary
        );
    }
    line('-');
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

pub fn delete_event_from_id(conn: &Connection, id: i32) {
    println!(
        "Delete event with id: '{}' and summary: `{}'?\nThis is not undoable.",
        id,
        match get_event_by_id(&conn, id as u32) {
            Ok(x) => x,
            Err(_) => {
                eprintln!(
                "Error: event not found.\nDid not delete.\nPlease try again with a valid event id"
            );
                return;
            }
        }
        .summary
    );
    if are_u_sure() {
        match delete_event_by_id(&conn, id) {
            Ok(_) => println!("Deleted event with id {}", id),
            Err(_) => eprintln!(
                "Error: event not found.\nDid not delete.\nPlease try again with a valid event id"
            ),
        }
    } else {
        println!("Canceled. Did not delete anything.");
    }
}

pub fn delete_down(conn: &Connection) -> Result<()> {
    println!("THIS WILL DELETE THE WHOLE DATABASE.");
    if are_u_sure() {
        down(&conn)?;
        println!("deleted everything");
    } else {
        println!("phew. did not delete anything");
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
    let mut events_out: Vec<Event> = Vec::new();
    for event_res in events {
        let _event = match event_res {
            Ok(x) => x,
            Err(_) => {
                eprintln!("Error Occured. Whoops.");
                return Ok(());
            }
        };
        events_out.push(_event);
    }
    print_vec_events(&events_out);
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
    let mut events_out: Vec<Event> = Vec::new();
    for event_res in events {
        let _event = match event_res {
            Ok(x) => x,
            Err(_) => {
                eprintln!("Error Occured. Whoops.");
                return Ok(());
            }
        };
        events_out.push(_event);
    }
    print_vec_events(&events_out);
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
    print_vec_events(&vec![event]);
    Ok(())
}

pub fn add_event(conn: &Connection, summ: String) -> Result<()> {
    Event::new(summ).into_database(&conn)?;
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

pub fn update_event_from_id(conn: &Connection, id: u32) -> Result<()> {
    let event = match get_event_by_id(&conn, id) {
        Ok(x) => x,
        Err(_) => {
            eprintln!("Id not valid please try again");
            return Ok(());
        }
    };
    println!(
        "What should be the new summary of the event with summary ``{}\" be:",
        event.summary
    );
    let mut new_summ: String = String::new();
    stdin().read_line(&mut new_summ).unwrap();
    let new_summ: String = new_summ.trim().to_string();
    let done: bool;
    if !event.done {
        println!("Mark event as done?");
        let mut sure: String = String::new();
        stdin().read_line(&mut sure).unwrap();
        let sure = sure.to_lowercase().trim().to_owned();
        if sure == String::from("y") || sure == String::from("yes") {
            done = true;
        } else {
            done = false;
        }
    } else {
        println!("Mark event as unfinished?");
        let mut sure: String = String::new();
        stdin().read_line(&mut sure).unwrap();
        let sure = sure.to_lowercase().trim().to_owned();
        if sure == String::from("y") || sure == String::from("yes") {
            done = false;
        } else {
            done = true;
        }
    }
    match update_event_by_id(&conn, Event::new_from_db(Some(id as i32), new_summ, done)) {
        Ok(_) => Ok(()),
        _ => {
            eprintln!("Error: something went wrong. oof");
            Ok(())
        }
    }
}
