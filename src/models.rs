#[derive(Queryable)]
pub struct Event {
    pub id: i32,
    pub summary: String,
    pub done: bool,
}

use super::schema::events;

#[derive(Insertable)]
#[table_name="events"]
pub struct NewEvent<'a> {
    pub summary: &'a str,
}
