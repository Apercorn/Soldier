use poise::serenity_prelude as serenity;

use crate::{embeds, response, send, CmdError, CommandResult, Context};

#[poise::command(slash_command, description_localized("en-US", "TODO"))]
pub async fn simple_command(ctx: Context<'_>) -> CommandResult {
  helper::send!(ctx,
    .embed(embeds::success("Title", "Description"))
    .ephemeral(true)
  )?;

  Ok(())
}