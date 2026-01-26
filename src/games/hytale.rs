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
        let version = match std::process::Command::new("su")
            .arg("-")
            .arg(self.user)
            .arg("-c")
            .arg(format!("./hytale_downloader -print-version"))
            .output()
        {
            Ok(o) if o.status.success() => String::from_utf8_lossy(&o.stdout).trim().to_string(),
            Ok(o) => {
                println!("version: {o:?}");
                "failed".to_string()
            }
            _ => "Unknown".to_string(),
        };

        format!(
            "## Help for Hytale\n\
            Hytale server, currently vanilla \n\
            (version: {2})\n\
            ### Installation\n\
            - Install hytale from [here](<https://www.curseforge.com/download/app>)\n\
            - Follow purchase/setup instructions\n\
            ### Connection\n\
            - Server > Add Server\n\
              - Connection address: {0}:{1}\n\
              - Name: Brumders\n\
            - Connect to the server\n\
            - Password prompt: `aids`\n\n",
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
                format!("The {} server started successfully", self.name())
            }
            // systemctl returns 0 even if already active, but keep this for safety
            Ok(s) if matches!(s.code(), Some(2)) => {
                format!("The {} server is already running", self.name())
            }
            _ => {
                format!(
                    "The {} server failed to start, ask Tony to fix it",
                    self.name()
                )
            }
        }
    }

    fn stop(&self) -> String {
        let status = std::process::Command::new("systemctl")
            .args(["stop", self.runner])
            .status();

        match status {
            Ok(s) if s.success() => {
                format!("The {} server stopped successfully", self.name())
            }
            // Same note as above
            Ok(s) if matches!(s.code(), Some(2)) => {
                format!("The {} server is already stopped", self.name())
            }
            _ => {
                format!("The {} server failed to stop, ask Tony", self.name())
            }
        }
    }

    fn restart(&self) -> String {
        let status = std::process::Command::new("systemctl")
            .args(["restart", self.runner])
            .status()
            .expect("failed to execute systemctl");

        if status.success() {
            format!("The {} server restarted successfully", self.name())
        } else {
            format!("The {} server failed to restart, ask Tony", self.name())
        }
    }

    fn update(&self) -> String {
        format!(
            "The {} server must be updated manually, ask Tony",
            self.name()
        )
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
