use crate::config::Config;
use crate::frontend::*;
use crate::fuzzy::*;
use clap::{App, Arg};
use rusqlite::Connection;
use rusqlite::Error;

// the parsing function for the app
pub fn parse(conn: &Connection, cfg: Option<Config>) -> Result<(), Error> {
    // building the matcher
    let matches = App::new("Work")
        .version("0.1.0")
        .author("Jacob G-W <jacoblevgw@gmail.com>")
        .about("A cli todo app")
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
        .subcommand(
            App::new("edit")
                .about("change an event that is in the database")
                .arg("<id> 'the id of the event that you want to change. to use a fuzzy finder to find your event use `work edit fzf`'"),
        )
        .subcommand(
            App::new("done")
                .about("mark an event as done")
                .arg("<id> 'the id of the event that you want to make done. to use a fuzzy finder to find your event use `work done fzf`'"),
        )
        .get_matches();
    // parsing config
    let cfg_parsed = match cfg {
        Some(x) => x,
        None => Config::default(),
    };
    // parsing ls cmd
    if let Some(ref lsmatches) = matches.subcommand_matches("ls") {
        if let Some(ids) = lsmatches.value_of("id") {
            match parse_ids(ids) {
                Ok(x) => {
                    list_event_by_id(&conn, x, &cfg_parsed)?;
                }
                Err(_) => {
                    eprintln!("Invalid input. Try doing something like `work ls <id of an event>`");
                }
            }
        } else {
            list_all_events(&conn, &cfg_parsed)?;
        }
    }
    // parsing rm cmd
    if let Some(ref rm_matches) = matches.subcommand_matches("rm") {
        if let Some(ids) = rm_matches.value_of("id") {
            match ids {
                "fzf" => rm_sk(&conn, &cfg_parsed)?,
                _ => match parse_ids(ids) {
                    Ok(x) => {
                        delete_event_from_id(&conn, x as i32, &cfg_parsed);
                    }
                    Err(_) => {
                        if ids == "all" {
                            delete_down(&conn, &cfg_parsed)?;
                        } else {
                            eprintln!(
                        "Invalid input. Try doing something like `work rm <id of an event>`"
                    );
                        }
                    }
                },
            }
        }
    }
    // parsing add cmd
    if let Some(ref add_matches) = matches.subcommand_matches("add") {
        if let Some(summary) = add_matches.value_of("summary") {
            add_event(&conn, summary.to_string(), &cfg_parsed)?;
        }
    }
    // parsing edit cmd
    if let Some(ref edit_matches) = matches.subcommand_matches("edit") {
        if let Some(idstring) = edit_matches.value_of("id") {
            match idstring {
                "fzf" => {
                    update_sk(&conn, &cfg_parsed)?;
                }
                _ => match parse_ids(idstring) {
                    Ok(x) => {
                        update_event_from_id(&conn, x, &cfg_parsed)?;
                    }
                    Err(_) => {
                        eprintln!(
                        "Error: invalid input. Try doing something like `work edit <id of an event>`"
                    );
                    }
                },
            }
        }
    }
    // parsing done cmd
    if let Some(ref done_matches) = matches.subcommand_matches("done") {
        if let Some(idstring) = done_matches.value_of("id") {
            match idstring {
                "fzf" => {
                    done_sk(&conn, &cfg_parsed)?;
                }
                _ => match parse_ids(idstring) {
                    Ok(x) => {
                        make_done(&conn, x, &cfg_parsed)?;
                    }
                    Err(_) => {
                        eprintln!(
                        "Error: invalid input. Try doing something like `work done <id of an event>`"
                    );
                    }
                },
            }
        }
    }
    Ok(())
}

fn parse_ids(id_string: &str) -> Result<u32, std::num::ParseIntError> {
    id_string.parse::<u32>()
}
