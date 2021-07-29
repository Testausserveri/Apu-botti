use std::{collections::HashSet, env, sync::Arc};
mod utils;

use utils::translate::parse;

use serenity::{
    async_trait,
    client::bridge::gateway::ShardManager,
    framework::{standard::macros::group, StandardFramework, standard::Args},
    http::Http,
    model::{event::ResumedEvent, gateway::Ready, event::ReactionAddEvent},
    prelude::*,
};
use tracing::{error, info};
use tracing_subscriber::{EnvFilter, FmtSubscriber};
use std::process::exit;
use serenity::model::channel::Reaction;
use serenity::client::bridge::gateway::GatewayIntents;
use serenity::http::CacheHttp;
use serenity::client::bridge::gateway::*;

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn reaction_add(&self, ctx: Context, add_reaction:Reaction) {
        info!("reaction added");
        if add_reaction.clone().member.unwrap().roles.contains(&serenity::model::id::RoleId::from(870364054122819614)) {
            println!("{}",add_reaction.clone().emoji.as_data());
            if add_reaction.clone().emoji.to_string() == "😎" {
                let url = format!("https://www.google.com/search?q={}",parse(add_reaction.message(&ctx.http.http()).await.unwrap().content));
                //let body = reqwest::get(&url).await.unwrap().text().await.unwrap();
                add_reaction.channel_id.send_message(&ctx.http,|m| {
                    m.embed( |e| {
                        e.title("Have you tried google?");
                        e.field("Try this",&url,true);
                        e
                    });
                    m
                }).await.unwrap();
            };
        };
    }

    async fn ready(&self, _: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
}

#[group]
// #[commands()]
struct General;

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load .env file");

    let subscriber =
        FmtSubscriber::builder().with_env_filter(EnvFilter::from_default_env()).finish();

    tracing::subscriber::set_global_default(subscriber).expect("Failed to start the logger");

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let http = Http::new_with_token(&token);

    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let framework =
        StandardFramework::new().configure(|c|
            c
                .owners(owners)
                .prefix("v;"))
            .group(&GENERAL_GROUP);

    let mut client = Client::builder(&token)
        .framework(framework)
        .event_handler(Handler)
        //.intents(GatewayIntents::GUILD_MESSAGES | GatewayIntents::GUILDS)
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    }

    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.expect("Could not register ctrl+c handler");
        shard_manager.lock().await.shutdown_all().await;
    });

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}