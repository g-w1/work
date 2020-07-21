extern crate skim;
use crate::database::*;
use crate::event::Event;
use crate::frontend::delete_event;
use crate::frontend::update_event_from_id;
use rusqlite::Connection;
use skim::prelude::*;

pub fn sk_all_events(
    conn: &Connection,
    multi: bool,
) -> Result<Option<Vec<String>>, rusqlite::Error> {
    // just getting a Vec<Event> from all events
    // TODO refactor this. I also use in frontend.rs:107
    let events = get_all_events(&conn)?;
    let mut events_out: Vec<Event> = Vec::new();
    for event_res in events {
        events_out.push(event_res?);
    }
    // skim stuff
    let options = SkimOptionsBuilder::default()
        .layout("reverse")
        .multi(multi)
        .build()
        .unwrap();
    let (tx_item, rx_item): (SkimItemSender, SkimItemReceiver) = unbounded();
    for event in events_out {
        let _ = tx_item.send(Arc::new(event));
    }
    drop(tx_item);
    let selected_items = Skim::run_with(&options, Some(rx_item))
        .map(|out| out.selected_items)
        .unwrap_or_else(|| Vec::new());

    let items_to_return: Option<Vec<String>> = match selected_items.len() {
        0 => None,
        _ => {
            let mut items = Vec::new();
            for item in selected_items.iter() {
                items.push(item.output().to_string());
            }
            Some(items)
        }
    };
    Ok(items_to_return)
}

pub fn update_sk(conn: &Connection) -> Result<(), rusqlite::Error> {
    match sk_all_events(&conn, false)? {
        Some(x) => {
            update_event_from_id(
                &conn,
                x[0].split(':').collect::<Vec<&str>>()[0]
                    .parse::<u32>()
                    .unwrap(),
            )?;
        }
        _ => {}
    }
    Ok(())
}
pub fn rm_sk(conn: &Connection) -> Result<(), rusqlite::Error> {
    match sk_all_events(&conn, true)? {
        Some(x) => {
            for i in &x {
                delete_event(
                    &conn,
                    get_event_by_id(
                        &conn,
                        i.split(':').collect::<Vec<&str>>()[0]
                            .parse::<u32>()
                            .unwrap(),
                    ),
                )?;
                if x.len() > 1 {
                    println!();
                }
            }
        }
        _ => {}
    }
    Ok(())
}

impl SkimItem for Event {
    fn display(&self) -> Cow<AnsiString> {
        Cow::Owned(self.get_summ_with_id().into())
    }
    fn text(&self) -> Cow<str> {
        Cow::Owned(self.get_summ_with_id())
    }
}

impl Event {
    // fn get_summ_with_done(&self) -> String {
    //     format!(
    //         "{}  {}",
    //         {
    //             if self.done {
    //                 "✅"
    //             } else {
    //                 "❌"
    //             }
    //         },
    //         self.summary
    //     )
    // }
    fn get_summ_with_id(&self) -> String {
        format!("{}: {}", self.id.unwrap(), self.summary)
    }
}
