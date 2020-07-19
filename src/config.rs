use crate::frontend::*;
use clap::{App, Arg};
use rusqlite::Connection;
use rusqlite::Error;

// the parsing function for the app
pub fn parse(conn: &Connection) -> Result<(), Error> {
    // building the matcher
    let matches = App::new("Work")
        .version("0.1.0")
        .author("Jacob G-W <jacoblevgw@gmail.com>")
        .about("A cli todo app")
        .arg("-c, --config=[FILE] 'use a custom config file'")
        // the ls subcommand. list stuff in the database
        .subcommand(
            App::new("ls").about("list events in database").arg(
                Arg::new("id")
                    .about("the id of the event to display")
                    .required(false)
                    .index(1),
            ),
        )
        // the rm subcommand. rm stuff from database
        .subcommand(
            App::new("rm")
                .about("rm an event from the database")
                .arg("<id> 'the id to rm'"),
        )
        // the add subcommand. add stuff to database
        .subcommand(
            App::new("add")
                .about("add an event to the database")
                .arg("<summary> 'summary of the event to add to the database'"),
        )
        .get_matches();
    // config parsing
    if let Some(c) = matches.value_of("config") {
        println!("Config file is {}", c);
    }
    // parsing ls cmd
    if let Some(ref lsmatches) = matches.subcommand_matches("ls") {
        if let Some(ids) = lsmatches.value_of("id") {
            match parse_ids(ids) {
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
    // parsing rm cmd
    if let Some(ref rm_matches) = matches.subcommand_matches("rm") {
        if let Some(ids) = rm_matches.value_of("id") {
            match parse_ids(ids) {
                Ok(x) => {
                    delete_event_from_id(&conn, x as i32);
                }
                Err(_) => {
                    if ids == "all" {
                        delete_down(&conn)?;
                    } else {
                        eprintln!(
                        "Invalid input. Try doing something like `work rm <id of an event or all to rm_everything>`"
                    );
                    }
                }
            }
        }
    }
    // parsing add cmd
    if let Some(ref add_matches) = matches.subcommand_matches("add") {
        if let Some(summary) = add_matches.value_of("summary") {
            add_event(&conn, summary.to_string())?;
            println!("added event with summary ``{}\" to database", summary);
        }
    }
    Ok(())
}

fn parse_ids(id_string: &str) -> Result<u32, std::num::ParseIntError> {
    id_string.parse::<u32>()
}
