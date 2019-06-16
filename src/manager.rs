use std::path::{PathBuf, Path};
use std::fs::{self, File};
use std::io;
use std::env;
use std::net::Ipv4Addr;

use dirs;
use serde::{Serialize, Deserialize};
use prettytable::{Table, Row, Cell};

use crate::server::Server;


#[derive(Debug, Serialize, Deserialize)]
pub struct Manager {
    pub servers: Vec<Server>,
    storage_file: PathBuf,
}

// Associated Items
impl Manager {
    pub fn new() -> Manager {
        let storage_file = Manager::storage_file();

        let mut manager = Manager {
            servers: Vec::new(),
            storage_file
        };
        manager.read_servers();
        manager
    }

    fn storage_file() -> PathBuf {
        let mut storage_file = dirs::config_dir().expect("Cannot get your config directory");

        // TODO: Change this to .json for prod
        storage_file.push("thrutch_data.dev");

        // Create if not exists
        if !Path::new(storage_file.as_os_str()).exists() {
            match File::create(Path::new(storage_file.as_os_str())) {
                Ok(_) => {},
                Err(_) => println!("Warning! storage file could not be created at {}. Server information will not be saved.", storage_file.display())
            };
        }

        storage_file
    }

    fn get_user_input(prompt: &str) -> String {
        print!("{}", prompt);
        io::Write::flush(&mut io::stdout()).expect("flush failed!");

        let mut value = String::new();
        if let Err(err) = io::stdin().read_line(&mut value) {
            println!("Could not parse input: {}", err);
        };

        value.trim().to_owned()
    }

    fn required_input(prompt: &str) -> String {
        loop {
            let value = Manager::get_user_input(prompt);
            if value.len() < 1 {
                println!("Need an input here");
            } else if value.len() > 25 {
                println!("Input too long");
            } else {
                return value;
            }
        }
    }

    // Keeps asking for input until a valid IPv4 address is entered
    fn ip_required_input(prompt: &str) -> Ipv4Addr {
        loop {
            let ip = Manager::required_input(prompt);
            match ip.parse::<Ipv4Addr>() {
                Ok(addr) => return addr,
                Err(_) => { println!("Need a valid IPv4 address") }
            }
        }
    }


}

// Methods
impl Manager {
    fn add(&mut self, server: Server) {
        self.servers.push(server);
        self.write_servers();
    }

    fn remove(&mut self, name: &str) {
        self.servers.retain(|server| server.name != name);
        self.write_servers();
    }

    fn write_servers(&self) {
        let to_write = serde_json::to_string(&self.servers).expect("Couldnt serialize servers!");
        fs::write(&Path::new(self.storage_file.as_os_str()), &to_write).expect("Couldnt write data to file!");
    }

    fn read_servers(&mut self) {
        let server_data = fs::read_to_string(&self.storage_file).expect("Couldnt open storage file");
        // TODO: Try to implement from_reader?
        self.servers = match serde_json::from_str(&server_data) {
            Ok(servers) => servers,
            Err(_) => Vec::new()
        };

    }

    // User functions
    // Prints a table of all servers
    pub fn table(&self) {
        let mut table = Table::new();

        table.add_row(row!["Name", "Address", "Location"]);
        for server in &self.servers {
            table.add_row(row![&server.name, server.address(), &server.location]);
        }

        table.printstd();
    }

    // Creates a new server through user input, uses `add`
    pub fn create(&mut self) {
        let name = Manager::required_input("Server name: ");
        let username = Manager::required_input("Username: ");
        let ip: Ipv4Addr = Manager::ip_required_input("IP: ");
        // This one isn't required
        let location = Manager::get_user_input("Location: ");

        if let Ok(server) = Server::new(&name, &username, &format!("{}", ip), &location) {
            self.add(server);
        } else {
            println!("Something went wrong when adding a server");
        }

        self.table();
    }

    // Asks for user input and removes that server, uses `remove`
    pub fn delete(&mut self) {
        self.table();
        let old_len = self.servers.len();

        let name = Manager::required_input("Server to remove: ");

        self.remove(&name);
        if self.servers.len() < old_len {
            println!("Server removed");
        } else {
            println!("Couldn't find that server");
        }
    }

    // Ask for a name and try to connect to that server, uses `server.connect`
    pub fn connect(&mut self) {
        self.table();
        let name = Manager::required_input("Connect to: ");

        if let Some(server) = self.servers.iter().find(|&server| server.name == name) {
            server.connect();
        } else {
            println!("Couldn't find server '{}'", name);
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    pub fn teardown() {
        // Truncates the file
        File::create(Manager::storage_file()).expect("Couldn't truncate file");
    }

    #[test]
    fn new_manager() {
        teardown();
        Manager::new();
        teardown();
    }

    #[test]
    fn add_server() {
        teardown();

        let mut manager = Manager::new();
        let server = Server::new("Brewpi", "llamicron", "192.168.0.1", "Outside").expect("Something went wrong :(");

        assert_eq!(manager.servers.len(), 0);
        manager.add(server);
        assert_eq!(manager.servers.len(), 1);

        teardown();
    }

    #[test]
    fn remove_server() {
        teardown();

        let mut manager = Manager::new();
        let server1 = Server::new("Some Server", "llamicron", "192.168.0.1", "Outside").expect("Something went wrong :(");
        let server2 = Server::new("remove me", "llamicron", "192.168.0.1", "Outside").expect("Something went wrong :(");

        manager.add(server1);
        manager.add(server2);

        assert_eq!(manager.servers.len(), 2);
        manager.remove("remove me");
        assert_eq!(manager.servers.len(), 1);
        manager.remove("");
        assert_eq!(manager.servers.len(), 1);

        teardown();
    }

}
