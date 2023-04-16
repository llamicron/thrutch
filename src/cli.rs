//! Contains methods for interacting with the manager through the command line.
//! These functions will ask for user input through the command line.

use std::{
    io::{self, BufRead},
    net::Ipv4Addr,
    str::FromStr,
};

use log::error;
use prettytable::Table;

use crate::{manager::Manager, server::Server};

#[derive(Debug, thiserror::Error)]
pub enum CLIError {
    #[error("Required value")]
    NoValue,
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Parse error: {0}")]
    ParseError(String),
}

pub struct CLI<R: BufRead> {
    reader: R,
    manager: Manager,
}

impl<R: BufRead> CLI<R> {
    pub fn new(reader: R) -> CLI<R> {
        CLI {
            reader,
            manager: Manager::new(),
        }
    }

    /// Just reads a line from the buffer
    pub fn read_line(&mut self) -> Result<String, CLIError> {
        let mut buffer = String::new();
        self.reader.read_line(&mut buffer)?;
        Ok(buffer)
    }

    /// Sets the prompt and returns self so you can use this
    /// like a builder pattern
    ///
    /// ## Example
    /// ```rust
    /// use thrutch::cli::CLI;
    ///
    /// let mut cli = CLI::new(std::io::stdin().lock());
    ///
    /// cli.prompt("Enter your name: ").required_input();
    /// ```
    pub fn prompt(&mut self, prompt: &str) -> &mut Self {
        print!("{}", prompt);
        io::Write::flush(&mut io::stdout()).expect("couldn't flush output");
        self
    }

    /// Will return an Ok(String) with some content (non empty) or an error
    pub fn required_input(&mut self) -> Result<String, CLIError> {
        let input = self.read_line()?;
        if input.trim().is_empty() {
            Err(CLIError::NoValue)
        } else {
            Ok(input)
        }
    }

    /// Asks for input and then parses it to the type T
    pub fn parse_input<T: FromStr>(&mut self) -> Result<T, CLIError> {
        let input = self.read_line()?;
        input.parse::<T>().map_err(|_| {
            CLIError::ParseError(format!("Couldn't parse string to proper value: {input}"))
        })
    }
}

impl<R: BufRead> CLI<R> {
    pub fn table(&self) -> String {
        let mut table = Table::new();

        table.add_row(row!["Name", "Address", "Location"]);
        for server in &self.manager.servers {
            table.add_row(row![&server.name, server.address(), &server.location]);
        }

        format!("{}", table)
    }

    pub fn create(&mut self) -> Result<(), CLIError> {
        println!("Adding a new server");
        let name = self.prompt("Name: ").required_input()?;
        let username = self.prompt("Username: ").required_input()?;
        let ip = self.prompt("IPv4: ").parse_input::<Ipv4Addr>()?;
        let location = self.prompt("Location (opt.): ").read_line()?;

        if let Ok(server) = Server::new(&name, &username, &format!("{}", ip), &location) {
            self.manager.servers.push(server);
            self.manager.write_servers();
        } else {
            error!("Couldn't create a server");
        }

        Ok(())
    }

    pub fn delete(&mut self) -> Result<(), CLIError> {
        println!("{}", self.table());
        let old_len = self.manager.servers.len();

        println!("Deleting a server");
        let name = self.prompt("Name: ").required_input()?;

        self.manager.servers.retain(|s| s.name != name);

        // TODO: improve this with Err()
        if self.manager.servers.len() < old_len {
            self.manager.write_servers();
            println!("Server removed");
        } else {
            eprintln!("Couldn't find that server");
        }

        Ok(())
    }

    pub fn connect(&mut self, server_name: Option<String>) -> Result<(), CLIError> {
        // This will crash, fit it later
        let mut name: String = server_name.unwrap_or_else(|| {
            println!("{}", self.table());
            self.prompt("Name: ").required_input().unwrap()
        });

        if let Some(server) = self.manager.servers.iter().find(|s| s.name == name) {
            println!("Connecting to {}...", server.name);
            server.connect();
        } else {
            eprintln!("Couln't find that server");
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::{
        io::{BufWriter, Cursor},
        net::Ipv4Addr,
    };

    use super::*;

    // This is equivalent to starting the CLI and the user
    // inputting a string
    fn cli(input: &str) -> CLI<Cursor<&str>> {
        let cursor = std::io::Cursor::new(input);
        CLI::new(cursor)
    }

    #[test]
    fn test_build_cli_with_custom_input() {
        let mut cli = cli("testing input");
        assert_eq!(cli.read_line().unwrap(), "testing input");
    }

    #[test]
    fn test_required_input() {
        // idk if this is how I want it to work
        assert!(cli("").required_input().is_err());
        assert_eq!(
            cli("something").required_input().unwrap(),
            "something".to_string()
        );
        assert!(cli("   ").required_input().is_err());
    }

    #[test]
    fn test_input_parse() {
        assert!(cli("0.0.0.0").parse_input::<Ipv4Addr>().is_ok());
        assert!(cli("9").parse_input::<u8>().is_ok());
        assert!(cli("not a number").parse_input::<usize>().is_err());
    }

    #[test]
    fn test_table() {
        let cli = cli("");
        assert!(cli.table().is_ascii());
    }
}
