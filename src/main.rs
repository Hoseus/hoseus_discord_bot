use std::env;

#[macro_use]
extern crate lazy_static;

use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use serenity::model::id::{GuildId};
use serenity::model::voice::VoiceState;
use serenity::Client as SerenityClient;
use reqwest::Client as ReqwestClient;
use serenity::http::CacheHttp;
use chrono::{DateTime, Utc, Duration};

struct Handler;

const DISCORD_BOT_TOKEN_ENV: &str = "DISCORD_BOT_TOKEN";
const TELEGRAM_BOT_TOKEN_ENV: &str = "TELEGRAM_BOT_TOKEN";
const TELEGRAM_CHAT_ID_ENV: &str = "TELEGRAM_CHAT_ID";
const TELEGRAM_POLL_TTL_ENV: &str = "TELEGRAM_POLL_TTL";

const TELEGRAM_API_URL: &str = "https://api.telegram.org";
const TELEGRAM_POLL_OPTIONS: &str = "[\"Yes\",\"No\"]";

const NOT_OBTAINED_STRING: &str = "<not_obtained>";

lazy_static! {
    static ref DISCORD_BOT_TOKEN: String = env::var(DISCORD_BOT_TOKEN_ENV).expect(format!("Env variable not defined: {}", DISCORD_BOT_TOKEN_ENV).clone().as_str());

    static ref TELEGRAM_BOT_TOKEN: String = env::var(TELEGRAM_BOT_TOKEN_ENV).expect(format!("Env variable not defined: {}", TELEGRAM_BOT_TOKEN_ENV).clone().as_str());
    static ref TELEGRAM_CHAT_ID: String = env::var(TELEGRAM_CHAT_ID_ENV).expect(format!("Env variable not defined: {}", TELEGRAM_CHAT_ID_ENV).clone().as_str());
    static ref TELEGRAM_POLL_TTL: Duration = Duration::seconds(env::var(TELEGRAM_POLL_TTL_ENV).expect(format!("Env variable not defined: {}", TELEGRAM_POLL_TTL_ENV).clone().as_str()).parse::<i64>().unwrap());

    static ref REQWEST_CLIENT: ReqwestClient = ReqwestClient::new();
}

static mut POLL_EXPIRATION: Option<DateTime<Utc>> = None;

fn main() {
    let mut serenity_client = SerenityClient::new(DISCORD_BOT_TOKEN.as_str(), Handler).expect("Error creating Serenity client");

    if let Err(why) = serenity_client.start() {
        println!("Serenity client error: {:?}", why);
    }
}

impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "-ring" {
            let user_name: String = msg.author.name;

            let channel_name: String = msg.channel_id.name(ctx.cache().unwrap()).unwrap_or(NOT_OBTAINED_STRING.to_string());

            let guild_name: String =
                msg.guild_id
                    .map(|guild_id| guild_id.to_partial_guild(ctx.http())).map(|partial_guild_result| partial_guild_result.ok()).flatten()
                    .map(|partial_guild| partial_guild.name)
                    .unwrap_or(NOT_OBTAINED_STRING.to_string());

            send_text_channel_poll_to_telegram(user_name, channel_name, guild_name);
        }

        if msg.content == "-reset_poll" {
            unsafe { POLL_EXPIRATION = None };
            let _result = msg.channel_id.say(ctx.http, "Success!");
        }
    }

    fn ready(&self, _: Context, ready: Ready) {
        println!("{} connected to discord server successfully", ready.user.name);
    }

    fn voice_state_update(&self, _ctx: Context, _guild_id: Option<GuildId>, _old_voice_state: Option<VoiceState>, new_voice_state: VoiceState) {
        match new_voice_state.channel_id {
            Some(_channel_id) => voice_state_join_channel(self, _ctx, _guild_id, _old_voice_state, new_voice_state),
            _ => ()
        }
    }
}

fn voice_state_join_channel(_handler: &Handler, _ctx: Context, _guild_id: Option<GuildId>, _old_voice_state: Option<VoiceState>, new_voice_state: VoiceState) {
    let now: DateTime<Utc> = Utc::now();

    println!("poll_ttl: {}", *TELEGRAM_POLL_TTL);
    println!("poll_ttl2: {}", Duration::seconds(600));
    println!("now: {}", now);

    unsafe {
        match POLL_EXPIRATION {
            Some(p) => println!("poll_expiration: {}", p),
            _ => ()
        }
    };

    if unsafe { POLL_EXPIRATION.is_none() || (POLL_EXPIRATION.is_some() && now > POLL_EXPIRATION.unwrap()) } {
        let user_name: String =
            new_voice_state.user_id
                .to_user(_ctx.http())
                .map(|user| user.name)
                .unwrap_or(NOT_OBTAINED_STRING.to_string());

        let channel_name: String =
            new_voice_state.channel_id
                .map(|channel_id| channel_id.name(_ctx.cache().unwrap()))
                .flatten()
                .unwrap_or(NOT_OBTAINED_STRING.to_string());

        let guild_name: String =
            _guild_id
                .map(|guild_id| guild_id.to_partial_guild(_ctx.http()))
                .map(|partial_guild_result| partial_guild_result.ok())
                .flatten()
                .map(|partial_guild| partial_guild.name)
                .unwrap_or(NOT_OBTAINED_STRING.to_string());

        send_voice_channel_poll_to_telegram(user_name, channel_name, guild_name);

        unsafe {
            POLL_EXPIRATION.replace(now + *TELEGRAM_POLL_TTL);
            match POLL_EXPIRATION {
                Some(p) => println!("poll_expiration: {}", p),
                _ => ()
            }
        };
    }
}

fn send_voice_channel_poll_to_telegram(user_name: String, channel_name: String, guild_name: String) {
    let question: String = format!("Ring! *{}* joined to voice channel *{}* in server *{}*. Are you joining?", user_name, channel_name, guild_name);

    send_poll_to_telegram(question);
}

fn send_text_channel_poll_to_telegram(user_name: String, channel_name: String, guild_name: String) {
    let question: String = format!("Ring! *{}* is calling in text channel *{}* in server *{}*. Are you joining?", user_name, channel_name, guild_name);

    send_poll_to_telegram(question);
}

fn send_poll_to_telegram(question: String) {
    let url_string: String = format!("{}/bot{}/sendPoll?chat_id={}&question={}&options={}", TELEGRAM_API_URL, *TELEGRAM_BOT_TOKEN, *TELEGRAM_CHAT_ID, question, TELEGRAM_POLL_OPTIONS);

    println!("Sending to telegram: {}", url_string);

    let _response = REQWEST_CLIENT.get(url_string.as_str()).send();
}