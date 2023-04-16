#![allow(dead_code, unused_imports)]

#[macro_use]
extern crate prettytable;
#[macro_use]
extern crate text_io;
extern crate dirs;
extern crate serde;
extern crate serde_json;

mod cli;
mod manager;
mod server;

use cli::CLIError;
use cli::CLI;
use manager::Manager;
use std::env;
use std::process;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

// Prints the help page
fn help() {
    let help_page = format!(
        "
    Thrutch v{}

    Commands:
     - list               List all servers
     - add                Adds a new server
     - remove             Removes a server
     - connect [server]   Connect to a server
     - backup             Backs up the storage file


    Thrutch does not handle passwords or ssh keys. It only stores an ssh connection,
    mostly so you don't have to remember usernames and IPs.

    Thrutch data file: {}
    ",
        VERSION,
        Manager::new().storage_file.display()
    );
    println!("{}", help_page);
}

fn main() -> Result<(), CLIError> {
    let args: Vec<String> = env::args().collect();

    let mut cli = CLI::new(std::io::stdin().lock());

    if args.len() < 2 {
        help();
        process::exit(0);
    };

    if args.contains(&String::from("--version")) {
        println!("Thrutch v{}", VERSION);
        process::exit(0);
    }

    match args[1].as_str() {
        "add" => cli.create()?,
        "remove" => cli.delete()?,
        "connect" => {
            if args.len() > 2 {
                cli.connect(Some(args[2].clone()))?;
            } else {
                cli.connect(None)?;
            }
        }
        // "edit" => manager.edit(),
        "list" => {
            println!("{}", cli.table());
        }
        // "file" => println!("{}", manager.storage_file.display()),
        // "backup" => manager.backup(),
        _ => help(),
    };
    Ok(())
}
