use std::env;
use std::ffi::OsString;
use std::path::PathBuf;
use std::process::Command;

const STEAM_CMD: &str = "steamcmd";

/// Trait to provide steamcmd functionality to [`Command`].
pub trait SteamCmd {
    /// Create a new steamcmd command.
    fn new() -> Self;
    /// Force the installation directory. This needs to be called first.
    fn force_install_dir(&mut self, dir: PathBuf) -> &mut Self;
    /// Set the desired user.
    fn user(&mut self, user: &str) -> &mut Self;
    /// Install or update an app by ID.
    fn app_update(&mut self, id: u32) -> &mut Self;
    /// Validate the app after updating or installing.
    fn validate(&mut self) -> &mut Self;
    /// Install or update a workshop item.
    fn workshop_download_item(&mut self, id: u32, workshop_item: u32) -> &mut Self;
    /// Quit steamcmd. This should be called last.
    fn quit(&mut self) -> &mut Self;
}

impl SteamCmd for Command {
    fn new() -> Self {
        Self::new(steamcmd())
    }

    fn force_install_dir(&mut self, dir: PathBuf) -> &mut Self {
        self.arg("+force_install_dir").arg(dir)
    }

    fn user(&mut self, user: &str) -> &mut Self {
        self.arg("+login").arg(user)
    }

    fn app_update(&mut self, id: u32) -> &mut Self {
        self.arg("+app_update").arg(id.to_string())
    }

    fn validate(&mut self) -> &mut Self {
        self.arg("validate")
    }

    fn workshop_download_item(&mut self, app_id: u32, item_id: u32) -> &mut Self {
        self.arg("+workshop_download_item")
            .arg(app_id.to_string())
            .arg(item_id.to_string())
    }

    fn quit(&mut self) -> &mut Self {
        self.arg("+quit")
    }
}

fn steamcmd() -> OsString {
    env::var_os("STEAMCMD").unwrap_or_else(|| STEAM_CMD.into())
}
