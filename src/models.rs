#[derive(Queryable)]
pub struct Event {
    pub id: i32,
    pub summary: String,
    pub time: String,
    pub done: bool,
}
