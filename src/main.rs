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

use std::env;

fn main() {
}
