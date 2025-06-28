use crate::games::GAME_SERVERS;

use serenity::builder::CreateCommand;

pub fn register() -> CreateCommand {
    CreateCommand::new("list").description("List current status of all servers")
}

pub fn run() -> String {
    let status_list: String = GAME_SERVERS
        .iter()
        .map(|server| {
            format!(
                "- `{}` - {}\n  - {}\n",
                server.name(),
                server.status(),
                server.description()
            )
        })
        .collect();

    format!(
        "## List of brumders servers\n\
        {status_list}\n"
    )
}
