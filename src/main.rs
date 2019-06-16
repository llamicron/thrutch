
mod server;

use std::process::Command;
use server::Server;
use std::str;

fn main() {
    let server = Server::new("Brewpi", "llamicron", "192.168.0.72", "right hur dude").expect("Something went wrong :(");
    server.connect();

    // let mut ssh = Command::new("sh");
    // ssh.arg("-c").arg(format!("{} {}", "ssh", server.address()));
    // // ssh.arg(server.address());

    // let output = ssh.output().expect("Something went wrong :(");
    // println!("{:?}", output);
}
