# Work: a terminal todo manager

![screencapture](https://raw.githubusercontent.com/g-w1/work/master/pics/out.gif)

Features:

- [x] use a sqlite database with [rusqlite](https://docs.rs/rusqlite/0.23.1/rusqlite/)

- [x] command line parsing with [clap.rs](https://clap.rs)

- [x] fuzzy finder built in with [skim](https://github.com/lotabout/skim)

- [ ] config file parsing with [serde.rs](https://serde.rs)

Limits:

- Only works on unix systems for now (I dont have any windows stuff to test on).

Install:

```bash
git clone https://github.com/g-w1/work.git
cd work
cargo install --path .
```

(I may put on crates.io soon!)
