use std::collections::HashSet;

use super::modification::Modification;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
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
