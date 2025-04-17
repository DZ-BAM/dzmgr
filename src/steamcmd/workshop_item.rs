use std::fmt::Display;

/// A Steam workshop item.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct WorkshopItem(u32);

impl WorkshopItem {
    /// Create a new `WorkshopItem`.
    #[must_use]
    pub const fn new(id: u32) -> Self {
        Self(id)
    }
    /// Return the item ID.
    #[must_use]
    pub const fn id(self) -> u32 {
        self.0
    }

    /// Return the URL to the Steam workshop website.
    #[must_use]
    pub fn url(self) -> String {
        format!(
            "https://steamcommunity.com/sharedfiles/filedetails/?id={}",
            self.0
        )
    }
}

impl Display for WorkshopItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl From<u32> for WorkshopItem {
    fn from(item_id: u32) -> Self {
        Self::new(item_id)
    }
}

impl From<WorkshopItem> for u32 {
    fn from(item: WorkshopItem) -> Self {
        item.0
    }
}
