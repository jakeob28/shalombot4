use log::{error, info};
use std::env;
use std::sync::atomic::AtomicBool;

use serenity::prelude::*;

use events::Handler;

mod crossword;
mod events;

#[tokio::main]
async fn main() {
    env_logger::init();
    info!("ShalomBot4 Started");

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    info!("Creating client...");
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler {
            is_watch_running: AtomicBool::new(false),
        })
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}
