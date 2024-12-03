use serenity::all::{
    CommandInteraction, Context, CreateInteractionResponse, CreateInteractionResponseMessage,
};

pub async fn respond_interaction(
    ctx: &Context,
    command: &CommandInteraction,
    f: impl Fn() -> CreateInteractionResponse,
) -> Result<(), ()> {
    if let Err(why) = command.create_response(ctx, f()).await {
        println!(
            "Error. Cannot respond to slash command. CommandName: {}. Trace: {:?}",
            command.data.name, why
        );

        return Err(());
    }

    Ok(())
}

pub async fn respond_interaction_with_string(
    ctx: &Context,
    command: &CommandInteraction,
    response_message: &str,
) -> Result<(), ()> {
    respond_interaction(ctx, command, || {
        CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new().content(response_message),
        )
    })
    .await
}
