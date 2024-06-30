use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use super::modification::Modification;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Server {
    mods: Vec<Modification>,
}

impl Server {
    #[must_use]
    pub fn mods(&self) -> &[Modification] {
        &self.mods
    }

    #[must_use]
    pub fn mod_ids(&self) -> HashSet<u32> {
        self.mods.iter().map(Modification::id).collect()
    }
}
