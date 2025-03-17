use log::{error, info};
use std::sync::atomic::AtomicBool;

use serenity::prelude::*;

use crate::botconfig::BotConfig;
use events::Handler;

mod botconfig;
mod commands;
mod crossword;
mod digits;
mod events;

#[tokio::main]
async fn main() {
    env_logger::init();
    info!("ShalomBot4 Started");

    info!("Loading config...");
    let config = BotConfig::global_cfg();

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MESSAGE_REACTIONS;

    info!("Creating client...");
    let mut client = Client::builder(&config.discord_token, intents)
        .event_handler(Handler {
            is_watch_running: AtomicBool::new(false),
        })
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}
