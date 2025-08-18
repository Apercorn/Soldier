pub mod commands;
pub mod entity;
pub mod events;
#[macro_use]
pub mod utils;

pub use utils::embeds;
pub use utils::emojis;
pub use utils::error;
pub use utils::helper;

pub type CmdError = crate::utils::error::CmdError;
pub type CommandResult = anyhow::Result<(), CmdError>;
pub type Context<'a> = poise::ApplicationContext<'a, Data, CmdError>;

pub struct CommandContext<'a> {
  pub db: &'a sea_orm::DatabaseConnection,
  pub clan: Option<entity::clan::Model>,
  pub user: Option<entity::user::Model>,
  pub rankers: &'a Vec<RankerAccount>
}

pub struct RankerAccount {
  pub model: entity::account::Model,
  pub roblox: oxid_roblox::models::User,
  pub groups: Vec<oxid_roblox::models::SkinnyGroupWithMemberCount>
}

pub struct Data {
  pub db: sea_orm::DatabaseConnection,
  pub rankers: Vec<RankerAccount>
}
