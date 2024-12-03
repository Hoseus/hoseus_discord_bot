pub fn build_voice_channel_notification_message(
    user_name: &str,
    channel_name: &str,
    guild_name: &str,
) -> String {
    format!(
        "*{}* joined to voice channel *{}* in server *{}*. Are you joining?",
        user_name, channel_name, guild_name
    )
}

pub fn build_text_channel_notification_message(
    user_name: &str,
    channel_name: &str,
    guild_name: &str,
) -> String {
    format!(
        "*{}* is calling in text channel *{}* in server *{}*. Are you joining?",
        user_name, channel_name, guild_name
    )
}
