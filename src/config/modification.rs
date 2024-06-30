use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Modification {
    id: u32,
    name: Option<String>,
}

impl Modification {
    #[must_use]
    pub const fn new(id: u32, name: Option<String>) -> Self {
        Self { id, name }
    }

    #[must_use]
    pub const fn id(&self) -> u32 {
        self.id
    }

    pub const fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }
}

impl From<u32> for Modification {
    fn from(id: u32) -> Self {
        Self { id, name: None }
    }
}

impl From<Modification> for u32 {
    fn from(m: Modification) -> Self {
        m.id
    }
}
