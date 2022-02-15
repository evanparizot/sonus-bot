mod commands;
mod hooks;

use std::{collections::{HashSet, HashMap}, env, sync::{Arc, atomic::{Ordering, AtomicUsize}}};

use commands::{ping::*, youtube::*, meta::*};
use hooks::{counter, counter::{MessageCount, CommandCounter}};
use serenity::{
    async_trait,
    client::bridge::gateway::ShardManager,
    framework::{standard::macros::{group}, StandardFramework},
    http::Http,
    model::{
        channel::Message,
        event::ResumedEvent,
        gateway::Ready,
    },
    prelude::*,
};
use tracing::{error, info};


pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }

    async fn message(&self, ctx: Context, message: Message) {
        if message.content.to_lowercase().contains("owo") {
            let count = {
                let data_read = ctx.data.read().await;
                data_read.get::<MessageCount>().expect("bal").clone()
            };
            count.fetch_add(1, Ordering::SeqCst);
        }


        info!("{}", message.id);
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
}

#[group]
#[commands(ping)]
struct General;

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    tracing_subscriber::fmt::init();

    let http = Http::new_with_token(&token);

    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let framework = StandardFramework::new()
        .configure(|c| c.owners(owners).prefix("!"))
        .before(counter::before)
        .group(&GENERAL_GROUP)
        .group(&YOUTUBE_GROUP)
        .group(&META_GROUP);

    let mut client = Client::builder(&token)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
        data.insert::<CommandCounter>(Arc::new(RwLock::new(HashMap::default())));
        data.insert::<MessageCount>(Arc::new(AtomicUsize::new(0)));
    }

    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Error setting CTRL-C handler");
        shard_manager.lock().await.shutdown_all().await;
    });

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}
