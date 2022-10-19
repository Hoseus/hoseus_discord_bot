use serenity::client::Context;
use serenity::http::CacheHttp;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::channel::GuildChannel;
use serenity::model::id::{ChannelId, GuildId, UserId};
use serenity::model::prelude::VoiceState;

pub const NOT_OBTAINED_STRING: &str = "<not_obtained>";

async fn get_user_name(ctx: Context, user_id: Option<UserId>) -> String {
    match user_id {
        Some(some_user_id) => some_user_id
            .to_user_cached(ctx.cache().unwrap())
            .await
            .map(|user| user.name)
            .unwrap_or(NOT_OBTAINED_STRING.to_string()),
        None => NOT_OBTAINED_STRING.to_string(),
    }
}

async fn get_channel_name(ctx: Context, channel_id: Option<ChannelId>) -> String {
    match channel_id {
        Some(some_channel_id) => some_channel_id
            .name(ctx.cache().unwrap())
            .await
            .unwrap_or(NOT_OBTAINED_STRING.to_string()),
        None => NOT_OBTAINED_STRING.to_string(),
    }
}

async fn get_guild_name(ctx: Context, guild_id: Option<GuildId>) -> String {
    match guild_id {
        Some(some_guild_id) => some_guild_id
            .name(ctx.cache().unwrap())
            .unwrap_or(NOT_OBTAINED_STRING.to_string()),
        None => NOT_OBTAINED_STRING.to_string(),
    }
}

async fn get_voice_channel_members_count(ctx: Context, channel_id: Option<ChannelId>) -> usize {
    let guild_channel: Option<GuildChannel> = channel_id
        .map(|channel_id| channel_id.to_channel_cached(ctx.cache().unwrap()))
        .flatten()
        .map(|channel| channel.guild())
        .flatten();

    match guild_channel {
        Some(some_guild_channel) => some_guild_channel
            .members(ctx.cache().unwrap())
            .await
            .map(|members| members.len())
            .unwrap(),
        None => 0,
    }
}

pub async fn get_user_name_from_voice_state(ctx: Context, voice_state: VoiceState) -> String {
    get_user_name(ctx, Some(voice_state.user_id)).await
}

pub async fn get_channel_name_from_voice_state(ctx: Context, voice_state: VoiceState) -> String {
    get_channel_name(ctx, voice_state.channel_id).await
}

pub async fn get_guild_name_from_voice_state(ctx: Context, voice_state: VoiceState) -> String {
    get_guild_name(ctx, voice_state.guild_id).await
}

pub async fn get_voice_channel_members_count_from_voice_state(
    ctx: Context,
    voice_state: VoiceState,
) -> usize {
    get_voice_channel_members_count(ctx, voice_state.channel_id).await
}

pub async fn get_channel_name_from_application_command(
    ctx: Context,
    command: ApplicationCommandInteraction,
) -> String {
    get_channel_name(ctx, Some(command.channel_id)).await
}

pub async fn get_guild_name_from_application_command(
    ctx: Context,
    command: ApplicationCommandInteraction,
) -> String {
    get_guild_name(ctx, command.guild_id).await
}
