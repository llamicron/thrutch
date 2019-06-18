use clap::{Arg, App, SubCommand};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub fn new() -> App<'static, 'static> {
    let app = App::new("Thrutch")
        .version(VERSION)
        .author("llamicron <llamicron@gmail.com>")
        .about("Connect to servers")
        .subcommand(SubCommand::with_name("list").arg(
            Arg::with_name("filter")
            .long("filter")
            .takes_value(true)
            .value_name("searchPattern")
            .short("f")
            .required(false)
            .help("Pattern for searching for servers")
        ))
        .subcommand(SubCommand::with_name("remove"))
        .subcommand(SubCommand::with_name("add"))
        .subcommand(SubCommand::with_name("connect"))
        .subcommand(SubCommand::with_name("backup"));
    app
}

