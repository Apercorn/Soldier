use anyhow::anyhow;
use oxid_roblox::derives::UserDerive;
use poise::serenity_prelude as serenity;

use crate::utils::{embeds, helper};
use crate::{CmdError, CommandResult, Context};

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
  #[description = "The default request system for ranking"] system: RankingSystem,
) -> CommandResult {
  let helper::CommandContext { db, clan, user } = helper::command_context(&ctx).await;
  // check if verified

  // Todo: Check if server already setup
  // if let Some(clan) = clan {}

  let group_modal = serenity::CreateQuickModal::new("Setup")
    .timeout(std::time::Duration::from_secs(180))
    .field(
      serenity::CreateInputText::new(
        serenity::InputTextStyle::Short,
        "Group Identifier",
        "setup::group_id",
      )
      .placeholder("A Roblox group ID or URL")
      .required(true)
      .max_length(10),
    );

  let group_modal_response = ctx
    .interaction
    .quick_modal(&ctx.serenity_context(), group_modal)
    .await?
    .ok_or(CmdError::Timeout)?;

  let input = group_modal_response.inputs.get(0).cloned().unwrap();

  // Parse group_id
  let group_id = {
    let regex = regex::Regex::new(r"roblox\.com/communities/(\d+)").unwrap();
    if let Some(matches) = regex.captures(input.as_str()) {
      matches.get(1).unwrap().as_str().parse::<i64>()
    } else {
      input.trim().parse::<i64>()
    }
  }
  .map_err(|_| {
    CmdError::Embed(embeds::error(
      "",
      "Failed to parse the group identifier—make sure it's a url or integer.",
    ))
  })?;

  // Fetch the group
  let group = oxid_roblox::group_from_id(group_id).await.map_err(|_| {
    CmdError::Embed(embeds::error("", "Failed to fetch the group—make sure it exists."))
  })?;

  // // Fetch the verified user's roblox account
  // let player = oxid_roblox::user_from_id(user.roblox_id)
  // .await
  //   .map_err(|_| CmdError::from(anyhow!("An unexpected error occured while fetching your Roblox info.")))?;

  // let group_roles = player.role_in_group(group.id)
  // .await
  // .map_err(|_| CmdError::from(anyhow!("You are not a member of this group")))?;
  // println!("{:?}", group_roles);

  // 1. Modal to get groupId
  // 2. Wait for user to submit

  // match system {
  //   RankingSystem::DirectRanking => ctx.reply("direct").await?,
  //   RankingSystem::RankRequest => ctx.reply("request").await?,
  // };

  Ok(())
}
