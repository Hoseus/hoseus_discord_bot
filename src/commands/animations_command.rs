use serenity::builder::{CreateApplicationCommand, CreateEmbed};
use serenity::client::Context;
use serenity::model::application::command::CommandOptionType;
use serenity::model::application::interaction::application_command::{
    ApplicationCommandInteraction, CommandDataOption,
};
use serenity::model::application::interaction::InteractionResponseType;

use crate::animation;
use crate::commands::serenity_command_helper;

pub const COMMAND_NAME: &str = "animations";

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name(COMMAND_NAME)
        .description("Notify everyone via text channel")
        .create_option(|option| {
            option
                .name("list")
                .description("List all animation urls")
                .kind(CommandOptionType::SubCommand)
        })
}

pub async fn run(ctx: Context, command: ApplicationCommandInteraction) -> Result<(), ()> {
    let options: &[CommandDataOption] = &command.data.options;

    let sub_command_list: CommandDataOption = options.get(0).unwrap().to_owned();

    match sub_command_list.kind {
        CommandOptionType::SubCommand => (),
        _ => {
            return respond_fail_interaction(ctx, command.to_owned(), "Invalid command".to_string())
                .await
        }
    }

    respond_success_interaction(ctx, command.to_owned(), animation::get_animation_urls()).await
}

async fn respond_success_interaction(
    ctx: Context,
    command: ApplicationCommandInteraction,
    animation_urls: Vec<String>,
) -> Result<(), ()> {
    serenity_command_helper::respond_interaction(ctx, command.to_owned(), |response| {
        response
            .kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|message| {
                message.add_embeds(
                    animation_urls
                        .iter()
                        .enumerate()
                        .map(|(index, animation_url)| {
                            CreateEmbed::default()
                                .title(index.to_string())
                                .image(animation_url.to_owned())
                                .to_owned()
                        })
                        .collect(),
                )
            })
    })
    .await
}

async fn respond_fail_interaction(
    ctx: Context,
    command: ApplicationCommandInteraction,
    error_message: String,
) -> Result<(), ()> {
    serenity_command_helper::respond_interaction_with_string(ctx, command.to_owned(), error_message)
        .await
}
