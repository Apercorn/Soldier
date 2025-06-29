use poise::Command;
use crate::{CommandResult, Context, Data, Error};

#[poise::command(prefix_command)]
pub async fn register(ctx: Context<'_>) -> CommandResult {
  poise::builtins::register_application_commands_buttons(ctx).await?;
  Ok(())
}

pub fn all() -> Vec<Command<Data, Error>> {
  let mut cmds = vec![register()];
  cmds.extend(config::commands());
  cmds.extend(dev::commands());
  cmds.extend(misc::commands());
  cmds.extend(roblox::commands());

  cmds
}

pub mod config;
pub mod dev;
pub mod misc;
pub mod roblox;