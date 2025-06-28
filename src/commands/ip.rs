use crate::games;

use serenity::builder::CreateCommand;

pub fn register() -> CreateCommand {
    CreateCommand::new("ip").description("Latest public IP")
}

pub fn run() -> String {
    format!("The current server ip is {}", games::public_ip())
}
