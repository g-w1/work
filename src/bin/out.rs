extern crate diesel;
extern crate work;

use self::diesel::prelude::*;
use self::models::Event;
use self::models::*;
use self::work::*;
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
    let event: models::Event = events
        .find(id_to_delete)
        .first(&connection)
        .unwrap_or_else(|_| panic!("Unable to find event {}", id_to_delete));
    println!(
        "delete event with id: {} and summary: {}?",
        id_to_delete, event.summary
    );
    if are_u_sure() {
        use work::schema::events::dsl::*;

        let connection = establish_connection();
        let num_deleted = diesel::delete(events.filter(id.eq(id_to_delete)))
            .execute(&connection)
            .expect("Error deleting event");
        println!("Deleted event with id {}", id_to_delete);
    } else {
        println!("Canceled. Did not delete anything.");
    }
}

fn mark_finished_or_not(id_to_mark_finished: usize, finished_or_not: bool) {
    use work::schema::events::dsl::{done, events};

    let connection = establish_connection();
    let _ = diesel::update(events.find(id_to_mark_finished))
        .set(done.eq(finished_or_not))
        .execute(&connection)
        .unwrap();

    let event: models::Event = events
        .find(id)
        .first(&connection)
        .unwrap_or_else(|_| panic!("Unable to find event {}", id));
    println!(
        "Marked event \"{}\" with id {} as finished.",
        event.summary, event.id
    );
}

fn print_events() {
    use work::schema::events::dsl::*;
    let connection = establish_connection();
    let results = events
        .filter(done.eq(false))
        .load::<Event>(&connection)
        .expect("Error loading events");

    println!("Displaying {} events", results.len());
    for event in results {
        println!("{}    {}", finished_or_not(event.done), event.summary);
    }
}

fn format_finished(finished_or_not: bool) -> &str {
    if finished_or_not {
        "✔️"
    } else {
        "❌"
    }
}

fn create_event_no_args() {
    let connection = establish_connection();
    println!("What would you like your new event to be?\nTYPE EVENT HERE:");
    let mut summary = String::new();
    stdin().read_line(&mut summary).unwrap();
    let event = create_event(&connection, summary);
    println!("Saved event {}", summary);
}

fn create_event_from_str(event_str: &str) {
    let connection = establish_connection();
    let event = create_event(&connection, event_str);
    println!("Saved event {}", event_str);
}
fn parse_cli() {
    let matches = App::new("Work")
        .version("0.1.0")
        .author("Jacob G-W <jacoblevgw@gmail.com>")
        .about("Cli TODO App")
        .arg(
            Arg::new("new")
                .about("make a new event")
        )
        .arg(
            Arg::new("v")
                .short('v')
                .multiple(true)
                .about("Sets the level of verbosity"),
        )
        .subcommand(
            App::new("ls")
                .about("lists things to-do")
                .arg(
                    Arg::new("id of item")
                        .about("type an id of an item to print it verbosely")
                        .required(false),
                ),
        )
        .get_matches();
}
