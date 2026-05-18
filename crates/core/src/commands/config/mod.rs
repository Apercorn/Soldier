use crate::{CmdError, Data};
use poise::Command;

pub fn commands() -> Vec<Command<Data, CmdError>> {
  vec![/* setup::setup(),*/ verify::verify(), /* settings::settings() */]
}

// pub mod settings;
// pub mod setup;
pub mod verify;
