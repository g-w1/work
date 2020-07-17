use crate::frontend::*;
use clap::App;
use rusqlite::Connection;
use rusqlite::Error;

pub fn parse(conn: &Connection) -> Result<(), Error> {
    let matches = App::new("Work")
        .version("0.1.0")
        .author("Jacob G-W <jacoblevgw@gmail.com>")
        .about("A cli todo app")
        .arg("-c, --config=[FILE] 'use a custom config file'")
        .subcommand(
            App::new("ls")
                .about("list events in database")
                .arg("<id> 'the id of the events to list (only lists on event)'"),
        )
        .get_matches();
    if let Some(c) = matches.value_of("config") {
        println!("Config file is {}", c);
    }
    if let Some(ref lsmatches) = matches.subcommand_matches("ls") {
        if let Some(ids) = lsmatches.value_of("id") {
            match parse_ids(ids.to_string()) {
                Ok(x) => {
                    list_event_by_id(&conn, x)?;
                }
                Err(_) => {
                    eprintln!("Invalid input. Try doing something like `work ls <id of an event>`");
                }
            }
        } else {
            list_all_events(&conn)?;
        }
    }
    Ok(())
}

fn parse_ids(id_string: String) -> Result<u32, std::num::ParseIntError> {
    id_string.parse::<u32>()
}
