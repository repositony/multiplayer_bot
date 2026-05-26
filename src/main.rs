mod commands;
mod events;
mod games;
mod tokens;

// discord API
use serenity::Client;
use serenity::all::GatewayIntents;

#[tokio::main]
async fn main() {
    // build the client
    let mut client = Client::builder(tokens::BOT_TOKEN.as_str(), GatewayIntents::empty())
        .event_handler(events::Handler)
        .await
        .expect("Error creating client");

    // start a single "shard" to listen to events
    //  - automatically try to reconnect with exponential backoff
    if let Err(error) = client.start().await {
        println!("Client error: {error:?}");
    }
}
