//! The manager handles maintenance of a collection of servers, ie.
//! writing them to a file and whatnot
use std::env;
use std::fs::{self, File};
use std::net::Ipv4Addr;
use std::path::{Path, PathBuf};
use std::{io, process};

use dirs;
use log::*;
use prettytable::{Cell, Row, Table};
use serde::{Deserialize, Serialize};

use crate::server::Server;

#[derive(Debug, Serialize, Deserialize)]
pub struct Manager {
    pub servers: Vec<Server>,
    pub storage_file: PathBuf,
}

// Associated Items
impl Manager {
    /// Make a new manager. This will call storage_file(), creating the file
    /// if it doesn't already exist
    pub fn new() -> Manager {
        let storage_file = Manager::storage_file();

        let mut manager = Manager {
            servers: Vec::new(),
            storage_file,
        };
        manager.read_servers();
        manager
    }

    /// Return the path to the storage file, creating it if it doesn't
    /// already exist
    fn storage_file() -> PathBuf {
        // First, try to get the file from an env variable
        // If that doesn't work, get the default which is in the config dir
        let storage_file = match std::env::var("THRUTCH_STORAGE_FILE") {
            Ok(overwrite_path) => PathBuf::from(overwrite_path),
            Err(_) => {
                let mut default_path =
                    dirs::config_dir().expect("Cannot get your config directory");
                default_path.push("thrutch_data.json");
                default_path
            }
        };

        info!("Using storage file {}", storage_file.display());

        // Create if not exists
        if !storage_file.exists() {
            info!("Storage file does not exist, creating it now...");
            match File::create(&storage_file) {
                Ok(_) => info!("Storage file created at {}", storage_file.display()),
                Err(_) => error!("Error! storage file could not be created at {}. Server information will not be saved.", storage_file.display())
            };
        }

        storage_file
    }

    pub fn write_servers(&self) {
        let to_write = serde_json::to_string(&self.servers).expect("Couldnt serialize servers!");
        fs::write(&Path::new(self.storage_file.as_os_str()), &to_write)
            .expect("Couldnt write data to file!");
    }

    pub fn read_servers(&mut self) {
        let server_data =
            fs::read_to_string(&self.storage_file).expect("Couldnt open storage file");
        // TODO: Try to implement from_reader?
        self.servers = match serde_json::from_str(&server_data) {
            Ok(servers) => servers,
            Err(_) => Vec::new(),
        };
    }

    pub fn backup(&mut self) {
        let backup_file = self.storage_file.with_extension("bak");
        println!("Backing up storage file to {}...", backup_file.display());

        match fs::copy(&self.storage_file, &backup_file) {
            Ok(_) => println!("Success"),
            Err(error) => println!("Error. Could not backup: {}", error),
        };
    }
}
