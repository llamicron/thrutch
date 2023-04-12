//! Contains methods for interacting with the manager through the command line.
//! These functions will ask for user input through the command line.

use std::{
    io::{self, BufRead},
    str::FromStr,
};

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
}

impl<R: BufRead> CLI<R> {
    pub fn new(reader: R) -> CLI<R> {
        CLI { reader }
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
}
