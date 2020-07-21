# Work: a terminal todo manager

![screencapture](https://raw.githubusercontent.com/g-w1/work/master/pics/out.gif)

Features:

- [x] use a sqlite database with [rusqlite](https://docs.rs/rusqlite/0.23.1/rusqlite/)

- [x] command line parsing with [clap.rs](https://clap.rs)

- [x] fuzzy finder built in with [skim](https://github.com/lotabout/skim)

- [ ] config file parsing with [serde.rs](https://serde.rs)

Instructions:

``work ls`` to list all of your events.
Ex:
```
[user@hostname ~]$ work ls
________________________________________
DONE| ID| SUMMARY
----------------------------------------
❌ | 1 | test
❌ | 2 | another another test
✅ | 3 | another test
----------------------------------------
```

``work add <string of an event to add>`` add an event. Ex: ``work add "test"``

``work rm <event id>`` to remove an event. Ex: ``work rm 1``

``work rm fzf`` to remove an event or multiple through a fuzzy finder. Search through the events and press tab to select multiple.

``work rm all`` delete the whole database of events.

``work edit <event id>`` get to a prompt to edit an event. This will allow you to edit the name and make it done. Ex: ``work edit 1``

``work edit fzf`` the same as ``work rm fzf`` except multiple are not allowed.



Limits:

- Only works on unix systems for now (I dont have any windows stuff to test on for now).

Install:

```bash
git clone https://github.com/g-w1/work.git
cd work
cargo install --path .
```

(I may put on crates.io soon!)
