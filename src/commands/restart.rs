use crate::games::{self, GAME_SERVERS};

use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::model::application::{CommandOptionType, ResolvedOption, ResolvedValue};

pub fn register() -> CreateCommand {
    let mut options =
        CreateCommandOption::new(CommandOptionType::String, "game", "Name of the game server")
            .required(true);

    for game in GAME_SERVERS.iter() {
        options = options.add_string_choice(game.name(), game.name())
    }

    CreateCommand::new("restart")
        .description("Restart a game server")
        .add_option(options)
}

pub fn run(options: &[ResolvedOption]) -> String {
    if let Some(ResolvedOption {
        value: ResolvedValue::String(game),
        ..
    }) = options.first()
    {
        // check if this exists and is valid
        if let Some(server_config) = games::get_game_server(game) {
            return server_config.restart();
        }
    };

    "Please provide a valid game name".to_string()
}
