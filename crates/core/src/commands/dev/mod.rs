use crate::{CmdError, Data};
use poise::Command;

pub fn commands() -> Vec<Command<Data, CmdError>> {
  vec![accounts::accounts(), eval::eval(), test::test()]
}

pub mod accounts;
pub mod eval;
pub mod test;
