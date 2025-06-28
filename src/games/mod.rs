mod enshrouded;
mod satisfactory;
mod sevendays;
mod skyvaults;
mod vaulthunters;
mod zomboid;

use std::sync::LazyLock;

/// Array of all game servers to add to the manager
pub static GAME_SERVERS: LazyLock<[Box<dyn GameServer>; 6]> = LazyLock::new(|| {
    [
        Box::new(sevendays::Server::new()),
        Box::new(enshrouded::Server::new()),
        Box::new(satisfactory::Server::new()),
        Box::new(skyvaults::Server::new()),
        Box::new(vaulthunters::Server::new()),
        Box::new(zomboid::Server::new()),
    ]
});

pub trait GameServer: Send + Sync {
    fn new() -> Self
    where
        Self: Sized;
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn port(&self) -> u16;
    fn help_message(&self) -> String;
    fn start(&self) -> String;
    fn stop(&self) -> String;
    fn restart(&self) -> String;
    fn update(&self) -> String;
    fn status(&self) -> String;
}

/// Get the server's public IP
///
/// WARNING! do not do this with anyone untrusted in the discord guild
pub fn public_ip() -> String {
    if let Ok(response) = minreq::get("https://ident.me").send() {
        if let Ok(ip) = response.as_str() {
            return ip.to_string();
        }
    };

    "unknown".to_string()
}

/// Get the game server data
pub fn get_game_server(name: &str) -> Option<&dyn GameServer> {
    GAME_SERVERS.iter().find(|g| g.name() == name).map(|v| &**v)
}
