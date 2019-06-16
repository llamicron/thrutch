use std::path::PathBuf;

use dirs;
use serde::{Serialize, Deserialize};
use prettytable::{Table, Row, Cell};

use crate::server::Server;

#[derive(Debug, Serialize, Deserialize)]
pub struct Manager {
    servers: Vec<Server>,
    storage_file: PathBuf,
}

// Associated Items
impl Manager {
    fn new() -> Manager {
        let mut storage_file = dirs::config_dir().expect("Cannot get your config directory");
        storage_file.push("thrutch_data.json");

        let servers = vec![];

        Manager {
            servers,
            storage_file
        }
    }
}

// Methods
impl Manager {
    fn add(&mut self, server: Server) {
        self.servers.push(server);
    }

    fn remove(&mut self, name: &str) {
        self.servers.retain(|server| server.name != name);
    }

    fn write_servers(&self) {
        unimplemented!();
    }

    fn read_servers(&self) -> Vec<Server> {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_manager() {
        let manager = Manager::new();

        let mut expected_file = dirs::config_dir().expect("Cannot get your config directory");
        expected_file.push("thrutch_data.json");

        assert_eq!(manager.servers.len(), 0);
        assert_eq!(manager.storage_file, expected_file);
    }

    #[test]
    fn add_server() {
        let mut manager = Manager::new();
        let server = Server::new("Brewpi", "llamicron", "192.168.0.1", "Outside").expect("Something went wrong :(");

        assert_eq!(manager.servers.len(), 0);
        manager.add(server);
        assert_eq!(manager.servers.len(), 1);
    }

    #[test]
    fn remove_server() {
        let mut manager = Manager::new();
        let server1 = Server::new("Some Server", "llamicron", "192.168.0.1", "Outside").expect("Something went wrong :(");
        let server2 = Server::new("remove me", "llamicron", "192.168.0.1", "Outside").expect("Something went wrong :(");

        manager.add(server1);
        manager.add(server2);

        assert_eq!(manager.servers.len(), 2);
        manager.remove("remove me");
        assert_eq!(manager.servers.len(), 1);
    }
}
