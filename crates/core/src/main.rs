use std::env;
use dotenv::dotenv;

use sea_orm::{Database, EntityTrait};
use oxid_roblox::derives::UserDerive;
use ::serenity::futures::{stream, StreamExt};
use tracing_subscriber::{self, EnvFilter};
use tracing::error;

use serenity::prelude::*;
use poise::serenity_prelude as serenity;
use soldier_core::{CmdError, CommandResult, RankerAccount, Context, Data};


pub use soldier_core::commands;
pub use soldier_core::entity;
pub use soldier_core::events;
pub use soldier_core::utils;

#[tokio::main]
async fn main() {
  dotenv().ok(); // Load .env variables

  // Setup tracing information to the console
  tracing_subscriber::fmt()
    .with_env_filter(EnvFilter::from_default_env())
    .with_test_writer()
    .init();

  // =========== Database ===========
  let conn_string = env::var("DATABASE_URL").expect("Expected DATABASE_URL in .env");
  let db = Database::connect(conn_string)
    .await
    .expect("Database connection failed");

  // =========== Bot Login ===========
  let token = env::var("DISCORD_TOKEN").expect("Expected DISCORD_TOKEN in .env");
  let http = serenity::Http::new(&token);

  let intents = serenity::GatewayIntents::GUILD_MESSAGES
    | serenity::GatewayIntents::DIRECT_MESSAGES
    | serenity::GatewayIntents::GUILDS;

  // Fetch guild count and determine appropriate shard count
  let app_info = http
    .get_current_application_info()
    .await
    .expect("Could not fetch application info");
  let shard_count = app_info.approximate_guild_count.unwrap() / 1000 + 1;

  // ========== Roblox Init ==========
  let ranker_models = entity::account::Entity::find().all(&db)
    .await
    .expect("Could not fetch ranker accounts");

  let rankers = stream::iter(ranker_models)
    .filter(|acc| {
      let cookie = acc.cookie.clone();
      async move {
        oxid_roblox::authenticated_user(Some(cookie)).await.is_ok()
      }
    })
    .then(|acc| async move {
      // todo error handling
      let user = oxid_roblox::user_from_id(acc.roblox_id).await.expect("Failed to fetch user");
      let groups = match user.group_roles().await {
        Ok(g) => g.into_iter().map(|gr| gr.group).collect::<Vec<oxid_roblox::models::SkinnyGroupWithMemberCount>>(),
        Err(e) => panic!("Failed to fetch groups on a ranker account")
      };

      RankerAccount {
        model: acc,
        roblox: user,
        groups
      }
    })
    .collect::<Vec<_>>().await;

  // =========== Framework ===========
  let options = poise::FrameworkOptions {
    commands: commands::all(),
    event_handler: |ctx, event, framework, data| {
      Box::pin(events::handle_event(ctx, event, framework, data))
    },
    on_error: |error| Box::pin(utils::error::handle_error(error)),
    ..Default::default()
  };

  let framework = poise::Framework::builder()
    .options(options)
    .setup(|_ctx, _ready, _framework|
      Box::pin(async move {
        Ok(Data {
          db,
          rankers
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
