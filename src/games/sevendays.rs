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
            user: "gs_7days",
            runner: "sdtdserver",
        }
    }

    fn name(&self) -> &str {
        "7days"
    }

    fn description(&self) -> &str {
        "Seven Days To Die server"
    }

    fn port(&self) -> u16 {
        26900
    }

    fn help_message(&self) -> String {
        format!(
            "## Help for Seven Days To Die\n\
            ### Installation\n\
            - Install the 7 Days 2 Die\n\
            - Click play\n\
            ### Connection\n\
            Either:\n\
            - Search for \"Brumders\" in the server browser\n\
            - Connect to {0}:{1} directly\n\n\
            :lock: The server password is \"aids\"\n\
            ### Server settings\n\
            - Random world generation (seed = \"aids\")\n\
            - 60 minute days: 42 day/18 night\n\
            - No death XP/food/health penalty\n\
            - Only drop backpack on death\n\
            - Enemy difficulty: Normal\n\
            - Blood moon every 14 days\n\
            - POI and loot respawn every 7 days\n\
            - Friendly fire is OFF\n\
            ### Client-side mods\n\
            These go in your `C:\\Program Files (x86)\\Steam\\steamapps\\common\\7 Days To Die\\Mods`\n\n\
            All of these are optional, you do not need them to join\n\
            - [Craft from containers](<https://www.nexusmods.com/7daystodie/mods/4970>)\n\
            - [Read book icons](<https://7daystodiemods.com/dewtas-better-read-book-icons>)\n\
            ### Server-side mods\n\
            These are automatic, you do not need to download them\n\
            - [HUDPlus](<https://7daystodiemods.com/agf-hudplus/>)\n\
            - [Bigger Backpack Mod](<https://7daystodiemods.com/bigger-backpack-mod-60-96-slot/>)\n\
            - [HP Bars](<https://7daystodiemods.com/hp-bars/>)\n",
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
