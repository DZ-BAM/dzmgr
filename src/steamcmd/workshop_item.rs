/// A Steam workshop item.
#[derive(Clone, Copy, Debug)]
pub struct WorkshopItem {
    app_id: u32,
    item_id: u32,
}

impl WorkshopItem {
    /// Create a new `WorkshopItem`.
    #[must_use]
    pub const fn new(app_id: u32, item_id: u32) -> Self {
        Self { app_id, item_id }
    }

    /// Return the app ID.
    #[must_use]
    pub const fn app_id(self) -> u32 {
        self.app_id
    }

    /// Return the item ID.
    #[must_use]
    pub const fn item_id(self) -> u32 {
        self.item_id
    }
}

impl From<(u32, u32)> for WorkshopItem {
    fn from((app_id, item_id): (u32, u32)) -> Self {
        Self::new(app_id, item_id)
    }
}
