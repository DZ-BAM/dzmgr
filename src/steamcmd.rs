use std::env;
use std::ffi::OsString;
use std::path::PathBuf;
use std::process::Command;

const STEAM_CMD: &str = "steamcmd";

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SteamCmd {
    force_install_dir: Option<PathBuf>,
    user: Option<String>,
}

impl SteamCmd {
    #[must_use]
    pub const fn new(force_install_dir: Option<PathBuf>, user: Option<String>) -> Self {
        Self {
            force_install_dir,
            user,
        }
    }
    #[must_use]
    pub fn app_update(&self, id: u32) -> Command {
        let mut cmd = self.cmd();
        cmd.arg("+app_update").arg(id.to_string()).arg("validate");
        cmd
    }

    #[must_use]
    pub fn workshop_download_item(&self, id: u32, workshop_item: u32) -> Command {
        let mut cmd = self.cmd();
        cmd.arg("+workshop_download_item")
            .arg(id.to_string())
            .arg(workshop_item.to_string());
        cmd
    }

    fn cmd(&self) -> Command {
        let mut cmd = Command::new(steamcmd());

        if let Some(force_install_dir) = &self.force_install_dir {
            cmd.arg("+force_install_dir").arg(force_install_dir);
        }

        if let Some(user) = &self.user {
            cmd.arg("+login").arg(user);
        }

        cmd
    }
}

fn steamcmd() -> OsString {
    env::var_os("STEAMCMD").unwrap_or_else(|| STEAM_CMD.into())
}
