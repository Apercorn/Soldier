use crate::{CmdError, Data};
use poise::Command;

pub fn all() -> Vec<Command<Data, CmdError>> {
  let mut cmds = vec![];
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
