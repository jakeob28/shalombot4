use log::info;
use serenity::async_trait;
use serenity::client::{Context, EventHandler};
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use std::sync::atomic::Ordering::Relaxed;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::botconfig::BotConfig;
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;

use crate::{commands, crossword};

pub struct Handler {
    pub(crate) is_watch_running: AtomicBool,
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        if !self.is_watch_running.load(Ordering::Relaxed) {
            info!("{} is connected!", ready.user.name);
            crossword::start_crossword_watch(Context::clone(&ctx)).await;
            self.is_watch_running.store(true, Relaxed);
        }

        let guild_id = GuildId(BotConfig::global_cfg().guild_settings.guild_id);

        let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands.create_application_command(|command| commands::digits::register(command))
        })
        .await;

        println!(
            "I now have the following guild slash commands: {:#?}",
            commands
        );
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("Received command interaction: {:#?}", command);

            let content = match command.data.name.as_str() {
                "digits" => commands::digits::run(&command.data.options),
                _ => "not implemented :(".to_string(),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }
}
