use std::sync::atomic::Ordering;

use serenity::client::Context;
use serenity::framework::standard::macros::group;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use tracing::info;

use crate::hooks::counter::MessageCount;

#[group]
#[commands(count, ping)]
pub struct Meta;

#[command]
async fn count(ctx: &Context, msg: &Message) -> CommandResult {
    let raw_count = {
        let data_read = ctx.data.read().await;
        data_read.get::<MessageCount>().expect("Expected MessageCount in TypeMap").clone()
    };

    let count = raw_count.load(Ordering::Relaxed);

    if count == 1 {
        msg.reply(ctx, "You are the first to say this!").await?;
    } else {
        msg.reply(ctx, format!("owo has been said {} times", count)).await?;
    }
    Ok(())
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    info!("Got pinged");
    
    let msg = msg.channel_id.send_message(&ctx.http, |m| {
        m.content("Pong")
        .embed(|e| {
            e.title("This is a title")
            .description("This is a description")
            .fields(vec![
                ("First", "First Body", true),
                ("Second", "Second Body", true),
                ])
                .footer(|f| f.text("This is a footer"))
                .timestamp(chrono::Utc::now())
        })
    }).await;

    Ok(())
}