use log::{error, info};
use serenity::all::{CreateAttachment, CreateInteractionResponse, CreateMessage, Interaction};
use serenity::async_trait;
use serenity::builder::CreateInteractionResponseMessage;
use serenity::client::{Context, EventHandler};
use serenity::futures::StreamExt;
use serenity::model::channel::Reaction;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::botconfig::BotConfig;
use serenity::model::gateway::Ready;
use serenity::model::id::{ChannelId, GuildId, RoleId};

use crate::{commands, crossword};

pub struct Handler {
    pub(crate) is_watch_running: AtomicBool,
}

#[async_trait]
impl EventHandler for Handler {
    async fn reaction_add(&self, ctx: Context, add_reaction: Reaction) {
        if add_reaction
            .user_id
            .is_some_and(|id| id.eq(&ctx.cache.current_user().id))
        {
            return;
        }

        if !add_reaction.emoji.unicode_eq("❌") {
            return;
        }

        if !add_reaction.member.as_ref().is_some_and(|member| {
            member.roles.contains(&RoleId::new(
                BotConfig::global_cfg().guild_settings.support_team_role,
            ))
        }) {
            return;
        }

        if add_reaction.message(&ctx.http).await.is_ok_and(|msg| {
            msg.content
                .starts_with("Thank you for your ticket with Shalom Support")
                && msg.author.id.eq(&ctx.cache.current_user().id)
        }) {
            let mut message_vec = Vec::new();
            let mut messages = add_reaction.channel_id.messages_iter(&ctx.http).boxed();
            while let Some(message_result) = messages.next().await {
                match message_result {
                    Ok(message) => message_vec.insert(
                        0,
                        format!(
                            "[{}] {}: {}",
                            message.timestamp.to_string(),
                            message.author.name,
                            message.content
                        ),
                    ),
                    Err(_) => {}
                }
            }

            ChannelId::new(BotConfig::global_cfg().guild_settings.ticket_log_channel)
                .send_message(
                    &ctx.http,
                    CreateMessage::new()
                        .content("Ticket Logged")
                        .add_file(CreateAttachment::bytes(message_vec.join("\n"), "log.txt")),
                )
                .await
                .unwrap();

            match add_reaction.channel_id.delete(&ctx.http).await {
                Ok(_) => {
                    info!("Ticket Channel Deleted")
                }
                Err(error) => {
                    error!("Error Deleting Ticket Channel: {}", error)
                }
            }
        };
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        if !self.is_watch_running.load(Ordering::Relaxed) {
            info!("{} is connected!", ready.user.name);
            crossword::start_crossword_watch(Context::clone(&ctx)).await;
            self.is_watch_running.store(true, Relaxed);
        }

        let guild_id = GuildId::new(BotConfig::global_cfg().guild_settings.guild_id);

        let commands = GuildId::set_commands(
            guild_id,
            &ctx.http,
            vec![
                commands::digits::register(),
                commands::ticket::register(),
                commands::close::register(),
            ],
        )
        .await;

        println!(
            "I now have the following guild slash commands: {:#?}",
            commands
        );
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            println!("Received command interaction: {:#?}", command);

            let content = match command.data.name.as_str() {
                "digits" => commands::digits::run(&command.data.options),
                "ticket" => commands::ticket::run(&ctx, &command).await,
                "close" => commands::close::run(&ctx, &command).await,
                _ => "not implemented :(".to_string(),
            };

            if let Err(why) = command
                .create_response(
                    &ctx.http,
                    CreateInteractionResponse::Message(
                        CreateInteractionResponseMessage::new().content(content),
                    ),
                )
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }
}
