use std::env;
use std::path::PathBuf;
use std::process::Command;

pub use app::App;
pub use workshop_item::WorkshopItem;

mod app;
mod workshop_item;

const STEAM_CMD: &str = "steamcmd";

/// Builder to construct a steamcmd invocation, returning a [`Command`].
#[derive(Debug)]
pub struct SteamCmd {
    steamcmd: PathBuf,
    force_install_dir: Option<PathBuf>,
    user: Option<String>,
    app_update: Vec<(App, bool)>,
    workshop_download_item: Vec<(App, WorkshopItem)>,
}

impl SteamCmd {
    pub fn new<T>(steamcmd: T) -> Self
    where
        T: Into<PathBuf>,
    {
        Self {
            steamcmd: steamcmd.into(),
            force_install_dir: None,
            user: None,
            app_update: Vec::new(),
            workshop_download_item: Vec::new(),
        }
    }

    /// Force the installation directory to the given path.
    #[must_use]
    pub fn force_install_dir<T>(mut self, dir: T) -> Self
    where
        T: Into<PathBuf>,
    {
        self.force_install_dir.replace(dir.into());
        self
    }

    /// Log in as the given user.
    #[must_use]
    pub fn user<T>(mut self, user: T) -> Self
    where
        T: Into<String>,
    {
        self.user.replace(user.into());
        self
    }

    /// Update the app with the given ID.
    #[must_use]
    pub fn app_update<T>(mut self, app: T, validate: bool) -> Self
    where
        T: Into<App>,
    {
        self.app_update.push((app.into(), validate));
        self
    }

    /// Download the given workshop item.
    #[must_use]
    pub fn workshop_download_item<A, W>(mut self, app: A, workshop_item: W) -> Self
    where
        A: Into<App>,
        W: Into<WorkshopItem>,
    {
        self.workshop_download_item
            .push((app.into(), workshop_item.into()));
        self
    }

    /// Build a [`Command`] from the builder.
    ///
    /// Only use this method if you want to construct an interactive `steamcmd`.
    /// Otherwise, use [`Self::quit()`] instead.
    #[must_use]
    pub fn build(self) -> Command {
        let mut command = Command::new(self.steamcmd);

        if let Some(dir) = self.force_install_dir {
            command.arg("+force_install_dir").arg(dir);
        }

        if let Some(user) = self.user {
            command.arg("+login").arg(user);
        }

        for (app, validate) in self.app_update {
            command.arg("+app_update").arg(app.to_string());

            if validate {
                command.arg("validate");
            }
        }

        for (app, workshop_item) in self.workshop_download_item {
            command
                .arg("+workshop_download_item")
                .arg(app.to_string())
                .arg(workshop_item.to_string());
        }

        command
    }

    /// Quit `steamcmd`. This will also finish the build process and return the [`Command`].
    #[must_use]
    pub fn quit(self) -> Command {
        let mut command = self.build();
        command.arg("+quit");
        command
    }
}

impl Default for SteamCmd {
    fn default() -> Self {
        Self::new(env::var_os("STEAMCMD").unwrap_or_else(|| STEAM_CMD.into()))
    }
}
