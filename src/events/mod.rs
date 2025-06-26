use crate::{CommandResult, Data, Error};
use poise::serenity_prelude as serenity;
use poise::{FrameworkContext};

pub async fn handle_event(
  ctx: &serenity::Context,
  event: &serenity::FullEvent,
  _framework: FrameworkContext<'_, Data, Error>,
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
