use crate::{CmdError, Data};
use poise::Command;

pub fn commands() -> Vec<Command<Data, CmdError>> {
  vec![accounts::accounts(), eval::eval()]
}

pub mod accounts;
pub mod eval;
