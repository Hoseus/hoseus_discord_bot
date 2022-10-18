pub fn build_voice_channel_notification_message(
    user_name: String,
    channel_name: String,
    guild_name: String,
) -> String {
    let message: String = format!(
        "*{}* joined to voice channel *{}* in server *{}*. Are you joining?",
        user_name, channel_name, guild_name
    );

    message
}

pub fn build_text_channel_notification_message(
    user_name: String,
    channel_name: String,
    guild_name: String,
) -> String {
    let message: String = format!(
        "*{}* is calling in text channel *{}* in server *{}*. Are you joining?",
        user_name, channel_name, guild_name
    );

    message
}
