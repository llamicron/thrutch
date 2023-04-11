#![allow(dead_code, unused_imports)]

#[macro_use] extern crate prettytable;
#[macro_use] extern crate text_io;
extern crate dirs;
extern crate serde;
extern crate serde_json;


mod server;
mod manager;

use std::env;
use std::process;
use manager::Manager;



const VERSION: &'static str = env!("CARGO_PKG_VERSION");

// Prints the help page
fn help() {
    let help_page = format!("
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
    ", VERSION, Manager::new().storage_file.display());
    println!("{}", help_page);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut manager = Manager::new();
    
    if args.len() < 2 {
        help();
        process::exit(0);
    };
    
    if args.contains(&String::from("--version")) {
        println!("Thrutch v{}", VERSION);
        process::exit(0);
    }
    
    match args[1].as_str() {
        "add" => manager.create(),
        "remove" => manager.delete(),
        "connect" => {
            if args.len() > 2 {
                manager.connect(Some(args[2].clone()));
            } else {
                manager.connect(None);
            }
        },
        "edit" => manager.edit(),
        "list" => manager.table(),
        // "file" => println!("{}", manager.storage_file.display()),
        "backup" => manager.backup(),
        _ => help()
    }
}
