pub mod commands;
pub mod events;

pub struct Data;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type CommandResult = Result<(), Error>;
pub type Context<'a> = poise::Context<'a, Data, Error>;