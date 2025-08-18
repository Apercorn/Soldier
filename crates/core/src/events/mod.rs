use crate::{CmdError, CommandResult, Data};
use poise::FrameworkContext;
use poise::serenity_prelude as serenity;

pub async fn handle_event(
  ctx: &serenity::Context,
  event: &serenity::FullEvent,
  _framework: FrameworkContext<'_, Data, CmdError>,
  _data: &Data,
) -> CommandResult {
  match event {
    serenity::FullEvent::Ready { data_about_bot, .. } => {
      ready::on_ready(ctx, &data_about_bot).await;
    }
    // More events here
    _ => {}
  }
  Ok(())
}

pub mod ready;
