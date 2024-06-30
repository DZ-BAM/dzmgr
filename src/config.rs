mod modification;
mod server;

use std::collections::{HashMap, HashSet};

use serde::Deserialize;
use server::Server;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    servers: HashMap<String, Server>,
}

impl Config {
    #[must_use]
    pub const fn servers(&self) -> &HashMap<String, Server> {
        &self.servers
    }

    #[must_use]
    pub fn get(&self, server: &str) -> Option<&Server> {
        self.servers.get(server)
    }
}
