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
            parse_ids(ids.to_string());
        } else {
            list_all_events(&conn)?;
        }
    }
    Ok(())
}

fn parse_ids(id_string: String) -> Result<(u32, Option<u32>)> {
    if id_string.contains('-') {
        let input_nums: Vec<&str> = id_string.split('-').collect();
        println!("input_nums: {:?}", input_nums);
    } else {
    }
    (1, Some(1))
}
