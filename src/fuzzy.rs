extern crate skim;
use crate::Event;
use database::*;
use skim::prelude::*;

fn sk_all_events(conn: &Connection) {
    // just getting a Vec<Event> from all events
    // TODO refactor this. I also use in frontend.rs:107
    let events_special_stuff = get_all_events(&conn);
    let events = match events_special_stuff {
        Ok(x) => x,
        Err(_) => {
            eprintln!("An Error Occured.\nTry adding some events to list first.\nWhoops");
            return Ok(());
        }
    };
    let mut events_out: Vec<Event> = Vec::new();
    for event_res in events {
        let event = match event_res {
            Ok(x) => x,
            Err(_) => {
                eprintln!("Error Occured. Whoops.");
                return Ok(());
            }
        };
        events_out.push(event);
    }
    // skim stuff
    let options = SkimOptions::default();
    let (tx_item, rx_item): (SkimItemSender, SkimItemReceiver) = unbounded();
    for event in events_out {
        let _ = tx_item.send(Arc::new(event));
    }
    drop(tx_item);
    let selected_items = Skim::run_with(&options, Some(rx_item))
        .map(|out| out.selected_items)
        .unwrap_or_else(|| Vec::new());
}

impl SkimItem for Event {
    fn display(&self) -> Cow<AnsiString> {
        Cow::Owned(self.summary.as_str().into())
    }
    fn text(&self) -> Cow<str> {
        Cow::Borrowed(&self.summary)
    }
}
