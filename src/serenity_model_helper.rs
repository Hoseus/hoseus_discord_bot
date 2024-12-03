use serenity::all::{
    ChannelId, CommandInteraction, Context, GuildChannel, GuildId, UserId, VoiceState,
};

pub const NOT_OBTAINED_STRING: &str = "<not_obtained>";

async fn get_user_name(ctx: &Context, user_id: Option<&UserId>) -> String {
    match user_id {
        Some(some_user_id) => some_user_id
            .to_user(ctx)
            .await
            .map(|user| user.name)
            .unwrap_or(NOT_OBTAINED_STRING.to_string()),
        None => NOT_OBTAINED_STRING.to_string(),
    }
}

async fn get_channel_name(ctx: &Context, channel_id: Option<&ChannelId>) -> String {
    match channel_id {
        Some(some_channel_id) => some_channel_id
            .name(ctx)
            .await
            .unwrap_or(NOT_OBTAINED_STRING.to_string()),
        None => NOT_OBTAINED_STRING.to_string(),
    }
}

async fn get_guild_name(ctx: &Context, guild_id: Option<&GuildId>) -> String {
    match guild_id {
        Some(some_guild_id) => some_guild_id
            .name(ctx)
            .unwrap_or(NOT_OBTAINED_STRING.to_string()),
        None => NOT_OBTAINED_STRING.to_string(),
    }
}

async fn get_guild_channel(ctx: &Context, channel_id: Option<&ChannelId>) -> Option<GuildChannel> {
    match channel_id {
        Some(some_channel_id) => some_channel_id
            .to_channel(ctx)
            .await
            .ok()
            .map(|channel| channel.guild())
            .flatten(),
        None => None,
    }
}

async fn get_voice_channel_members_count(ctx: &Context, channel_id: Option<&ChannelId>) -> usize {
    match get_guild_channel(ctx, channel_id).await {
        Some(some_guild_channel) => some_guild_channel
            .members(ctx)
            .map(|members| members.len())
            .unwrap(),
        None => 0,
    }
}

pub async fn get_user_name_from_voice_state(ctx: &Context, voice_state: &VoiceState) -> String {
    get_user_name(ctx, Some(&voice_state.user_id)).await
}

pub async fn get_channel_name_from_voice_state(ctx: &Context, voice_state: &VoiceState) -> String {
    get_channel_name(ctx, voice_state.channel_id.as_ref()).await
}

pub async fn get_guild_name_from_voice_state(ctx: &Context, voice_state: &VoiceState) -> String {
    get_guild_name(ctx, voice_state.guild_id.as_ref()).await
}

pub async fn get_voice_channel_members_count_from_voice_state(
    ctx: &Context,
    voice_state: &VoiceState,
) -> usize {
    get_voice_channel_members_count(ctx, voice_state.channel_id.as_ref()).await
}

pub async fn get_channel_name_from_application_command(
    ctx: &Context,
    command: &CommandInteraction,
) -> String {
    get_channel_name(ctx, Some(&command.channel_id)).await
}

pub async fn get_guild_name_from_application_command(
    ctx: &Context,
    command: &CommandInteraction,
) -> String {
    get_guild_name(ctx, command.guild_id.as_ref()).await
}
