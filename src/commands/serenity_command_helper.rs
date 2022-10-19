use serenity::builder::CreateInteractionResponse;
use serenity::client::Context;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::InteractionResponseType;

pub async fn respond_interaction<'a, F>(
    ctx: Context,
    command: ApplicationCommandInteraction,
    f: F,
) -> Result<(), ()>
where
    for<'b> F:
        FnOnce(&'b mut CreateInteractionResponse<'a>) -> &'b mut CreateInteractionResponse<'a>,
{
    if let Err(why) = command.create_interaction_response(&ctx.http, f).await {
        println!(
            "Error. Cannot respond to slash command. CommandName: {}. Trace: {:?}",
            command.data.name, why
        );

        return Err(());
    }

    Ok(())
}

pub async fn respond_interaction_with_string(
    _ctx: Context,
    command: ApplicationCommandInteraction,
    response_message: String,
) -> Result<(), ()> {
    respond_interaction(_ctx, command.to_owned(), |response| {
        response
            .kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|message| message.content(response_message))
    })
    .await
}
