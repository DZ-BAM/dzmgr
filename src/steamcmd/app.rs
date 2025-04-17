use crate::steamcmd::workshop_item::WorkshopItem;

/// Represents an app for `steamcmd`.
#[derive(Clone, Copy, Debug)]
pub struct App(u32);

impl App {
    /// Create a new `AppUpdate`.
    #[must_use]
    pub const fn new(app_id: u32) -> Self {
        Self(app_id)
    }

    /// Return the app ID.
    #[must_use]
    pub const fn app_id(self) -> u32 {
        self.0
    }

    /// Create a [`WorkshopItem`] for this app.
    #[must_use]
    pub const fn workshop_item(self, item_id: u32) -> WorkshopItem {
        WorkshopItem::new(self.0, item_id)
    }
}

impl From<u32> for App {
    fn from(app_id: u32) -> Self {
        Self::new(app_id)
    }
}

impl From<App> for u32 {
    fn from(app: App) -> Self {
        app.0
    }
}
