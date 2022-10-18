use std::borrow::Borrow;

use serenity::builder::CreateApplicationCommand;
use serenity::client::Context;
use serenity::model::application::command::CommandOptionType;
use serenity::model::application::interaction::application_command::{
    ApplicationCommandInteraction, CommandDataOption,
};

use crate::animation;

pub const COMMAND_NAME: &str = "animations";

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name(COMMAND_NAME)
        .description("Notify everyone via text channel")
        .create_option(|option| {
            option
                .name("list")
                .description("List all animation urls")
                .kind(CommandOptionType::SubCommand)
        })
}

pub async fn run(_ctx: Context, command: ApplicationCommandInteraction) -> Result<String, String> {
    let options: &[CommandDataOption] = &command.data.options;

    let sub_command_list: CommandDataOption = options.get(0).unwrap().to_owned();

    if let CommandOptionType::SubCommand = sub_command_list.kind {
        return Err("Invalid command".to_string());
    };

    let result: String = animation::get_animation_urls().iter().enumerate().fold(
        "".to_string(),
        |acc, (i, animation_url)| {
            acc + format!(
                "Index: {} | Url: {}\n",
                i.to_owned(),
                animation_url.to_owned()
            )
            .borrow()
        },
    );

    Ok(result)
}
