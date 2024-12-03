use frankenstein::api_params::FileUpload;
use frankenstein::Api as FrankensteinApi;
use frankenstein::{SendAnimationParams, TelegramApi};
use std::ops::Deref;
use std::sync::LazyLock;

use crate::config;

static FRANKENSTEIN_API: LazyLock<FrankensteinApi> =
    LazyLock::new(|| FrankensteinApi::new(config::telegram_bot_token().as_str()));

pub fn send_notification_to_telegram(animation_url: &str, message: &str) {
    let send_animation_params: SendAnimationParams = SendAnimationParams::builder()
        .chat_id(config::telegram_chat_id())
        .animation(FileUpload::String(animation_url.to_string()))
        .caption(message)
        .build();

    println!(
        "Start. Sending to telegram. Animation url: {}. Caption: {}",
        animation_url, message
    );

    if let Err(why) = FRANKENSTEIN_API
        .deref()
        .send_animation(&send_animation_params)
    {
        println!(
            "Error. Could not send message to telegram. Trace: {:?}",
            why
        );
    } else {
        println!(
            "End. Sent to telegram. Animation url: {}. Caption: {}",
            animation_url, message
        );
    }
}
