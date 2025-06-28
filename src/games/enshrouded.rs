use crate::games::{self, GameServer};

pub struct Server<'a> {
    /// User that runs the game server
    user: &'a str,
    /// Server executable that handles the game
    runner: &'a str,
    /// tmux session name
    session: &'a str,
}

impl Server<'_> {
    fn tmux_session_exists(&self) -> bool {
        std::process::Command::new("sudo")
            .arg("-u")
            .arg(self.user)
            .arg("tmux")
            .arg("has-session")
            .arg("-t")
            .arg(self.session)
            .output()
            .expect("Failed to run su command")
            .status
            .success()
    }
}

impl GameServer for Server<'_> {
    fn new() -> Self {
        Server {
            user: "gs_enshrouded",
            runner: "/usr/bin/wine /home/gs_enshrouded/serverfiles/enshrouded_server.exe",
            session: "enshrouded_server",
        }
    }

    fn name(&self) -> &str {
        "enshrouded"
    }

    fn description(&self) -> &str {
        "Enshrouded server"
    }

    fn port(&self) -> u16 {
        15637
    }

    fn help_message(&self) -> String {
        format!(
            "## Help for Enshrouded\n\
            ### Installation\n\
            - Install Enshrouded\n\
            - Click play\n\
            ### Connection\n\
            In-game:\n\
            - Search for \"Brumders\" in the server browser\n\
            - Connect to {0}:{1} directly\n\n\
            Via steam:\n\
            - View>Game Servers>Favorites>Add {0}:{1}\n\n\
            :lock: The server password is \"aids\"\n\
            ### Server settings\n\
            - Default difficulty\n\
            - 45/15 minute day/night cycle\n",
            games::public_ip(),
            self.port()
        )
    }

    fn start(&self) -> String {
        if self.tmux_session_exists() {
            return format!("✅ The {} server is already running", self.name());
        }

        let status = std::process::Command::new("sudo")
            .arg("-u")
            .arg(self.user)
            .arg("tmux")
            .arg("new-session")
            .arg("-d")
            .arg("-s")
            .arg(self.session)
            .arg(self.runner)
            .status()
            .expect("failed to execute tmux");

        if status.success() {
            format!("✅ The {} server started successfully", self.name())
        } else {
            format!(
                "❌ The {} server failed to start, ask Tony to fix it",
                self.name()
            )
        }
    }

    fn stop(&self) -> String {
        if !self.tmux_session_exists() {
            return format!("✅ The {} server is already stopped", self.name());
        }

        let status = std::process::Command::new("sudo")
            .arg("-u")
            .arg(self.user)
            .arg("tmux")
            .arg("kill-session")
            .arg("-t")
            .arg(self.session)
            .status()
            .expect("failed to execute tmux");

        if status.success() {
            format!("✅ The {} server stopped successfully", self.name())
        } else {
            format!(
                "❌ The {} server failed to stop, ask Tony to fix it",
                self.name()
            )
        }
    }

    fn restart(&self) -> String {
        // kill the session if it exists
        let _ = std::process::Command::new("sudo")
            .arg("-u")
            .arg(self.user)
            .arg("tmux")
            .arg("kill-session")
            .arg("-t")
            .arg(self.session)
            .status();

        // now start as usual
        let status = std::process::Command::new("sudo")
            .arg("-u")
            .arg(self.user)
            .arg("tmux")
            .arg("new-session")
            .arg("-d")
            .arg("-s")
            .arg(self.session)
            .arg(self.runner)
            .status()
            .expect("failed to execute tmux");

        if status.success() {
            format!("✅ The {} server restarted successfully", self.name())
        } else {
            format!(
                "❌ The {} server failed to restart, ask Tony to fix it",
                self.name()
            )
        }
    }

    // todo check the status options
    fn update(&self) -> String {
        let output = std::process::Command::new("sudo")
            .args([
                "-u",
                self.user,
                "/usr/games/steamcmd",
                "+@sSteamCmdForcePlatformType",
                "windows",
                "+force_install_dir",
                &format!("/home/{}/serverfiles", self.user),
                "+login",
                "anonymous",
                "+app_update",
                "2278520",
                "+quit",
            ])
            .output()
            .expect("unable to execute command");

        let stdout = String::from_utf8_lossy(&output.stdout);
        if stdout.contains("Success!") {
            format!("✅ The {} server updated successfully", self.name())
        } else {
            format!("❌ The {} server failed to update, ask Tony)", self.name())
        }
    }

    fn status(&self) -> String {
        if self.tmux_session_exists() {
            ":arrow_forward: Running".to_string()
        } else {
            ":stop_button: Idle".to_string()
        }
    }
}
