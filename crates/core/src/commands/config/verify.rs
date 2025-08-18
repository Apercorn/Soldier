use fake::faker::lorem::en::Word;
use fake::Fake;
use oxid_roblox::models::SkinnyUser;
use poise::serenity_prelude as serenity;

use crate::utils::{embeds, helper};
use crate::{response, send, update_msg, CmdError, CommandResult, Context};

#[poise::command(
  slash_command,
  description_localized("en-US", "Links your Discord account to your Roblox account")
)]
pub async fn verify(
  ctx: Context<'_>,
  #[description = "The Roblox username of the account you wish to link"] username: Option<String>,
) -> CommandResult {
  // 1. display choice embed [oauth2, legacy (code verification)]
  // 2. check username's roblox account, error if its already connected or doesnt exist
  // 3. if legacy verification, then pass to code_verification

  let roblox_user = match username {
    Some(ref u) => oxid_roblox::user_from_username(&u).await.ok().flatten(),
    None => None,
  };

  // If the user provided wasn't found
  if username.is_some() && roblox_user.is_none() {
    return Err(CmdError::Embed(embeds::error(
      "",
      format!("The user `{}` was not found.", username.unwrap()),
    )));
  }

  // Send main embed
  send!(ctx,
    .embed(embeds::info("Verification", "todo text"))
    .components(vec![serenity::CreateActionRow::Buttons(
      vec![
        serenity::CreateButton::new_link("https://soldierbot.app/").label("Oauth2"),
        serenity::CreateButton::new("verify::code").label("Code").style(serenity::ButtonStyle::Secondary).disabled(roblox_user.is_none())
      ]
    )])
    .ephemeral(true)
  );

  if let Some(mci) = serenity::ComponentInteractionCollector::new(ctx)
    .timeout(std::time::Duration::from_secs(60))
    .filter(move |mci| mci.data.custom_id.starts_with("verify::"))
    .await
  {
    code_verification(ctx, mci, roblox_user.unwrap()).await?;
  }

  Ok(())
}

async fn code_verification(
  ctx: Context<'_>,
  interaction: serenity::ComponentInteraction,
  player: SkinnyUser,
) -> CommandResult {
  let helper::CommandContext { db, clan, user } = helper::command_context(&ctx).await;

  // 1. generate a random set of words
  // 2. send a formatted embed with the verification phrase and a [done] button
  // 3. on done:click check if the blurb contains the phrase

  let words = {
    let words_vec: Vec<String> = (0..3).map(|_| Word().fake()).collect();
    words_vec.join(" ")
  };

  update_msg!(interaction, ctx,
    .embed(
      embeds::info("Account Verification", format!("Please put this randomly generated phrase into your [about](https://roblox.com/users/{}) page.", player.id))
      .field("Phrase", format!("If one of the words are tags, please try again.\n```{}```", words), false)
    )
    .button(serenity::CreateButton::new("verify::code::done").label("Done").style(serenity::ButtonStyle::Secondary))
    .ephemeral(true)
  );

  if let Some(mci) = serenity::ComponentInteractionCollector::new(ctx)
    .timeout(std::time::Duration::from_secs(10))
    .filter(move |mci| mci.data.custom_id.starts_with("verify::"))
    .await
  {
    let phrase_match = {
      let player_info = oxid_roblox::user_from_id(player.id).await?;
      player_info.description.contains(&words)
    };

    if !phrase_match {
      return Err(CmdError::Embed(embeds::error(
        "Verification Failed",
        "The verification phrase was not found in your Roblox profile.",
      )));
    }

    response!(mci, ctx,
      .embed(
        embeds::success("Account Verification", "Your account has been verified successfully!")
          .field("Account", format!("Username: [{}](https://www.roblox.com/users/{})\nDisplay Name: `{}`\nRoblox ID: `{}`", player.name, player.id, player.display_name.as_deref().unwrap_or(&player.name), player.id), false)
      )
      .ephemeral(true)
    )
  } else {
    return Err(CmdError::Timeout);
  }

  Ok(())
}

async fn complete_verification() {
  todo!()
}
