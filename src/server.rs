use serde::{Deserialize, Serialize};
use std::fmt;
use std::net::Ipv4Addr;
use std::process::{Command, ExitStatus};

#[derive(Debug, Serialize, Deserialize)]
pub struct Server {
    pub name: String,
    pub ip: Ipv4Addr,
    pub location: String,
    pub username: String,
}

impl Server {
    pub fn new(
        name: &str,
        username: &str,
        ip: &str,
        location: &str,
    ) -> Result<Server, &'static str> {
        let parsed_ip = match ip.parse::<Ipv4Addr>() {
            Ok(x) => x,
            Err(_) => return Err("Invalid IP Address"),
        };

        let server = Server {
            name: name.to_owned(),
            username: username.to_owned(),
            ip: parsed_ip,
            location: location.to_owned(),
        };
        Ok(server)
    }

    pub fn address(&self) -> String {
        format!("{}@{}", self.username, self.ip)
    }

    pub fn connect(&self) -> ExitStatus {
        let mut ssh = Command::new("sh");
        ssh.arg("-c");
        ssh.arg(format!("ssh {}", self.address()));

        return ssh.status().expect("Something went horribly wrong :(");
    }
}

impl fmt::Display for Server {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}: {}@{}; {}",
            self.name, self.username, self.ip, self.location
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tester_server() -> Server {
        Server::new("Brewpi", "llamicron", "192.168.0.1", "Outside")
            .expect("Something went wrong :(")
    }

    #[test]
    fn new_server() {
        let server = tester_server();
        assert_eq!(server.ip, Ipv4Addr::new(192, 168, 0, 1));
        assert_eq!(server.name, String::from("Brewpi"));
        assert_eq!(server.username, String::from("llamicron"));
        assert_eq!(server.location, String::from("Outside"));
    }

    #[test]
    fn address() {
        let server = tester_server();
        assert_eq!(server.address(), String::from("llamicron@192.168.0.1"));
    }

    #[test]
    fn display() {
        let server = tester_server();
        assert_eq!(
            format!("{}", server),
            "Brewpi: llamicron@192.168.0.1; Outside"
        );
    }
}
