use crate::{CommandResult, Context};

#[poise::command(slash_command, description_localized("en-US", "Checks the connection status of the bot"))]
pub async fn ping(ctx: Context<'_>) -> CommandResult {
  let _ = ctx.say("text").await;
  Ok(())
}