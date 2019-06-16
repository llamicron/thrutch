use std::net::Ipv4Addr;
use std::fmt;

pub struct Server {
    name: String,
    ip: Ipv4Addr,
    location: String,
    username: String
}

impl Server {
    fn new(name: &str, username: &str, ip: &str, location: &str) -> Result<Server, &'static str> {
        let parsed_ip = match ip.parse::<Ipv4Addr>() {
            Ok(x) => x,
            Err(_) => return Err("Invalid IP Address")
        };

        let server = Server {
            name: name.to_owned(),
            username: username.to_owned(),
            ip: parsed_ip,
            location: location.to_owned(),
        };
        Ok(server)
    }
}

impl fmt::Display for Server {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}@{}; {}", self.name, self.username, self.ip, self.location)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_server() {
        let server = Server::new("Brewpi", "llamicron", "192.168.0.1", "Outside").expect("Something went wrong :(");
        assert_eq!(server.ip, Ipv4Addr::new(192, 168, 0, 1));
        assert_eq!(server.name,     String::from("Brewpi"));
        assert_eq!(server.username, String::from("llamicron"));
        assert_eq!(server.location, String::from("Outside"));
    }

    #[test]
    fn display() {
        let server = Server::new("Brewpi", "llamicron", "192.168.0.1", "Outside").expect("Something went wrong :(");
        assert_eq!(format!("{}", server), "Brewpi: llamicron@192.168.0.1; Outside");
    }
}
