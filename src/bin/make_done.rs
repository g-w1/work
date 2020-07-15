extern crate work;
extern crate diesel;

use self::diesel::prelude::*;
use self::work::*;
use self::models::Event;
use std::env::args;

fn main() {
    use work::schema::events::dsl::{events, done};

    let id = args().nth(1).expect("make_done requires an event id")
        .parse::<i32>().expect("Invalid ID");
    let connection = establish_connection();

    let _ = diesel::update(events.find(id))
        .set(done.eq(true))
        .execute(&connection)
        .unwrap();

    let event: models::Event = events
        .find(id)
        .first(&connection)
        .unwrap_or_else(|_| panic!("Unable to find event {}", id));
    println!("marked event {} as finished", event.summary);
}
