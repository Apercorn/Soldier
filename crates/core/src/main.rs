use dotenv::dotenv;
use std::env;

use tracing::error;
use tracing_subscriber::{self, EnvFilter};

use poise::serenity_prelude as serenity;
use serenity::prelude::*;
use soldier_core::{CommandResult, Context, Data, Error};

use sea_orm::Database;

pub mod commands;
pub mod events;
pub mod utils;
pub mod entity;

#[tokio::main]
async fn main() {
  dotenv().ok(); // Load .env variables

  // Setup tracing information to the console
  tracing_subscriber::fmt()
    .with_env_filter(EnvFilter::from_default_env())
    .with_test_writer()
    .init();

  // ====== Database ======
  let conn_string = env::var("DATABASE_URL").expect("Expected DATABASE_URL in .env");
  let db = Database::connect(conn_string)
    .await
    .expect("Database connection failed");

  // ====== Bot Login ======
  let token = env::var("DISCORD_TOKEN").expect("Expected DISCORD_TOKEN in .env");
  let http = serenity::Http::new(&token);

  let intents = GatewayIntents::GUILD_MESSAGES
    | GatewayIntents::DIRECT_MESSAGES
    | GatewayIntents::MESSAGE_CONTENT;

  // Fetch guild count and determine appropriate shard count
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

  #[rustfmt::skip]
  let framework = poise::Framework::builder()
    .options(options)
    .setup(|_ctx, _ready, _framework|
      Box::pin(async move {
        Ok(Data {
          db
        })
      })
    )
    .build();

  let mut client = serenity::ClientBuilder::new(&token, intents)
    .framework(framework)
    .await
    .expect("Error creating client");

  if let Err(why) = client.start_shards(shard_count).await {
    error!("Failed to connect {why:?}");
  }
}
