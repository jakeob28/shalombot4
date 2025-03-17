use crate::botconfig::BotConfig;
use serenity::all::{CommandInteraction, CreateCommand, CreateMessage, ReactionType, RoleId};
use serenity::client::Context;

pub async fn run(ctx: &Context, command: &CommandInteraction) -> String {
    if !command.member.as_ref().is_some_and(|m| {
        m.roles.contains(&RoleId::new(
            BotConfig::global_cfg().guild_settings.support_team_role,
        ))
    }) {
        return "This command is restricted to the Shalom Support team!".to_string();
    }

    if !command
        .channel_id
        .name(&ctx.http)
        .await
        .unwrap_or("".to_string())
        .starts_with("ticket")
    {
        return "This command can only be run in a ticket channel!".to_string();
    }

    let msg = command.channel_id.send_message(&ctx.http, CreateMessage::new()
        .content("Thank you for your ticket with Shalom Support. Please react with an ❌ to indicate you are ready to close this ticket.")).await.expect("error sending message");
    msg.react(&ctx.http, ReactionType::from('❌'))
        .await
        .expect("error reacting");

    "".to_string()
}

pub fn register() -> CreateCommand {
    CreateCommand::new("close").description("Close a shalom ticket")
}
