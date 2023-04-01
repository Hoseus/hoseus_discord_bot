use serenity::builder::CreateApplicationCommand;
use serenity::client::Context;
use serenity::model::application::command::CommandOptionType;
use serenity::model::application::interaction::application_command::{
    ApplicationCommandInteraction, CommandDataOption, CommandDataOptionValue,
};

use crate::{animation, message_helper, serenity_model_helper, telegram};
use crate::commands::serenity_command_helper;

pub const COMMAND_NAME: &str = "notify";

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name(COMMAND_NAME)
        .description("Notify everyone via text channel")
        .create_option(|option| {
            option
                .name("index")
                .description("Index of the chosen media to send")
                .kind(CommandOptionType::Integer)
                .min_int_value(0)
                .required(false)
        })
        .create_option(|option| {
            option
                .name("message")
                .description("Custom message to send")
                .kind(CommandOptionType::String)
                .required(false)
        })
}

pub async fn run(ctx: Context, command: ApplicationCommandInteraction) -> Result<(), ()> {
    let options: &[CommandDataOption] = &command.data.options;

    let option_animation_index: Option<CommandDataOptionValue> = options
        .get(0)
        .map(|option| option.to_owned().resolved)
        .flatten();

    let option_message: Option<CommandDataOptionValue> = options
        .get(1)
        .map(|option| option.to_owned().resolved)
        .flatten();

    let animation_url: String =
        if let Some(CommandDataOptionValue::Integer(inputted_animation_index)) =
            option_animation_index
        {
            let index = usize::try_from(inputted_animation_index).unwrap();
            if let Some(some_animation_url) = animation::get_animation_url(index.to_owned()) {
                some_animation_url.to_string()
            } else {
                let animation_urls_size = animation::get_animation_urls_size();
                return respond_fail_interaction(
                    ctx.to_owned(),
                    command.to_owned(),
                    format!(
                        "Index {} does not exist. Use a value between {} and {}",
                        index,
                        0,
                        animation_urls_size - 1
                    ),
                )
                .await;
            }
        } else {
            animation::get_random_animation_url()
        };

    let message: String = if let Some(CommandDataOptionValue::String(inputted_message)) =
        option_message
    {
        inputted_message.to_string()
    } else {
        let user_name: String = command.user.name.to_string();
        let channel_name: String =
            serenity_model_helper::get_channel_name_from_application_command(
                ctx.to_owned(),
                command.to_owned(),
            )
            .await;
        let guild_name: String = serenity_model_helper::get_guild_name_from_application_command(
            ctx.to_owned(),
            command.to_owned(),
        )
        .await;

        message_helper::build_text_channel_notification_message(user_name, channel_name, guild_name)
    };

    telegram::send_notification_to_telegram(animation_url, message);

    respond_success_interaction(ctx.to_owned(), command.to_owned(), "Success!".to_string()).await
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

async fn respond_fail_interaction(
    ctx: Context,
    command: ApplicationCommandInteraction,
    error_message: String,
) -> Result<(), ()> {
    serenity_command_helper::respond_interaction_with_string(
        ctx,
        command.to_owned(),
        error_message.to_owned(),
    )
    .await
}
