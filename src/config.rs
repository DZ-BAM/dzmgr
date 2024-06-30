use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use server::Server;

mod modification;
mod server;

#[derive(Clone, Debug, Deserialize, Serialize)]
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
