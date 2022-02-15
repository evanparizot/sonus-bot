use std::sync::atomic::Ordering;

use serenity::client::Context;
use serenity::framework::standard::macros::group;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;

use crate::hooks::counter::MessageCount;

#[group]
#[commands(count)]
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
