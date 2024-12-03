use serenity::all::CreateCommandOption;
use serenity::builder::CreateCommand;
use serenity::client::Context;
use serenity::model::application::CommandOptionType;
use serenity::model::application::{CommandDataOption, CommandDataOptionValue, CommandInteraction};

use crate::commands::serenity_command_helper;
use crate::{animation, message_helper, serenity_model_helper, telegram};

pub const COMMAND_NAME: &str = "notify";

pub fn register() -> CreateCommand {
    CreateCommand::new(COMMAND_NAME)
        .description("Notify everyone via text channel")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::Integer,
                "index",
                "Index of the chosen media to send",
            )
            .min_int_value(0)
            .required(false),
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "message",
                "Custom message to send",
            )
            .required(false),
        )
}

pub async fn run(ctx: &Context, command: &CommandInteraction) -> Result<(), ()> {
    let options: &[CommandDataOption] = command.data.options.as_slice();

    let option_animation_index: Option<&CommandDataOptionValue> =
        options.get(0).map(|option| &option.value);

    let option_message: Option<&CommandDataOptionValue> =
        options.get(1).map(|option| &option.value);

    let animation_url: &str =
        if let Some(CommandDataOptionValue::Integer(inputted_animation_index)) =
            option_animation_index
        {
            let index = usize::try_from(*inputted_animation_index).unwrap();
            if let Some(some_animation_url) = animation::get_animation_url(index) {
                &some_animation_url.to_owned()
            } else {
                let animation_urls_size = animation::get_animation_urls_size();
                return respond_fail_interaction(
                    ctx,
                    command,
                    format!(
                        "Index {} does not exist. Use a value between {} and {}",
                        index,
                        0,
                        animation_urls_size - 1
                    )
                    .as_str(),
                )
                .await;
            }
        } else {
            &animation::get_random_animation_url()
        };

    let message: &str = if let Some(CommandDataOptionValue::String(inputted_message)) =
        option_message
    {
        inputted_message.as_str()
    } else {
        let user_name: &str = command.user.name.as_str();
        let channel_name: &str =
            &serenity_model_helper::get_channel_name_from_application_command(ctx, command)
                .await;
        let guild_name: &str =
            &serenity_model_helper::get_guild_name_from_application_command(ctx, command)
                .await;

        &message_helper::build_text_channel_notification_message(user_name, channel_name, guild_name)
    };

    telegram::send_notification_to_telegram(animation_url, message);

    respond_success_interaction(ctx, command, "Success!").await
}

async fn respond_success_interaction(
    ctx: &Context,
    command: &CommandInteraction,
    success_message: &str,
) -> Result<(), ()> {
    serenity_command_helper::respond_interaction_with_string(ctx, command, success_message).await
}

async fn respond_fail_interaction(
    ctx: &Context,
    command: &CommandInteraction,
    error_message: &str,
) -> Result<(), ()> {
    serenity_command_helper::respond_interaction_with_string(ctx, command, error_message).await
}
