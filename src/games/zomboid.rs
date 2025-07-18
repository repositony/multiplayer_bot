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
            user: "gs_zomboid",
            runner: "pzserver",
        }
    }

    fn name(&self) -> &str {
        "zomboid"
    }

    fn description(&self) -> &str {
        "Project Zomboid server"
    }

    fn port(&self) -> u16 {
        16261
    }

    fn help_message(&self) -> String {
        format!(
            "## Help for Project Zomboid\n\
            ### Installation\n\
            - Install Project Zomboid\n\
            - Click play\n\
            ### Connection\n\
            Either:\n\
            - Search for \"Brumders\" in the server browser\n\
            - Connect to {0}:{1} directly\n\n\
            :lock: The server password is \"aids\"\n",
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

    // todo check the status options
    fn update(&self) -> String {
        let status = std::process::Command::new("su")
            .arg("-")
            .arg(self.user)
            .arg("-c")
            .arg(format!("/home/{}/{} update", self.user, self.runner))
            .status()
            .expect("failed to execute process");

        if status.success() {
            format!("✅ The {} server updated successfully", self.name())
        } else {
            format!("❌ The {} server failed to update, ask Tony)", self.name())
        }
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
