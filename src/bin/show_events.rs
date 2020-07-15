extern crate work;
extern crate diesel;

use self::work::*;
use self::models::*;
use self::diesel::prelude::*;

fn main() {
    use work::schema::events::dsl::*;
    let connection = establish_connection();
    let results = events.filter(done.eq(false))
        .limit(5)
        .load::<Event>(&connection)
        .expect("Error loading events");

    println!("Displaying {} events", results.len());
    for event in results {
        println!("{}", event.summary);
    }
}
