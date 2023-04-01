#[macro_use]
extern crate lazy_static;

use serenity::Client as SerenityClient;
use serenity::client::Context;
use serenity::model::{gateway::Ready, voice::VoiceState};
use serenity::model::application::command::Command;
use serenity::model::application::interaction::Interaction;
use serenity::prelude::*;

use crate::commands::serenity_command_helper;

mod animation;
mod commands;
mod config;
mod message_helper;
mod serenity_model_helper;
mod telegram;

struct Handler;

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("Start. Ready event. User: {}", ready.user.name);

        println!(
            "User {} connected to discord servers successfully",
            ready.user.name
        );

        println!("Creating application commands");

        if let Err(why) =
            Command::set_global_application_commands(&ctx.http, |create_application_commands| {
                create_application_commands
                    .create_application_command(|create_application_command| {
                        commands::notify_command::register(create_application_command)
                    })
                    .create_application_command(|create_application_command| {
                        commands::animations_command::register(create_application_command)
                    })
                    .create_application_command(|create_application_command| {
                        commands::invite_command::register(create_application_command)
                    })
            })
            .await
        {
            println!("Error. Could not create commands. Trace: {:?}", why)
        } else {
            println!("Created commands");

            println!("End. Ready event. User: {}", ready.user.name);
        }
    }

    async fn voice_state_update(
        &self,
        ctx: Context,
        old_voice_state: Option<VoiceState>,
        new_voice_state: VoiceState,
    ) {
        let user_name: String = serenity_model_helper::get_user_name_from_voice_state(
            ctx.to_owned(),
            new_voice_state.to_owned(),
        )
        .await;
        let channel_name: String = serenity_model_helper::get_channel_name_from_voice_state(
            ctx.to_owned(),
            new_voice_state.to_owned(),
        )
        .await;
        let guild_name: String = serenity_model_helper::get_guild_name_from_voice_state(
            ctx.to_owned(),
            new_voice_state.to_owned(),
        )
        .await;

        println!(
            "Start. Voice state update event. UserName: {}. ChannelName: {}. GuildName: {}",
            user_name.to_owned(),
            channel_name.to_owned(),
            guild_name.to_owned()
        );

        if old_voice_state.is_none() && new_voice_state.channel_id.is_some() {
            let member_count =
                serenity_model_helper::get_voice_channel_members_count_from_voice_state(
                    ctx.to_owned(),
                    new_voice_state.to_owned(),
                )
                .await;

            if member_count <= 1 {
                let animation_url: String = animation::get_random_animation_url();
                let message = message_helper::build_voice_channel_notification_message(
                    user_name.to_owned(),
                    channel_name.to_owned(),
                    guild_name.to_owned(),
                );
                telegram::send_notification_to_telegram(animation_url, message);

                println!(
                    "End. Voice state update event. UserName: {}. ChannelName: {}. GuildName: {}",
                    user_name.to_owned(),
                    channel_name.to_owned(),
                    guild_name.to_owned()
                );
            } else {
                println!(
                    "Discarded. Voice state update event. Discarded because there is more than one member in voice channel. UserName: {}. ChannelName: {}. GuildName: {}",
                    user_name.to_owned(),
                    channel_name.to_owned(),
                    guild_name.to_owned()
                );
            }
        } else {
            println!(
                "Discarded. Voice state update event. Discarded because there is an old state for user. UserName: {}. ChannelName: {}. GuildName: {}",
                user_name.to_owned(),
                channel_name.to_owned(),
                guild_name.to_owned()
            );
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let user_name = command.user.name.to_owned();
            let channel_name = serenity_model_helper::get_channel_name_from_application_command(
                ctx.to_owned(),
                command.to_owned(),
            )
            .await;

            println!(
                "Start. Application command interaction. CommandName: {}, UserName: {}. ChannelName: {}",
                command.data.name,
                user_name,
                channel_name
            );

            let command_interaction_result = match command.data.name.as_str() {
                commands::notify_command::COMMAND_NAME => {
                    commands::notify_command::run(ctx.to_owned(), command.to_owned()).await
                }
                commands::animations_command::COMMAND_NAME => {
                    commands::animations_command::run(ctx.to_owned(), command.to_owned()).await
                }
                commands::invite_command::COMMAND_NAME => {
                    commands::invite_command::run(ctx.to_owned(), command.to_owned()).await
                }
                _ => {
                    serenity_command_helper::respond_interaction_with_string(
                        ctx,
                        command.to_owned(),
                        "Error! Command does not exist!".to_string(),
                    )
                    .await
                }
            };

            if let Err(_) = command_interaction_result {
                println!("Error. Failure running command. CommandName: {}, UserName: {}. ChannelName: {}.",
                     command.data.name,
                     user_name,
                     channel_name,
                );
            } else {
                println!(
                    "End. Application command interaction. CommandName: {}, UserName: {}. ChannelName: {}",
                    command.data.name,
                    user_name,
                    channel_name
                );
            }
        }
    }
}

#[tokio::main]
async fn main() {
    println!("Start. main");

    let intents = GatewayIntents::GUILD_VOICE_STATES
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILDS;

    let mut serenity_client =
        SerenityClient::builder(config::discord_bot_token().as_str(), intents)
            .event_handler(Handler)
            .await
            .expect("Error. Could not create Serenity client");

    if let Err(why) = serenity_client.start().await {
        println!("Error. Serenity client error. Trace: {:?}", why);
    } else {
        println!("End. main");
    }
}
