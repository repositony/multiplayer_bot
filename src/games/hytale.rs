use crate::games::{self, GameServer};

pub struct Server<'a> {
    /// User that runs the game server
    user: &'a str,
    /// Server executable that handles the game
    runner: &'a str,
}

impl GameServer for Server<'_> {
    fn new() -> Self {
        Server {
            user: "gs_hytale",
            runner: "hytale.service",
        }
    }

    fn name(&self) -> &str {
        "hytale"
    }

    fn description(&self) -> &str {
        "Hytale server"
    }

    fn port(&self) -> u16 {
        5520
    }

    fn help_message(&self) -> String {
        // get release version
        let version =
            read_version_from_file(format!("/home/{}/updates/server_version.dat", self.user));

        format!(
            "## Help for Hytale\n\
            Hytale server (version: {2})\n\
            ### Installation\n\
            - Install hytale from [here](<https://www.curseforge.com/download/app>)\n\
            - Follow setup instructions\n\
            ### Connection\n\
            - Server > Add Server\n\
            - Connection address: {0}:{1}\n\
            - Name: Brumders\n\
            - Password prompt: `aids`\n\
            ### Updates\n\
            The server should detect new versions automatically at 3am and update itself.\n\n\
            Updates can be forced with `/update game hytale`. This will take about 5-10 mins if an update is actually required.\n\
            ### Mods\n\
            Mods are all server-side, you don't have to do anything. Currently minimal QoL while Hytale is constantly changing.\n\n\
            Installed mods:\n\
            - [EyeSpy](<https://legacy.curseforge.com/hytale/mods/eyespy>) - tells you what block you are looking at\n\
            - [BetterMap](<https://legacy.curseforge.com/hytale/mods/bettermap>) - larger world map, shared exploration\n\
            - [Where this at?](<https://legacy.curseforge.com/hytale/mods/where-this-at>) - lazy storage management\n\
            - [AutoSort](<https://www.curseforge.com/hytale/mods/autosort>) - option to autosort chests/inventory\n\
            - [Gravestones](<https://www.curseforge.com/hytale/mods/gravestones>) - on death, items stored in gravestone\n\
            - [Recover Arrows](<https://www.curseforge.com/hytale/mods/recover-arrows>) - pick up arrows from the ground\n\
            - [Pick block](<https://www.curseforge.com/hytale/mods/pandas-adventure-pick-block>) - MMB creative block picker when item exists in your inventory\n\
            - [Better wardrobes](<https://www.curseforge.com/hytale/mods/better-wardrobes>) - wardrobes can now be used for storage\n\
            - [Underwater breathing potions](<https://www.curseforge.com/hytale/mods/hazeyones-water-breathing-potions>) - craft potions to extend oxygen timer\n\
            - [Wormhole items](<https://www.curseforge.com/hytale/mods/recall-wormhole-items>) - crafts to teleport home/to other players\n\
            - [Endgame content](<https://www.curseforge.com/hytale/mods/endgame-qol>) - added endgame bosses, dungeons, and crafts\n\
            - [Bigger ore stacks](<https://legacy.curseforge.com/hytale/mods/bigger-ore-stacks>) - increased from 25 to 100\n\n",
            games::public_ip(),
            self.port(),
            version,
        )
    }

    fn start(&self) -> String {
        let status = std::process::Command::new("systemctl")
            .args(["start", self.runner])
            .status();

        match status {
            Ok(s) if s.success() => {
                format!(
                    "{} server started ({}:{})",
                    self.name(),
                    games::public_ip(),
                    self.port(),
                )
            }
            // systemctl returns 0 even if already active, but keep this for safety
            Ok(s) if matches!(s.code(), Some(2)) => {
                format!(
                    "{} server already running ({}:{})",
                    self.name(),
                    games::public_ip(),
                    self.port(),
                )
            }
            _ => {
                format!("{} server failed to start, ask Tony to fix it", self.name())
            }
        }
    }

    fn stop(&self) -> String {
        let status = std::process::Command::new("systemctl")
            .args(["stop", self.runner])
            .status();

        match status {
            Ok(s) if s.success() => {
                format!("{} server stopped", self.name())
            }
            Ok(s) if matches!(s.code(), Some(2)) => {
                format!("{} server already stopped", self.name())
            }
            _ => {
                format!("{} server failed to stop, ask Tony", self.name())
            }
        }
    }

    fn restart(&self) -> String {
        let status = std::process::Command::new("systemctl")
            .args(["restart", self.runner])
            .status()
            .expect("failed to execute systemctl");

        if status.success() {
            format!(
                "{} server restarted ({}:{})",
                self.name(),
                games::public_ip(),
                self.port(),
            )
        } else {
            format!("{} failed to restart, ask Tony", self.name())
        }
    }

    fn update(&self) -> String {
        let installed_version =
            read_version_from_file("/home/gs_hytale/updates/server_version.dat");

        let latest_version = match std::process::Command::new("su")
            .arg("-")
            .arg(self.user)
            .arg("-c")
            .arg("/home/gs_hytale/updates/hytale_downloader -print-version")
            .output()
        {
            Ok(output) => String::from_utf8_lossy(&output.stdout).trim().to_string(),
            _ => "none".to_string(),
        };

        if installed_version == latest_version {
            return format!("{} server already latest version", self.name());
        }

        if std::process::Command::new("su")
            .arg("-")
            .arg(self.user)
            .arg("-c")
            .arg("/home/gs_hytale/updates/update_hytale_server.sh")
            .status()
            .is_ok()
        {
            format!(
                "{} server updated ({}:{})",
                self.name(),
                games::public_ip(),
                self.port(),
            )
        } else {
            format!("{} server failed to update, ask Tony", self.name())
        }
    }

    fn status(&self) -> String {
        match std::process::Command::new("systemctl")
            .args(["is-active", "--quiet", "hytale"])
            .status()
        {
            Ok(s) if s.success() => "Running",
            Ok(s) if matches!(s.code(), Some(1 | 3)) => "Idle",
            _ => "Unknown",
        }
        .to_string()
    }
}

fn read_version_from_file<P: AsRef<std::path::Path>>(path: P) -> String {
    // Read the entire file into a string
    if let Ok(contents) = std::fs::read_to_string(path) {
        contents.trim().to_string()
    } else {
        "unknown".into()
    }
}
