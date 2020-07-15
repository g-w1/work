extern crate work;
extern crate diesel;

use self::diesel::prelude::*;
use self::work::*;
use std::env::args;

fn main() {
    use work::schema::events::dsl::*;

    let target = args().nth(1).expect("Expected a target to match against");
    let pattern = format!("%{}%", target);

    let connection = establish_connection();
    let num_deleted = diesel::delete(events.filter(summary.like(pattern)))
        .execute(&connection)
        .expect("Error deleting event");

    println!("Deleted {} events", num_deleted);
}
