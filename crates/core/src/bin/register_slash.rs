use dotenv::from_path;
use poise::serenity_prelude as serenity;
use std::env;

use soldier_core::commands;

#[tokio::main]
async fn main() {
  let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
  from_path(manifest_dir.join("../../.env")).ok();

  let token = env::var("DISCORD_TOKEN").expect("Expected DISCORD_TOKEN in .env");
  let app_id = env::var("APPLICATION_ID").expect("Expected APPLICATION_ID in .env")
    .parse::<u64>()
    .expect("Expected u64 for APPLICATION_ID");

  let http = serenity::Http::new(&token);
  http.set_application_id(serenity::ApplicationId::new(app_id));

  let cmds = poise::builtins::create_application_commands(&commands::all());
  let result = serenity::Command::set_global_commands(&http, cmds).await;

  let count = result
    .expect("Failed to set global application commands")
    .len();

  println!("Set {} global commands", count);
}
