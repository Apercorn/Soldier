use crate::{Data, Error};
use poise::Command;

pub fn commands() -> Vec<Command<Data, Error>> {
  vec![ping::ping()]
}

pub mod ping;
