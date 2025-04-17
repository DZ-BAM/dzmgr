use std::fmt::Display;

/// Represents an app for `steamcmd`.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct App(u32);

impl App {
    /// Create a new `AppUpdate`.
    #[must_use]
    pub const fn new(id: u32) -> Self {
        Self(id)
    }

    /// Return the app ID.
    #[must_use]
    pub const fn id(self) -> u32 {
        self.0
    }
}

impl Display for App {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
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
