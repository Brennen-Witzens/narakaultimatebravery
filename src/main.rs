use std::env;

mod commands;
use poise::{framework, serenity_prelude as poise_serenity};
//use serenity::async_trait;
//use serenity::model::channel::Message;
//use serenity::model::gateway::Ready;
//use serenity::prelude::*;
use std::{collections::HashMap, sync::Mutex};

//struct Data {} // User data, which is stored and accessible in all command invocations
// Custom user data passed to all command functions
pub struct Data {
    votes: Mutex<HashMap<String, u32>>,
}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

struct Handler;

//#[async_trait]
//impl EventHandler for Handler {
// Set a handler for the 'message' event - so that whenever a message is recieved
// the closure (or function) passed will be called.

// Event handlers are dispatched through a threadpool and so multiple events can be
// dispatched simulaneously
//
//    async fn message(&self, ctx: Context, msg: Message) {
//        if msg.content == "!ping" {
//            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
//                println!("Error sending message: {why:?}");
//            }
//        }
//    }
//
//    async fn ready(&self, _: Context, ready: Ready) {
//        println!("{} is connected!", ready.user.name);
//    }
//}

#[poise::command(prefix_command)]
async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<poise_serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

#[actix_web::main]
async fn main() {
    // Load env files
    dotenvy::dotenv().ok();

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment variable");
    let intents = poise_serenity::GatewayIntents::non_privileged()
        | poise_serenity::GatewayIntents::MESSAGE_CONTENT;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![age(), commands::getRandomNum(), commands::ultimatebravery()],
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("!".into()),
                ..Default::default()
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    votes: Mutex::new(HashMap::new()),
                })
            })
        })
        .build();

    let client = poise_serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}

//let intents = GatewayIntents::GUILD_MESSAGES
//    | GatewayIntents::DIRECT_MESSAGES
//    | GatewayIntents::MESSAGE_CONTENT;

//let mut client = Client::builder(token, intents)
//    .event_handler(Handler)
//    .await
//    .expect("Error creating client");

//// Start listening for events by starting a single shard
//if let Err(why) = client.start().await {
//    println!("An error occurred while running the client: {:?}", why);
//}
//}
