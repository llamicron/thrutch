use std::path::{PathBuf, Path};
use std::fs::{File};
use std::fs;
use std::env;

use dirs;
use serde::{Serialize, Deserialize};
use prettytable::{Table, Row, Cell};

use crate::server::Server;
use crate::dev;



#[derive(Debug, Serialize, Deserialize)]
pub struct Manager {
    pub servers: Vec<Server>,
    storage_file: PathBuf,
}

// Associated Items
impl Manager {
    fn storage_file() -> PathBuf {
        let ext = if dev() { "dev" } else { "json" };


        let mut storage_file = dirs::config_dir().expect("Cannot get your config directory");
        storage_file.push("thrutch_data.json");
        storage_file = storage_file.with_extension(ext);

        // Create if not exists
        if !Path::new(storage_file.as_os_str()).exists() {
            match File::create(Path::new(storage_file.as_os_str())) {
                Ok(_) => {},
                Err(e) => println!("Warning! storage file could not be created at {}. Server information will not be saved.", storage_file.display())
            };
        }

        storage_file
    }

    pub fn new() -> Manager {
        let storage_file = Manager::storage_file();

        let mut manager = Manager {
            servers: Vec::new(),
            storage_file
        };
        manager.read_servers();
        manager
    }
}

// Methods
impl Manager {
    pub fn add(&mut self, server: Server) {
        self.servers.push(server);
        self.write_servers();
    }

    pub fn remove(&mut self, name: &str) {
        self.servers.retain(|server| server.name != name);
        self.write_servers();
    }

    pub fn write_servers(&self) {
        let to_write = serde_json::to_string(&self.servers).expect("Couldnt serialize servers!");
        fs::write(&Path::new(self.storage_file.as_os_str()), &to_write).expect("Couldnt write data to file!");
    }

    pub fn read_servers(&mut self) {
        let server_data = fs::read_to_string(&self.storage_file).expect("Couldnt open storage file");
        // TODO: Try to implement from_reader?
        self.servers = match serde_json::from_str(&server_data) {
            Ok(servers) => servers,
            Err(_) => Vec::new()
        };

    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    pub fn teardown() {
        // Truncates both files
        env::set_var("THRUTCH_DEV", "0");
        File::create(Manager::storage_file());
        env::set_var("THRUTCH_DEV", "1");
        File::create(Manager::storage_file());
    }

    #[test]
    fn correct_file() {
        teardown();

        let mut expected_file = dirs::config_dir().expect("Cannot get your config directory");

        let ext = if dev() { "dev" } else { "json" };
        println!("{:?}", dev());
        expected_file.push("thrutch_data");
        expected_file = expected_file.with_extension(ext);

        assert_eq!(Manager::storage_file(), expected_file);

        teardown();
    }

    #[test]
    fn new_manager() {
        Manager::new();
    }

    #[test]
    fn environment_variables() {
        env::set_var("THRUTCH_DEV", "0");
        assert!(!dev());
        assert_eq!(Manager::new().storage_file.extension().unwrap(), "json");
        env::set_var("THRUTCH_DEV", "1");
        assert!(dev());
        assert_eq!(Manager::new().storage_file.extension().unwrap(), "dev");

        teardown();
    }

    #[test]
    fn add_server() {
        let mut manager = Manager::new();
        let server = Server::new("Brewpi", "llamicron", "192.168.0.1", "Outside").expect("Something went wrong :(");

        assert_eq!(manager.servers.len(), 0);
        manager.add(server);
        assert_eq!(manager.servers.len(), 1);

        teardown();
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

        teardown();
    }
}
