extern crate work;
extern crate diesel;

use self::work::*;
use std::io::{stdin, Read};

fn main() {
    let connection = establish_connection();
    println!("What would you like your summary to be?");
    let mut summary = String::new();
    stdin().read_line(&mut summary).unwrap();
    let summary = &summary[..(summary.len() - 1)]; // Drop the newline character
    println!("\nOk! Let's write {} (Press {} when finished)\n", summary, EOF);
    let mut time = String::new();
    stdin().read_to_string(&mut time).unwrap();
    let event = create_event(&connection, summary, &time);
    println!("\nSaved event {}", summary);
}

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";

#[cfg(windows)]
const EOF: &'static str = "CTRL+Z";
