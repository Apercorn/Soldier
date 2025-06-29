pub mod commands;
pub mod entity;
pub mod events;
pub mod utils;

pub use utils::embeds;
pub use utils::emojis;

pub struct Data {
  pub db: sea_orm::DatabaseConnection,
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type CommandResult = Result<(), Error>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
