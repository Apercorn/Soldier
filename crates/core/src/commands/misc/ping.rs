use crate::{embeds, helper, CommandResult, Context};

#[poise::command(
  slash_command,
  description_localized("en-US", "Checks the connection status of the bot")
)]
pub async fn ping(ctx: Context<'_>) -> CommandResult {
  let shard_id = ctx.serenity_context().shard_id;
  let runners = ctx.framework.shard_manager.runners.lock().await;
  let latency = runners
    .get(&shard_id)
    .and_then(|r| r.latency)
    .map(|l| format!("{}ms", l.as_millis()))
    .unwrap_or_else(|| "N/A".to_string());
  
  drop(runners);

  helper::send!(ctx,
    .embed(embeds::success("Pong!", format!("Gateway latency: `{latency}`")))
    .ephemeral(true)
  )?;

  Ok(())
}
