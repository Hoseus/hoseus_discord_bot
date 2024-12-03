use serenity::all::{CommandInteraction, Context, CreateCommand};

use crate::commands::serenity_command_helper;
use crate::config;

pub const COMMAND_NAME: &str = "invite";

pub fn register() -> CreateCommand {
    CreateCommand::new(COMMAND_NAME).description("Display an invite link to a telegram group")
}

pub async fn run(ctx: &Context, command: &CommandInteraction) -> Result<(), ()> {
    respond_success_interaction(
        ctx,
        command,
        format!(
            "Use this link to join our telegram!\n{}",
            config::telegram_invite_link()
        )
        .as_str(),
    )
    .await
}

async fn respond_success_interaction(
    ctx: &Context,
    command: &CommandInteraction,
    success_message: &str,
) -> Result<(), ()> {
    serenity_command_helper::respond_interaction_with_string(ctx, command, success_message).await
}
