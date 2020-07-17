use clap::App;

let matches = App::new("Work")
.version("0.1.0")
.author("Jacob G-W <jacoblevgw@gmail.com>")
.about("A cli todo app")
.arg("-c, --config=[FILE]")
.arg("-d, --database=[FILE]")
.subcommand(
    App::new("ls")
    .about("list events in database")
    );
