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
                .arg("<id(s)> 'the id(s) of the events to list'"),
        )
        .get_matches();
    if let Some(c) = matches.value_of("config") {
        println!("Config file is {}", c);
    }
    if let Some(ref lsmatches) = matches.subcommand_matches("ls") {
        if let Some(ids) = lsmatches.value_of("id(s)") {
            println!("ids: {}", ids);
        } else {
            list_all_events(&conn)?;
        }
    }
    Ok(())
}
