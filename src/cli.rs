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
    use std::io::BufWriter;

    use super::*;

    #[test]
    fn test_build_cli_with_custom_input() {
        let cursor = std::io::Cursor::new("testing input");
        let mut cli = CLI::new(cursor);
        assert_eq!(cli.read_line().unwrap(), "testing input");
    }
}
