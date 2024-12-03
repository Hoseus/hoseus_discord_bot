use std::env;
use std::ops::Deref;
use std::string::ToString;
use std::sync::LazyLock;

const DISCORD_BOT_TOKEN_ENV: &str = "DISCORD_BOT_TOKEN";
const TELEGRAM_BOT_TOKEN_ENV: &str = "TELEGRAM_BOT_TOKEN";
const TELEGRAM_CHAT_ID_ENV: &str = "TELEGRAM_CHAT_ID";
const TELEGRAM_INVITE_LINK_ENV: &str = "TELEGRAM_INVITE_LINK";

static DISCORD_BOT_TOKEN: LazyLock<String> = LazyLock::new(|| {
    env::var(DISCORD_BOT_TOKEN_ENV)
        .expect(format!("Env variable not defined: {}", DISCORD_BOT_TOKEN_ENV).as_str())
});

static TELEGRAM_BOT_TOKEN: LazyLock<String> = LazyLock::new(|| {
    env::var(TELEGRAM_BOT_TOKEN_ENV)
        .expect(format!("Env variable not defined: {}", TELEGRAM_BOT_TOKEN_ENV).as_str())
});

static TELEGRAM_CHAT_ID: LazyLock<String> = LazyLock::new(|| {
    env::var(TELEGRAM_CHAT_ID_ENV)
        .expect(format!("Env variable not defined: {}", TELEGRAM_CHAT_ID_ENV).as_str())
});

static TELEGRAM_INVITE_LINK: LazyLock<String> = LazyLock::new(|| {
    env::var(TELEGRAM_INVITE_LINK_ENV)
        .expect(format!("Env variable not defined: {}", TELEGRAM_INVITE_LINK_ENV).as_str())
});

pub fn discord_bot_token() -> String {
    DISCORD_BOT_TOKEN.deref().to_string()
}

pub fn telegram_bot_token() -> String {
    TELEGRAM_BOT_TOKEN.deref().to_string()
}

pub fn telegram_chat_id() -> String {
    TELEGRAM_CHAT_ID.deref().to_string()
}

pub fn telegram_invite_link() -> String {
    TELEGRAM_INVITE_LINK.deref().to_string()
}
