use serenity::client::Context;
use serenity::framework::standard::macros::group;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use tracing::info;

#[group]
#[commands(play, current, queue, skip, pause, start)]
pub struct Youtube;

#[command]
async fn play(ctx: &Context, msg: &Message) -> CommandResult {
    info!("played something");
    Ok(())
}

#[command]
async fn current(ctx: &Context, msg: &Message) -> CommandResult {
    info!("current something");
    Ok(())
}

#[command]
async fn queue(ctx: &Context, msg: &Message) -> CommandResult {
    info!("queued something");
    Ok(())
}

#[command]
async fn skip(ctx: &Context, _: &Message) -> CommandResult {
    info!("skipped something");
    Ok(())
}

#[command]
async fn pause(ctx: &Context, msg: &Message) -> CommandResult {
    info!("paused something");
    Ok(())
}

#[command]
async fn start(ctx: &Context, msg: &Message) -> CommandResult {
    info!("started something");
    Ok(())
}