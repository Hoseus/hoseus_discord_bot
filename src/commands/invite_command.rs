use serenity::builder::CreateApplicationCommand;
use serenity::client::Context;
use serenity::model::application::interaction::application_command::{
    ApplicationCommandInteraction
};

use crate::config;
use crate::commands::serenity_command_helper;

pub const COMMAND_NAME: &str = "invite";

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name(COMMAND_NAME)
        .description("Display an invite link to a telegram group")
}

pub async fn run(ctx: Context, command: ApplicationCommandInteraction) -> Result<(), ()> {
    respond_success_interaction(ctx.to_owned(), command.to_owned(), format!("Use this link to join our telegram!\n{}", config::telegram_invite_link())).await
}

async fn respond_success_interaction(
    ctx: Context,
    command: ApplicationCommandInteraction,
    success_message: String,
) -> Result<(), ()> {
    serenity_command_helper::respond_interaction_with_string(
        ctx,
        command.to_owned(),
        success_message.to_owned(),
    )
    .await
}
