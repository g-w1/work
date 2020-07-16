pub mod config;
pub mod database;
use clap::{Arg, App, Subcommand};
use std::env::args;
use std::io::{stdin, Read};

fn are_u_sure() -> bool {
    println!("Are you sure? (Y/n)");
    let mut sure: String = String::new();
    stdin().read_line(&mut sure).unwrap();
    if sure.to_lowercase() == String::from("y") || sure.to_lowercase() == String::from("yes") {
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

fn mark_finished_or_not(id_to_mark_finished: usize, finished_or_not: bool) {
   println!(
        "Marked event \"{{}}\" with id {{}} as finished.",
        // event.summary, event.id
    );
}

fn print_events() {
    // println!("Displaying {} events", results.len());
    // for event in results {
    //     println!("{}    {}", finished_or_not(event.done), event.summary);
    // }
}

fn format_finished(finished_or_not: bool, in_20th_century: bool) -> String {
    if finished_or_not && in_20th_century{
        String::from("✅")
    } else if in_20th_century{
        String::from("❌")
    }
    else if finished_or_not {
        String::from("[X]")
    }
    else {
        String::from("[ ]")
    }
}

// fn create_event_no_args() {
//     let connection = establish_connection();
//     println!("What would you like your new event to be?\nTYPE EVENT HERE:");
//     let mut summary = String::new();
//     stdin().read_line(&mut summary).unwrap();
//     let event = create_event(&connection, summary);
//     println!("Saved event {}", summary);
// }

// fn create_event_from_str(event_str: &str) {
//     let connection = establish_connection();
//     let event = create_event(&connection, event_str);
//     println!("Saved event {}", event_str);
// }

fn parse_cli() {
    let matches = App::new("Work")
        .version("0.1.0")
        .author("Jacob G-W <jacoblevgw@gmail.com>")
        .about("Cli TODO App")
        .arg(Arg::new("new").about("make a new event"))
        .arg(
            Arg::new("v")
                .short('v')
                .multiple(true)
                .about("Sets the level of verbosity"),
        )
        .subcommand(
            App::new("ls").about("lists things to-do").arg(
                Arg::new("id of item")
                    .about("type an id of an item to print it verbosely")
                    .required(false),
            ),
        )
        .get_matches();
}
fn main() {
    println!("{}", format_finished(true, true));
}
