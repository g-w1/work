extern crate diesel;
extern crate work;

use self::diesel::prelude::*;
use self::work::*;
use std::env::args;

fn main() {
    use work::schema::events::dsl::*;

    let target = args()
        .nth(1)
        .expect("expected an id of the event to delete");
    let pattern = format!("%{}%", target);

    let connection = establish_connection();
    let num_deleted = diesel::delete(events.filter(id.eq(target)))
        .execute(&connection)
        .expect("Error deleting event");

    println!("Deleted {} events", num_deleted);
}
