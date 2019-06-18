#![allow(dead_code, unused_imports)]

#[macro_use] extern crate clap;
#[macro_use] extern crate prettytable;
// #[macro_use] extern crate text_io;
extern crate dirs;
extern crate serde;
extern crate serde_json;

use clap::App;
use std::env;
use std::process;

mod server;
mod manager;
mod cli;

use manager::Manager;

fn main() {
    let mut manager = Manager::new();

    let app = cli::new();
    let matches = app.get_matches();
    println!("{:?}", matches.subcommand.unwrap());
    if let Some(subc) = matches.subcommand {
        match subc.name.as_str() {
            "list" => manager.table(),
            _ => {}
        }
    }
}
