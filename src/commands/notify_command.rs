use crate::{
    build_text_channel_notification_message, get_animation_url, get_random_animation_url,
    send_notification_to_telegram, NOT_OBTAINED_STRING,
};
use serenity::builder::CreateApplicationCommand;
use serenity::client::Context;
use serenity::http::CacheHttp;
use serenity::model::application::command::CommandOptionType;
use serenity::model::application::interaction::application_command::{
    ApplicationCommandInteraction, CommandDataOption, CommandDataOptionValue,
};

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

pub async fn run(ctx: Context, command: ApplicationCommandInteraction) {
    let options: &[CommandDataOption] = &command.data.options;

    let option_animation_index: Option<CommandDataOptionValue> = options
        .get(1)
        .map(|option| option.to_owned().resolved)
        .flatten();

    let option_message: Option<CommandDataOptionValue> = options
        .get(1)
        .map(|option| option.to_owned().resolved)
        .flatten();

    let animation_url: String;
    if let Some(CommandDataOptionValue::Integer(inputted_animation_index)) = option_animation_index
    {
        animation_url = get_animation_url(usize::try_from(inputted_animation_index).unwrap());
    } else {
        animation_url = get_random_animation_url();
    }

    let message: String;
    if let Some(CommandDataOptionValue::String(inputted_message)) = option_message {
        message = inputted_message.to_string();
    } else {
        let user_name: String = command.user.name.to_string();

        let channel_name: String = command
            .channel_id
            .name(ctx.cache().unwrap())
            .await
            .unwrap_or(NOT_OBTAINED_STRING.to_string());

        let guild_name: String = command
            .guild_id
            .map(|guild_id| guild_id.name(ctx.cache().unwrap()))
            .flatten()
            .unwrap_or(NOT_OBTAINED_STRING.to_string());

        message = build_text_channel_notification_message(user_name, channel_name, guild_name);
    }

    send_notification_to_telegram(animation_url, message);
}
