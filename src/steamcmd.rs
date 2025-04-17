use std::env;
use std::path::PathBuf;
use std::process::Command;

const STEAM_CMD: &str = "steamcmd";

/// Builder to construct a steamcmd invocation, returning a [`Command`].
#[derive(Debug, Default)]
pub struct SteamCmd {
    force_install_dir: Option<PathBuf>,
    user: Option<String>,
    app_update: Vec<u32>,
    workshop_download_item: Vec<(u32, u32)>,
    validate: bool,
}

impl SteamCmd {
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
    pub fn app_update(mut self, id: u32) -> Self {
        self.app_update.push(id);
        self
    }

    /// Validate the installed app(s).
    #[must_use]
    pub const fn validate(mut self) -> Self {
        self.validate = true;
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
        let mut command = Command::new(env::var_os("STEAMCMD").unwrap_or_else(|| STEAM_CMD.into()));

        if let Some(dir) = self.force_install_dir {
            command.arg("+force_install_dir").arg(dir);
        }

        if let Some(user) = self.user {
            command.arg("+login").arg(user);
        }

        for app_id in self.app_update {
            command.arg("+app_update").arg(app_id.to_string());
        }

        if self.validate {
            command.arg("+validate");
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
