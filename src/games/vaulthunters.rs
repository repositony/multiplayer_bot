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
            user: "gs_vaulthunters",
            runner: "mcserver",
        }
    }

    fn name(&self) -> &str {
        "vaulthunters"
    }

    fn description(&self) -> &str {
        "Minecraft server - Vaulthunters modpack"
    }

    fn port(&self) -> u16 {
        25565
    }

    fn help_message(&self) -> String {
        format!(
            "## Help for Vaulthunters\n\
            For Vaulthunters with skyblock world generation use `skyvaults`.\n\
            ### Installation\n\
            - Download curseforge (<https://www.curseforge.com/download/app>)\n\
            - Install the Vault Hunters v3 modpack\n\
            - Click play\n\
            ### Connection\n\
            - Connect to {0}:{1}\n\
            ### Server settings\n\
            - Vault difficulty: normal\n\
            - Vanilla difficulty: hard\n",
            games::public_ip(),
            self.port()
        )
    }

    fn start(&self) -> String {
        let status = std::process::Command::new("su")
            .arg("-")
            .arg(self.user)
            .arg("-c")
            .arg(format!("/home/{}/{} start", self.user, self.runner))
            .status()
            .expect("failed to execute process");

        if status.success() {
            format!("✅ The {} server started successfully", self.name())
        } else if status.code() == Some(2) {
            format!("✅ The {} server is already running", self.name())
        } else {
            format!(
                "❌ The {} server failed to start, ask Tony to fix it)",
                self.name()
            )
        }
    }

    fn stop(&self) -> String {
        let status = std::process::Command::new("su")
            .arg("-")
            .arg(self.user)
            .arg("-c")
            .arg(format!("/home/{}/{} stop", self.user, self.runner))
            .status()
            .expect("failed to execute process");

        if status.success() {
            format!("✅ The {} server stopped successfully", self.name())
        } else if status.code() == Some(2) {
            format!("✅ The {} server is already stopped", self.name())
        } else {
            format!("❌ The {} server failed to stop, ask Tony)", self.name())
        }
    }

    fn restart(&self) -> String {
        let status = std::process::Command::new("su")
            .arg("-")
            .arg(self.user)
            .arg("-c")
            .arg(format!("/home/{}/{} restart", self.user, self.runner))
            .status()
            .expect("failed to execute process");

        if status.success() {
            format!("✅ The {} server restarted successfully", self.name())
        } else {
            format!("❌ The {} server failed to restart, ask Tony)", self.name())
        }
    }

    fn update(&self) -> String {
        format!(
            "❌ The {} server must be updated manually, ask Tony",
            self.name()
        )
    }

    fn status(&self) -> String {
        let check = &format!(
            "[ -e /home/{}/lgsm/lock/{}-started.lock ] && echo 1 || echo 0",
            self.user, self.runner
        );

        if let Ok(status) = std::process::Command::new("su")
            .arg("-")
            .arg(self.user)
            .arg("-c")
            .arg(format!("bash -c '{}'", check))
            .output()
        {
            match String::from_utf8_lossy(&status.stdout).trim() {
                "1" => ":arrow_forward: Running".to_string(),
                "0" => ":stop_button: Idle".to_string(),
                _ => "\u{2753} Unknown".to_string(),
            }
        } else {
            "\u{2753} Unknown".to_string()
        }
    }
}
