use crate::{CmdError, Data};
use poise::Command;

pub fn commands() -> Vec<Command<Data, CmdError>> {
  vec![ping::ping()]
}

pub mod ping;
