use crate::games::{self, GAME_SERVERS};

use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::model::application::{CommandOptionType, ResolvedOption, ResolvedValue};

pub fn register() -> CreateCommand {
    let mut options =
        CreateCommandOption::new(CommandOptionType::String, "game", "Name of the game server")
            .required(false);

    for game in GAME_SERVERS.iter() {
        options = options.add_string_choice(game.name(), game.name())
    }

    CreateCommand::new("help")
        .description("Help information")
        .add_option(options)
}

pub fn run(options: &[ResolvedOption]) -> String {
    if let Some(ResolvedOption {
        value: ResolvedValue::String(game),
        ..
    }) = options.first()
    {
        // if the game name exists and is valid, print the specifics
        if let Some(server_config) = games::get_game_server(game) {
            return server_config.help_message();
        }
    };

    // otherwise dump a generic help message
    general_help()
}

fn general_help() -> String {
    let server_list: String = GAME_SERVERS
        .iter()
        .map(|server| format!("- `{}` - {}\n", server.name(), server.description()))
        .collect();

    format!(
        "# Welcome to the Brumders multiplayer bot\n\
        This bot allows you to interact with our game servers\n\
        ## Slash Commands\n\
        General commands:\n\
        ```\n\
        /help : Show this general help info\n\
        /ip   : Latest public IP\n\
        /list : List of available servers and their status\n\n\
        ```\n\
        Game-specific commands:\n\
        ```\n\
        /help    <game> : How to set up your game\n\
        /start   <game> : Start the server\n\
        /stop    <game> : Stop the server\n\
        /restart <game> : Restart the server\n\
        /update  <game> : Update the server if possible\n\
        ```\n\
        All `/` commands have autocomplete suggestions for game selection\n\
        ## Available game servers\n\
        {server_list}\n"
    )
}
