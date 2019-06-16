#![allow(dead_code, unused_imports)]

#[macro_use]
extern crate prettytable;
extern crate dirs;
extern crate serde;
extern crate serde_json;

mod server;
mod manager;

use server::Server;
use manager::Manager;

fn main() {
    let mut man = Manager::new();

    man.table();

    let server = Server::new("Test server", "pi", "192.68.0.1", "Downstairs").expect("Couldnt create server");
    man.add(server);

    man.table();
}
