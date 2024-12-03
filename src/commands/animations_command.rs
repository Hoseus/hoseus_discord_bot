use serenity::all::{
    CommandDataOption, CommandInteraction, CommandOptionType, Context, CreateCommand,
    CreateCommandOption, CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage,
};

use crate::animation;
use crate::commands::serenity_command_helper;

pub const COMMAND_NAME: &str = "animations";

pub fn register() -> CreateCommand {
    CreateCommand::new(COMMAND_NAME)
        .description("Notify everyone via text channel")
        .add_option(CreateCommandOption::new(
            CommandOptionType::SubCommand,
            "list",
            "List all animation urls",
        ))
}

pub async fn run(ctx: &Context, command: &CommandInteraction) -> Result<(), ()> {
    let options: &[CommandDataOption] = command.data.options.as_slice();

    let sub_command_list: &CommandDataOption = options.get(0).unwrap();

    match sub_command_list.kind() {
        CommandOptionType::SubCommand => (),
        _ => return respond_fail_interaction(ctx, command, "Invalid command").await,
    }

    respond_success_interaction(ctx, command, animation::get_animation_urls()).await
}

async fn respond_success_interaction(
    ctx: &Context,
    command: &CommandInteraction,
    animation_urls: Vec<String>,
) -> Result<(), ()> {
    serenity_command_helper::respond_interaction(ctx, command, || {
        CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new().add_embeds(
                animation_urls
                    .iter()
                    .enumerate()
                    .map(|(index, animation_url)| {
                        CreateEmbed::new().title(index.to_string()).image(animation_url)
                    })
                    .collect(),
            ),
        )
    })
    .await
}

async fn respond_fail_interaction(
    ctx: &Context,
    command: &CommandInteraction,
    error_message: &str,
) -> Result<(), ()> {
    serenity_command_helper::respond_interaction_with_string(ctx, command, error_message).await
}
