use std::borrow::Borrow;

use frankenstein::{SendAnimationParams, TelegramApi};
use frankenstein::Api as FrankensteinApi;
use frankenstein::api_params::File;

use crate::config;

lazy_static! {
    static ref FRANKENSTEIN_API: FrankensteinApi =
        FrankensteinApi::new(&config::telegram_bot_token().borrow());
}

pub fn send_notification_to_telegram(animation_url: String, message: String) {
    let send_animation_params = SendAnimationParams::builder()
        .chat_id(config::telegram_chat_id().to_string())
        .animation(File::String(animation_url.to_string()))
        .caption(message.to_string())
        .build();

    println!(
        "Start. Sending to telegram. Animation url: {}. Caption: {}",
        animation_url, message
    );

    if let Err(why) = FRANKENSTEIN_API.send_animation(&send_animation_params) {
        println!(
            "Error. Could not send message to telegram. Trace: {:?}", why
        );
    } else {
        println!(
            "End. Sent to telegram. Animation url: {}. Caption: {}",
            animation_url, message
        );
    }
}
