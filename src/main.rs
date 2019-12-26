use std::env;

#[macro_use]
extern crate lazy_static;

use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use serenity::model::id::{GuildId, UserId, ChannelId};
use serenity::model::voice::VoiceState;
use serenity::Client as SerenityClient;
use reqwest::Client as ReqwestClient;
use std::time::Duration;
use serenity::http::CacheHttp;
use serenity::model::user::User;
use serenity::model::guild::PartialGuild;

struct Handler;

const TELEGRAM_API_URL: &str = "https://api.telegram.org";

const DISCORD_BOT_TOKEN: &'static str = env!("DISCORD_BOT_TOKEN");
const TELEGRAM_BOT_TOKEN: &'static str = env!("TELEGRAM_BOT_TOKEN");
const TELEGRAM_CHAT_ID: &'static str = env!("TELEGRAM_CHAT_ID");

lazy_static! {
    static ref REQWEST_CLIENT: ReqwestClient = ReqwestClient::new();
}

impl EventHandler for Handler {



    // Set a handler for the `message` event - so that whenever a new message
    // is received - the closure (or function) passed will be called.
    //
    // Event handlers are dispatched through a threadpool, and so multiple
    // events can be dispatched simultaneously.
    fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "-ring" {
            let user: User = msg.author;
            let user_name: &str = user.name.as_str();

            let channel_id: ChannelId = msg.channel_id;
            let channel_name: String = channel_id.name(ctx.cache().unwrap()).unwrap();

            let guild_id: GuildId = msg.guild_id.unwrap();
            let guild: PartialGuild = guild_id.to_partial_guild(ctx.http()).unwrap();
            let guild_name: String = guild.name;

            let message: String = format!("Ring! *{}* is calling in text channel *{}* in server *{}*", user_name, channel_name, guild_name);

            send_message_to_telegram(message.as_str());

        }
    }

    // Set a handler to be called on the `ready` event. This is called when a
    // shard is booted, and a READY payload is sent by Discord. This payload
    // contains data like the current user's guild Ids, current user data,
    // private channels, and more.
    //
    // In this case, just print what the current user's username is.
    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

    fn voice_state_update(&self, _ctx: Context, _guild_id: Option<GuildId>, _old_voice_state: Option<VoiceState>, new_voice_state: VoiceState) {
        let user_id: UserId = new_voice_state.user_id;
        let user: User = user_id.to_user(_ctx.http()).unwrap();
        let user_name: &str = user.name.as_str();

        let channel_id: ChannelId = new_voice_state.channel_id.unwrap();
        let channel_name: String = channel_id.name(_ctx.cache().unwrap()).unwrap();

        let guild_id: GuildId = _guild_id.unwrap();
        let guild: PartialGuild = guild_id.to_partial_guild(_ctx.http()).unwrap();
        let guild_name: String = guild.name;

        let message: String = format!("Ring! *{}* joined to voice channel *{}* in server *{}*", user_name, channel_name, guild_name);

        send_message_to_telegram(message.as_str());
    }
}

fn main() {
    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut serenity_client = SerenityClient::new_with_cache_update_timeout(DISCORD_BOT_TOKEN, Handler, Some(Duration::from_secs(60))).expect("Error creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = serenity_client.start() {
        println!("Serenity client error: {:?}", why);
    }
}

fn send_message_to_telegram(message: &str) {
    let url: String = format!("{}/bot{}/sendMessage?parse_mode=Markdown&chat_id={}&text={}", TELEGRAM_API_URL, TELEGRAM_BOT_TOKEN, TELEGRAM_CHAT_ID, message);

    println!("Sending to telegram: {}", url);

    let _response = reqwest::get(url.as_str());
}