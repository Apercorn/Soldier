use fake::faker::lorem::en::Word;
use fake::Fake;
use oxid_roblox::models::SkinnyUser;
use poise::serenity_prelude::{self as serenity, UserId};

use crate::entity::user as User;
use crate::{components, embeds, helper, CmdError, CommandContext, CommandResult, Context};
use sea_orm::sea_query::OnConflict;
use sea_orm::ActiveValue::{NotSet, Set};
use sea_orm::{DatabaseConnection, EntityTrait};

#[poise::command(
  slash_command,
  description_localized("en-US", "Links your Discord account to your Roblox account")
)]
pub async fn verify(
  ctx: Context<'_>,
  #[description = "The Roblox username of the account you wish to link"] username: Option<String>,
) -> CommandResult {
  let CommandContext { user, .. } = helper::command_context(&ctx).await?;

  let roblox_user = match username {
    Some(ref u) => oxid_roblox::user_from_username(u).await.ok().flatten(),
    None => None,
  };

  if username.is_some() && roblox_user.is_none() {
    return Err(CmdError::Embed(embeds::error(
      "",
      format!("The user `{}` was not found.", username.unwrap()),
    )));
  }
  println!("{user:#?}");
  // If already verified
  let update_from: Option<serenity::ComponentInteraction> = if let Some(existing) = user {
    let roblox_name = oxid_roblox::user_from_id(existing.roblox_id)
      .await
      .ok()
      .map(|u| u.name)
      .unwrap_or_else(|| existing.roblox_id.to_string());

    helper::send!(ctx,
      .embed(
        embeds::warn("Verification", "You have already verified your account")
        .field("Details", format!("You're already verified as [{}](https://www.roblox.com/users/{}).\nWould you like to verify with a different account?", roblox_name, existing.roblox_id), false)
      )
      .components(vec![components::buttons(vec![
        components::button_danger("verify::reverify", "Verify Again"),
        components::button_secondary("verify::cancel", "Cancel"),
      ])])
      .ephemeral(true)
    )?;

    let Some(mci) = serenity::ComponentInteractionCollector::new(ctx)
      .timeout(std::time::Duration::from_secs(60))
      .message_id(ctx.interaction.get_response(ctx.http()).await?.id)
      .filter(move |mci| mci.data.custom_id.starts_with("verify::"))
      .await
    else {
      return Ok(());
    };

    if mci.data.custom_id.ends_with("cancel") {
      helper::update_msg!(mci, ctx,
        .embed(embeds::cancelled())
        .components(vec![])
      )?;
      return Ok(());
    }

    Some(mci)
  } else {
    None
  };

  // Main verification embed
  let method_embed = embeds::emoji(
    "Verification",
    "To get started using Soldier, you need to connect your Roblox account.",
    crate::emojis::Emoji::Check,
    serenity::Colour::ROHRKATZE_BLUE
  )
  .field("Details", "You can login via the Roblox OAuth2 client or verify manually by placing a randomnly generated phrase in your profile description.", false);

  let method_components = vec![components::buttons(vec![
    components::button_link("https://soldierbot.app/", "Login With Roblox"),
    components::button_secondary("verify::code", "Code Verify").disabled(roblox_user.is_none()),
  ])];

  match update_from {
    Some(mci) => {
      helper::update_msg!(mci, ctx,
        .embed(method_embed)
        .components(method_components)
      )?;
    },
    None => {
      helper::send!(ctx,
        .embed(method_embed)
        .components(method_components)
        .ephemeral(true)
      )?;
    },
  }

  let Some(mci) = serenity::ComponentInteractionCollector::new(ctx)
    .timeout(std::time::Duration::from_secs(60))
    .message_id(ctx.interaction.get_response(ctx.http()).await?.id)
    .filter(move |mci| mci.data.custom_id.starts_with("verify::"))
    .await
  else {
    return Ok(());
  };

  let Some(roblox_user) = roblox_user else {
    return Ok(());
  };

  code_verification(ctx, mci, roblox_user).await
}

async fn code_verification(
  ctx: Context<'_>,
  interaction: serenity::ComponentInteraction,
  player: SkinnyUser,
) -> CommandResult {
  let CommandContext { db, .. } = helper::command_context(&ctx).await?;

  let words = {
    let words_vec: Vec<String> = (0..3).map(|_| Word().fake()).collect();
    words_vec.join(" ")
  };

  helper::update_msg!(interaction, ctx,
    .embed(
      embeds::info("Account Verification", format!("Please put this randomly generated phrase into your [about](https://roblox.com/users/{}) page.", player.id))
        .field("Phrase", format!("If one of the words are tags, please try again.\n```{}```", words), false)
    )
    .components(vec![components::buttons(vec![
      components::button_success("verify::code::done", "Done"),
      components::button_secondary("verify::code::cancel", "Cancel"),
    ])])
    .ephemeral(true)
  )?;

  let Some(mci) = serenity::ComponentInteractionCollector::new(ctx)
    .timeout(std::time::Duration::from_secs(300))
    .filter(move |mci| mci.data.custom_id.starts_with("verify::"))
    .await
  else {
    return Err(CmdError::Timeout);
  };

  // Cancel button
  if mci.data.custom_id.ends_with("cancel") {
    helper::update_msg!(mci, ctx,
      .embed(embeds::cancelled())
    )?;
    return Ok(());
  }

  let phrase_match = {
    let player_info = oxid_roblox::user_from_id(player.id).await?;
    player_info.description.contains(&words)
  };

  if !phrase_match {
    helper::update_msg!(mci, ctx,
      .embed(embeds::error("Verification Failed", "The verification phrase was not found in your Roblox profile."))
      .components(vec![])
    )?;

    return Err(CmdError::Handled);
  }

  complete_verification(db, ctx.author().id, player.id).await?;

  helper::response!(mci, ctx,
    .embed(
      embeds::success("Account Verification", "Your account has been verified successfully!")
        .field("Account", format!("Username: [{}](https://www.roblox.com/users/{})\nDisplay Name: `{}`\nRoblox ID: `{}`", player.name, player.id, player.display_name.as_deref().unwrap_or(&player.name), player.id), false)
    )
  )?;

  Ok(())
}

async fn complete_verification(
  db: &DatabaseConnection,
  discord_id: UserId,
  roblox_id: i64,
) -> CommandResult {
  let user = User::ActiveModel {
    id: NotSet,
    discord_id: Set(discord_id.to_string()),
    roblox_id: Set(roblox_id),

    ..Default::default()
  };

  User::Entity::insert(user)
    .on_conflict(
      OnConflict::column(User::Column::DiscordId)
        .update_column(User::Column::RobloxId)
        .to_owned(),
    )
    .exec(db)
    .await?;

  Ok(())
}
