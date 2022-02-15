use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use tracing::info;

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    info!("Got pinged");
    
    msg.channel_id.say(&ctx.http, "Pong!").await?;

    Ok(())
}