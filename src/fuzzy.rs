extern crate skim;
use crate::database::*;
use crate::event::Event;
use rusqlite::Connection;
use skim::prelude::*;

pub fn sk_all_events(conn: &Connection) -> Option<Vec<String>> {
    // just getting a Vec<Event> from all events
    // TODO refactor this. I also use in frontend.rs:107
    let events_special_stuff = get_all_events(&conn);
    let events = match events_special_stuff {
        Ok(x) => x,
        Err(_) => {
            eprintln!("An Error Occured.\nTry adding some events to list first.\nWhoops");
            return None;
        }
    };
    let mut events_out: Vec<Event> = Vec::new();
    for event_res in events {
        let event = match event_res {
            Ok(x) => x,
            Err(_) => {
                eprintln!("Error Occured. Whoops.");
                return None;
            }
        };
        events_out.push(event);
    }
    // skim stuff
    let options = SkimOptionsBuilder::default().layout("reverse").multi(false).build().unwrap();
    let (tx_item, rx_item): (SkimItemSender, SkimItemReceiver) = unbounded();
    for event in events_out {
        let _ = tx_item.send(Arc::new(event));
    }
    drop(tx_item);
    let selected_items = Skim::run_with(&options, Some(rx_item))
        .map(|out| out.selected_items)
        .unwrap_or_else(|| Vec::new());
    let mut items_to_return = Vec::new();
    for item in selected_items.iter() {
        items_to_return.push(item.output().to_string());
    }
    // for item in selected_items.iter() {
    //     println!("{}", item.output());
    // }
    Some(items_to_return)
}

// pub fn update_sk(conn: &Connection) {

// }
impl SkimItem for Event {
    fn display(&self) -> Cow<AnsiString> {
        Cow::Owned(self.summary.as_str().into())
    }
    fn text(&self) -> Cow<str> {
        Cow::Owned(self.get_text())
    }
}
impl Event {
    fn get_text(&self) -> String {
        format!("{}: {}", self.id.unwrap(), self.summary)

    }

}
