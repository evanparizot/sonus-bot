mod commands;

use std::{env, collections::HashSet, sync::Arc};

use serenity::{
    async_trait,
    model::{gateway::Ready, event::ResumedEvent},
    framework::{standard::macros::group, StandardFramework},
    prelude::*,
    http::Http, client::bridge::gateway::ShardManager
};

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        println!("Resumed");
    }
}

#[group]
#[commands(ping)]
struct General;

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let http = Http::new_with_token(&token);

    let (owners, bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        },
        Err(why) => panic!("Could not access application info: {:?}", why)
    };

    let framework = StandardFramework::new()
    .configure(|c| c.owners(owners).prefix("~")).group(&GENERAL_GROUP);

    let mut client =
        Client::builder(&token)
        .framework(framework)
        .event_handler(Handler).await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    }

    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.expect("Error setting CTRL-C handler");
        shard_manager.lock().await.shutdown_all().await;
    });

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}