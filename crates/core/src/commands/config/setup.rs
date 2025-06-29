use sea_orm::EntityTrait;

use crate::{CommandResult, Context, utils::embeds};
use crate::entity::user::Entity as User;

#[derive(poise::ChoiceParameter)]
enum RankingSystem {
  #[name = "Direct Ranking"]
  DirectRanking,
  #[name = "Rank Requests"]
  RankRequest,
}

#[poise::command(
  slash_command,
  required_permissions = "ADMINISTRATOR",
  description_localized("en-US", "Configures the server for ranking")
)]
pub async fn setup(
  ctx: Context<'_>,
  #[description = "The default request system for ranking"]
  system: RankingSystem,
) -> CommandResult {
  match system {
    RankingSystem::DirectRanking => ctx.reply("direct").await?,
    RankingSystem::RankRequest => ctx.reply("request").await?,
  };

  let emb = embeds::success("test", "abc");

  ctx.send(poise::CreateReply {
    content: Some(String::from("Hello")),
    embeds: vec![emb],
    ..Default::default()
  }).await?;

  let db = &ctx.data().db;

  Ok(())
}
