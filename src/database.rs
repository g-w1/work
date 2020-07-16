use crate::config::DATABASE_URL;
use rusqlite::{Connection, Result};
#[derive(Debug)]
pub struct Event<'a> {
    id: usize,
    summary: &'a str,
    done: bool,
}

pub fn start() {
    let conn = Connection::open(DATABASE_URL);

}
