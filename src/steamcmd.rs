use std::env;
use std::path::PathBuf;
use std::process::Command;

const STEAM_CMD: &str = "steamcmd";

/// Builder to construct a steamcmd invocation, returning a [`Command`].
#[derive(Debug)]
pub struct SteamCmd {
    steamcmd: PathBuf,
    force_install_dir: Option<PathBuf>,
    user: Option<String>,
    app_update: Vec<(u32, bool)>,
    workshop_download_item: Vec<(u32, u32)>,
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
    pub fn app_update(mut self, id: u32, validate: bool) -> Self {
        self.app_update.push((id, validate));
        self
    }

    /// Download the given workshop item.
    #[must_use]
    pub fn workshop_download_item(mut self, app_id: u32, item_id: u32) -> Self {
        self.workshop_download_item.push((app_id, item_id));
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

        for (app_id, validate) in self.app_update {
            command.arg("+app_update").arg(app_id.to_string());

            if validate {
                command.arg("validate");
            }
        }

        for (app_id, item_id) in self.workshop_download_item {
            command
                .arg("+workshop_download_item")
                .arg(app_id.to_string())
                .arg(item_id.to_string());
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
