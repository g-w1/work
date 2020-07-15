#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

use self::models::{Event, NewEvent};

pub fn create_event<'a>(conn: &SqliteConnection, summary: &'a str) -> usize {
    use schema::events;

    let new_event = NewEvent { summary: summary };

    diesel::insert_into(events::table)
        .values(&new_event)
        .execute(conn)
        .expect("Error saving new event")
}
