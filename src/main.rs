use std::env;

#[macro_use]
extern crate lazy_static;
extern crate core;

use frankenstein::{api_params::File, Api as FrankensteinApi, SendAnimationParams, TelegramApi};
use rand::seq::SliceRandom;
use serenity::model::channel::GuildChannel;
use serenity::{
    client::Context,
    framework::standard::macros::{command, group},
    framework::standard::CommandResult,
    framework::StandardFramework,
    http::CacheHttp,
    model::{channel::Message, gateway::Ready, voice::VoiceState},
    prelude::*,
    Client as SerenityClient,
};

const DISCORD_BOT_TOKEN_ENV: &str = "DISCORD_BOT_TOKEN";
const TELEGRAM_BOT_TOKEN_ENV: &str = "TELEGRAM_BOT_TOKEN";
const TELEGRAM_CHAT_ID_ENV: &str = "TELEGRAM_CHAT_ID";

const NOT_OBTAINED_STRING: &str = "<not_obtained>";

lazy_static! {
    static ref DISCORD_BOT_TOKEN: String = env::var(DISCORD_BOT_TOKEN_ENV).expect(
        format!("Env variable not defined: {}", DISCORD_BOT_TOKEN_ENV)
            .clone()
            .as_str()
    );
    static ref TELEGRAM_BOT_TOKEN: String = env::var(TELEGRAM_BOT_TOKEN_ENV).expect(
        format!("Env variable not defined: {}", TELEGRAM_BOT_TOKEN_ENV)
            .clone()
            .as_str()
    );
    static ref TELEGRAM_CHAT_ID: String = env::var(TELEGRAM_CHAT_ID_ENV).expect(
        format!("Env variable not defined: {}", TELEGRAM_CHAT_ID_ENV)
            .clone()
            .as_str()
    );
    static ref FRANKENSTEIN_API: FrankensteinApi = FrankensteinApi::new(&TELEGRAM_BOT_TOKEN);
    static ref ANIMATION_URLS: Vec<String> = get_animation_urls();
}

fn get_animation_urls() -> Vec<String> {
    let file_path = "animation_urls.json".to_string();
    let json_string: String = std::fs::read_to_string(file_path.to_string())
        .expect(format!("Unable to read file {}", file_path).as_str());
    let animation_urls: Vec<String> = serde_json::from_str(json_string.as_str())
        .expect(format!("Unable to parse json in {}", file_path).as_str());
    if animation_urls.is_empty() {
        panic!("Animation urls list cannot be empty");
    }
    return animation_urls;
}

#[group]
#[description = "Hoseus bot commands"]
#[commands(notify)]
struct General;

#[command]
#[description = "Notify everyone via text channel"]
async fn notify(ctx: &Context, msg: &Message) -> CommandResult {
    let user_name: String = msg.author.name.clone();

    let channel_name: String = msg
        .channel_id
        .name(ctx.cache().unwrap())
        .await
        .unwrap_or(NOT_OBTAINED_STRING.to_string());

    let guild_name: String = msg
        .guild_id
        .map(|guild_id| guild_id.name(ctx.cache().unwrap()))
        .flatten()
        .unwrap_or(NOT_OBTAINED_STRING.to_string());

    let animation_url: String = get_random_animation_url();
    let message = build_text_channel_notification_message(user_name, channel_name, guild_name);

    send_notification_to_telegram(animation_url, message);

    msg.reply(ctx, "Success!").await?;

    Ok(())
}

struct Handler;

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!(
            "{} connected to discord server successfully",
            ready.user.name
        );
    }

    async fn voice_state_update(
        &self,
        _ctx: Context,
        _old_voice_state: Option<VoiceState>,
        new_voice_state: VoiceState,
    ) {
        if _old_voice_state.is_none() && new_voice_state.channel_id.is_some() {
            let guild_channel: Option<GuildChannel> = new_voice_state
                .channel_id
                .map(|channel_id| channel_id.to_channel_cached(_ctx.cache().unwrap()))
                .flatten()
                .map(|channel| channel.guild())
                .flatten();

            let member_count = match guild_channel {
                Some(val) => val
                    .members(_ctx.cache().unwrap())
                    .await
                    .map(|members| members.len())
                    .unwrap_or(0),
                None => 0,
            };

            if member_count <= 1 {
                let user_name: String = new_voice_state
                    .user_id
                    .to_user(_ctx.http())
                    .await
                    .map(|user| user.name)
                    .unwrap_or(NOT_OBTAINED_STRING.to_string());

                let channel_name: String = match new_voice_state.channel_id {
                    Some(channel_id) => channel_id
                        .name(_ctx.cache().unwrap())
                        .await
                        .unwrap_or(NOT_OBTAINED_STRING.to_string()),
                    None => NOT_OBTAINED_STRING.to_string(),
                };

                let guild_name: String = new_voice_state
                    .guild_id
                    .map(|guild_id| guild_id.name(_ctx.cache().unwrap()))
                    .flatten()
                    .unwrap_or(NOT_OBTAINED_STRING.to_string());

                let animation_url: String = get_random_animation_url();
                let message =
                    build_voice_channel_notification_message(user_name, channel_name, guild_name);
                send_notification_to_telegram(animation_url, message);
            }
        }
    }
}

fn get_random_animation_url() -> String {
    let mut rng = rand::thread_rng();
    return ANIMATION_URLS
        .choose(&mut rng)
        .expect("List cannot be empty")
        .to_string();
}

fn build_voice_channel_notification_message(
    user_name: String,
    channel_name: String,
    guild_name: String,
) -> String {
    let message: String = format!(
        "*{}* joined to voice channel *{}* in server *{}*. Are you joining?",
        user_name, channel_name, guild_name
    );

    return message;
}

fn build_text_channel_notification_message(
    user_name: String,
    channel_name: String,
    guild_name: String,
) -> String {
    let message: String = format!(
        "*{}* is calling in text channel *{}* in server *{}*. Are you joining?",
        user_name, channel_name, guild_name
    );

    return message;
}

fn send_notification_to_telegram(animation_url: String, message: String) {
    let send_animation_params = SendAnimationParams::builder()
        .chat_id(TELEGRAM_CHAT_ID.to_string())
        .animation(File::String(animation_url.to_string()))
        .caption(message.to_string())
        .build();

    println!(
        "Sending to telegram. Animation url: {}. Caption: {}",
        animation_url, message
    );

    let _result = FRANKENSTEIN_API.send_animation(&send_animation_params);
}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("hbot -"))
        .group(&GENERAL_GROUP);

    let intents = GatewayIntents::GUILD_VOICE_STATES
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILDS;

    let mut serenity_client = SerenityClient::builder(DISCORD_BOT_TOKEN.as_str(), intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating Serenity client");

    if let Err(why) = serenity_client.start().await {
        println!("Serenity client error: {:?}", why);
    }
}
