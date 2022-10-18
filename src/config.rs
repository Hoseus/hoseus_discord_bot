use std::borrow::Borrow;
use std::env;
use std::string::ToString;

const DISCORD_BOT_TOKEN_ENV: &str = "DISCORD_BOT_TOKEN";
const TELEGRAM_BOT_TOKEN_ENV: &str = "TELEGRAM_BOT_TOKEN";
const TELEGRAM_CHAT_ID_ENV: &str = "TELEGRAM_CHAT_ID";

lazy_static! {
    static ref DISCORD_BOT_TOKEN: String = env::var(DISCORD_BOT_TOKEN_ENV)
        .expect(format!("Env variable not defined: {}", DISCORD_BOT_TOKEN_ENV).borrow());
    static ref TELEGRAM_BOT_TOKEN: String = env::var(TELEGRAM_BOT_TOKEN_ENV)
        .expect(format!("Env variable not defined: {}", TELEGRAM_BOT_TOKEN_ENV).borrow());
    static ref TELEGRAM_CHAT_ID: String = env::var(TELEGRAM_CHAT_ID_ENV)
        .expect(format!("Env variable not defined: {}", TELEGRAM_CHAT_ID_ENV).borrow());
}

pub fn discord_bot_token() -> String {
    DISCORD_BOT_TOKEN.to_string()
}

pub fn telegram_bot_token() -> String {
    TELEGRAM_BOT_TOKEN.to_string()
}

pub fn telegram_chat_id() -> String {
    TELEGRAM_CHAT_ID.to_string()
}
