use serenity::all::{
    Client as SerenityClient, Command, Context, EventHandler, GatewayIntents, Interaction, Ready,
    VoiceState,
};

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
        let user_name = ready.user.name.as_str();

        println!("Start. Ready event. User: {}", user_name);

        println!(
            "User {} connected to discord servers successfully",
            user_name
        );

        println!("Creating application commands");

        if let Err(why) = Command::set_global_commands(
            &ctx,
            vec![
                commands::notify_command::register(),
                commands::animations_command::register(),
                commands::invite_command::register(),
            ],
        )
        .await
        {
            println!("Error. Could not create commands. Trace: {:?}", why)
        } else {
            println!("Created commands");

            println!("End. Ready event. User: {}", user_name);
        }
    }

    async fn voice_state_update(
        &self,
        ctx: Context,
        old_voice_state: Option<VoiceState>,
        new_voice_state: VoiceState,
    ) {
        let user_name: &str =
            &serenity_model_helper::get_user_name_from_voice_state(&ctx, &new_voice_state)
                .await;

        let channel_name: &str =
            &serenity_model_helper::get_channel_name_from_voice_state(&ctx, &new_voice_state)
                .await;

        let guild_name: &str =
            &serenity_model_helper::get_guild_name_from_voice_state(&ctx, &new_voice_state)
                .await;

        println!(
            "Start. Voice state update event. UserName: {}. ChannelName: {}. GuildName: {}",
            user_name, channel_name, guild_name
        );

        if old_voice_state.is_none() && new_voice_state.channel_id.is_some() {
            let member_count =
                serenity_model_helper::get_voice_channel_members_count_from_voice_state(
                    &ctx,
                    &new_voice_state,
                )
                .await;

            if member_count <= 1 {
                let animation_url: &str = &animation::get_random_animation_url();
                let message: &str = &message_helper::build_voice_channel_notification_message(
                    user_name,
                    channel_name,
                    guild_name,
                );
                telegram::send_notification_to_telegram(animation_url, message);

                println!(
                    "End. Voice state update event. UserName: {}. ChannelName: {}. GuildName: {}",
                    user_name, channel_name, guild_name
                );
            } else {
                println!(
                    "Discarded. Voice state update event. Discarded because there is more than one member in voice channel. UserName: {}. ChannelName: {}. GuildName: {}",
                    user_name,
                    channel_name,
                    guild_name
                );
            }
        } else {
            println!(
                "Discarded. Voice state update event. Discarded because there is an old state for user. UserName: {}. ChannelName: {}. GuildName: {}",
                user_name,
                channel_name,
                guild_name
            );
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            let command_name: &str = command.data.name.as_str();
            let user_name: &str = command.user.name.as_str();
            let channel_name: &str =
                &serenity_model_helper::get_channel_name_from_application_command(&ctx, &command)
                    .await;

            println!(
                "Start. Application command interaction. CommandName: {}, UserName: {}. ChannelName: {}",
                command_name,
                user_name,
                channel_name
            );

            let command_interaction_result = match command_name {
                commands::notify_command::COMMAND_NAME => {
                    commands::notify_command::run(&ctx, &command).await
                }
                commands::animations_command::COMMAND_NAME => {
                    commands::animations_command::run(&ctx, &command).await
                }
                commands::invite_command::COMMAND_NAME => {
                    commands::invite_command::run(&ctx, &command).await
                }
                _ => {
                    serenity_command_helper::respond_interaction_with_string(
                        &ctx,
                        &command,
                        "Error! Command does not exist!",
                    )
                    .await
                }
            };

            if let Err(_) = command_interaction_result {
                println!("Error. Failure running command. CommandName: {}, UserName: {}. ChannelName: {}.",
                     command_name,
                     user_name,
                     channel_name,
                );
            } else {
                println!(
                    "End. Application command interaction. CommandName: {}, UserName: {}. ChannelName: {}",
                    command_name,
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
