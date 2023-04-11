//! Contains methods for interacting with the manager through the command line.
//! These functions will ask for user input through the command line.

use std::io::{BufRead, self};


pub struct CLI<R: BufRead> {
    reader: R
}

impl<R: BufRead> CLI<R> {
    pub fn new(reader: R) -> CLI<R> {
        CLI { reader }
    }    

    pub fn read_line(&mut self) -> Result<String, io::Error> {
        let mut buffer = String::new();
        self.reader.read_line(&mut buffer)?;
        Ok(buffer)
    }

}






#[cfg(test)]
mod tests {
    use std::io::{BufWriter, Cursor};

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
        // assert!(cli("").required_input().is_err());
        // assert_eq!(cli("something").required_input(), Ok("something"));
        // assert!(cli("   ").required_input().is_err());
    }
}
