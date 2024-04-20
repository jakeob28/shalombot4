use std::collections::HashMap;
use serenity::all::{CommandInteraction, CreateChannel, CreateCommand, CreateEmbed, CreateMessage};

use serenity::client::Context;
use serenity::model::channel::{ChannelType, GuildChannel, PermissionOverwrite, PermissionOverwriteType};
use serenity::model::id::{ChannelId, GuildId, RoleId};
use serenity::model::Permissions;
use serenity::prelude::Mentionable;

use crate::botconfig::BotConfig;

pub async fn run(ctx: &Context, command: &CommandInteraction) -> String {
    let guild_id = GuildId::new(BotConfig::global_cfg().guild_settings.guild_id);

    let mut ticket_number = 1;
    let channels = match guild_id.channels(&ctx.http).await {
        Ok(channels) => { channels }
        Err(_) => { return "Error getting channels".to_string(); }
    };

    while channel_exists(&channels, ticket_number) {
        ticket_number += 1;
    }


    let ticket_channel = match guild_id.create_channel(
        &ctx.http, CreateChannel::new(format!("ticket-{}", ticket_number))
            .kind(ChannelType::Text)
            .category(ChannelId::new(BotConfig::global_cfg().guild_settings.ticket_category))
            .permissions(vec![
                PermissionOverwrite {
                    allow: Permissions::empty(),
                    deny: Permissions::VIEW_CHANNEL | Permissions::SEND_MESSAGES,
                    kind: PermissionOverwriteType::Role(RoleId::new(BotConfig::global_cfg().guild_settings.everyone_role)),
                },
                PermissionOverwrite {
                    allow: Permissions::VIEW_CHANNEL | Permissions::SEND_MESSAGES,
                    deny: Permissions::empty(),
                    kind: PermissionOverwriteType::Role(RoleId::new(BotConfig::global_cfg().guild_settings.support_team_role)),
                },
                PermissionOverwrite {
                    allow: Permissions::VIEW_CHANNEL | Permissions::SEND_MESSAGES,
                    deny: Permissions::empty(),
                    kind: PermissionOverwriteType::Member(command.user.id),
                }]),
    ).await {
        Ok(channel) => { channel }
        Err(_) => { return "Error Creating Channel".to_string(); }
    };

    let _message = ticket_channel.send_message(
        &ctx.http, CreateMessage::new().embed(CreateEmbed::new().title("New Ticket")
            .description("Before asking for support on your newly created ticket, please read our simple terms of service.
•    First of all, keep in mind that tickets may not be private, and may be used for Shalom Support Team ™ training, and also examples for our members.
•    Second, you should be careful about holding information back. If you cannot provide a full conversation, we cannot provide the best answer for you.
•    Please note that all tickets may cost you up to several thousands of dollars.
•    Don’t forget that the Shalom Support Team ™ is completely serious and all of our answers take careful critiquing, and contemplation.
•    Releasing information about the Shalom Support Team ™ and fellow members of the Shalom Support Group ™, is punishable by full force explosions.
•    The Shalom Support Team ™ has full control over your ability to create tickets. Our team takes the upmost importance to fulfill your tickets accurately, so please do not fool around with our ticketing service. If you do, we will ban your ticket making ability.
•    Keep in mind the Shalom Support Team ™ may be offline and not able to answer your ticket at any possible time.
•    If your ticket is taking time to be processed please be patient. If you have waited over 24 hours then you my contact a Shalom Supporter ™ outside of your ticket.
•    All tickets are logged in the event that there is a complaint against a member of support staff or a member. If you would like a copy of your ticket's log, let us know.
")
            .thumbnail("https://cdn2.iconfinder.com/data/icons/flaturici-set-4/512/ticket-512.png")
            .field("Ticket Author", command.user.mention().to_string(), true)
            .field("Ticket ID", &ticket_channel.name, true)
        ),
    ).await;

    format!("Ticket created at {}", ticket_channel.mention().to_string())
}

fn channel_exists(channels: &HashMap<ChannelId, GuildChannel>, ticket_number: i32) -> bool {
    for channel_entry in channels {
        if channel_entry.1.name == format!("ticket-{}", ticket_number) {
            return true;
        }
    }

    false
}

pub fn register() -> CreateCommand {
    CreateCommand::new("ticket")
        .description("Create a Shalom Support ticket")
}