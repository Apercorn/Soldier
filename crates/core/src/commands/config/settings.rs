use crate::utils::helper;
use crate::{CommandResult, Context};

#[poise::command(
  slash_command,
  required_permissions = "ADMINISTRATOR",
  description_localized("en-US", "Configures settings in this server")
)]
pub async fn settings(
  ctx: Context<'_>,
  #[description = "The setting to view"] option: Option<SettingOption>,
) -> CommandResult {
  let helper::CommandContext { db, clan, user } = helper::command_context(&ctx).await;

  Ok(())
}

#[derive(poise::ChoiceParameter)]
enum SettingOption {
  #[name = "Group"]
  Group,
  #[name = "Ranker Account"]
  RankerAccount,
  #[name = "Rank Locks"]
  RankLocks,
  #[name = "Xp Settings"]
  XpSettings,
  #[name = "Verification Role"]
  VerificationSettings,
  #[name = "Ranking Access Roles"]
  RankingAccessRoles,
  #[name = "Ranking Request System"]
  RankingRequestSystem,
  #[name = "Ranking Request Role"]
  RankingRequestRole,
  #[name = "Ranking Request Channel"]
  RankingRequestChannel,
  #[name = "Ranking Log Channel"]
  RankingLogChannel,
  #[name = "Role Levels"]
  RoleLevels,
  #[name = "Events"]
  Events,
  #[name = "Medals"]
  Medals,
}
