// internal
use crate::{commands, tokens};

// discord API
use serenity::all::CommandInteraction;
use serenity::async_trait;
use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::model::application::Interaction;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        println!("Registering the guild slash commands");
        let commands = &tokens::GUILD_ID
            .set_commands(
                &ctx.http,
                vec![
                    commands::help::register(),
                    commands::ip::register(),
                    commands::list::register(),
                    commands::restart::register(),
                    commands::start::register(),
                    commands::stop::register(),
                    commands::update::register(),
                ],
            )
            .await;

        println!("Registered guild slash commands: {commands:#?}");
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            // stop random members from messing with running servers
            if !is_trusted_member(&command) {
                let msg = CreateInteractionResponseMessage::new()
                    .content("This bot is restricted to trusted memebers, ask someone to add you to the `Server Vet` role");
                let builder = CreateInteractionResponse::Message(msg);
                let _ = command.create_response(&ctx.http, builder).await;
                return;
            }

            // slash commands
            match command.data.name.as_str() {
                "help" => {
                    let message = commands::help::run(&command.data.options());
                    respond_immediately(&ctx, &command, message).await;
                }
                "ip" => {
                    let message = commands::ip::run();
                    respond_immediately(&ctx, &command, message).await;
                }
                "list" => {
                    respond_deferred(
                        &ctx,
                        &command,
                        "Listing status of all game servers...",
                        commands::list::run,
                    )
                    .await;
                }
                "start" => {
                    let cmd_data = command.data.clone();
                    respond_deferred(&ctx, &command, "Starting game server...", move || {
                        commands::start::run(&cmd_data.options())
                    })
                    .await;
                }
                "stop" => {
                    let cmd_data = command.data.clone();
                    respond_deferred(
                        &ctx,
                        &command,
                        "Stopping game server (may take a few minutes)...",
                        move || commands::stop::run(&cmd_data.options()),
                    )
                    .await;
                }
                "restart" => {
                    let cmd_data = command.data.clone();
                    respond_deferred(&ctx, &command, "Restarting game server...", move || {
                        commands::restart::run(&cmd_data.options())
                    })
                    .await;
                }
                "update" => {
                    let cmd_data = command.data.clone();
                    respond_deferred(
                        &ctx,
                        &command,
                        "Updating game server (may take a several minutes)...",
                        move || commands::update::run(&cmd_data.options()),
                    )
                    .await;
                }
                unrecognised => {
                    respond_immediately(
                        &ctx,
                        &command,
                        format!("\"{unrecognised}\" is not a recognised command"),
                    )
                    .await;
                }
            }
        }
    }
}

/// Check to see if the user belongs to the trusted role
fn is_trusted_member(command: &CommandInteraction) -> bool {
    if let Some(member) = command.member.as_ref() {
        member.roles.contains(&tokens::TRUSTED_ROLE_ID)
    } else {
        false
    }
}

/// For anything very likely to take <3 seconds
async fn respond_immediately(ctx: &Context, command: &CommandInteraction, content: String) {
    let data = CreateInteractionResponseMessage::new().content(content);
    let builder = CreateInteractionResponse::Message(data);
    let _ = command.create_response(&ctx.http, builder).await;
}

/// For anything that might take >3 seconds, where discord will otherwise assume
/// failure and not wait for the response
async fn respond_deferred<F>(
    ctx: &Context,
    command: &CommandInteraction,
    initial_message: &str,
    run_function: F,
) where
    F: FnOnce() -> String + Send + 'static,
{
    if let Err(e) = command.defer(&ctx.http).await {
        eprintln!("Failed to defer: {e}");
        return;
    }

    let progress = serenity::builder::EditInteractionResponse::new().content(initial_message);
    let _ = command.edit_response(&ctx.http, progress).await;

    let result = tokio::task::spawn_blocking(run_function).await.unwrap();

    let final_msg = serenity::builder::EditInteractionResponse::new().content(result);
    let _ = command.edit_response(&ctx.http, final_msg).await;
}
