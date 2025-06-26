use dotenv::dotenv;
use std::env;

use tracing::{error, info};
use tracing_subscriber;

use soldier_core::{Data, Error, Context, CommandResult};
use poise::serenity_prelude as serenity;
use serenity::prelude::*;

mod commands;
mod events;

#[tokio::main]
async fn main() {
  // Setup tracing information to the console
  tracing_subscriber::fmt::init();
  dotenv().ok();

  let token = env::var("DISCORD_TOKEN").expect("Expected a token in .env");
  let http = serenity::Http::new(&token);

  let intents = GatewayIntents::GUILD_MESSAGES
    | GatewayIntents::DIRECT_MESSAGES
    | GatewayIntents::MESSAGE_CONTENT;

  let app_info = http
    .get_current_application_info()
    .await
    .expect("Could not fetch application info");
  let shard_count = app_info.approximate_guild_count.unwrap() / 1000 + 1;

  let options = poise::FrameworkOptions {
    commands: commands::all(),
    event_handler: |ctx, event, framework, data| {
      Box::pin(events::handle_event(ctx, event, framework, data))
    },
    // on_error
    ..Default::default()
  };

  let framework = poise::Framework::builder()
    .options(options)
    .setup(|_ctx, _ready, _framework| Box::pin(async move { Ok(Data) }))
    .build();

  let mut client = serenity::ClientBuilder::new(&token, intents)
    .framework(framework)
    .await
    .expect("Error creating client");

  if let Err(why) = client.start_shards(shard_count).await {
    error!("Failed to connect {why:?}");
  }
}
