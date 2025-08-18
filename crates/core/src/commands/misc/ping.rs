use oxid_roblox::derives::GroupDerive;
use poise::{serenity_prelude as serenity, CreateReply};

use crate::{embeds, followup, send, CommandResult, Context};

#[poise::command(
  slash_command,
  description_localized("en-US", "Checks the connection status of the bot")
)]
pub async fn ping(ctx: Context<'_>) -> CommandResult {
  // let handle = ctx.send(CreateReply::default().content("content")).await?;

  // tokio::time::sleep(std::time::Duration::from_secs(1)).await;

  // let _ = handle.delete(poise::Context::Application(ctx)).await;

  // tokio::time::sleep(std::time::Duration::from_secs(1)).await;

  // let _ = ctx.send(CreateReply::default().content("content")).await;

  let create_modal = serenity::CreateQuickModal::new("Create Account")
    .timeout(std::time::Duration::from_secs(300))
    .field(
      serenity::CreateInputText::new(
        serenity::InputTextStyle::Paragraph,
        "Security Cookie",
        "accounts::create::cookie",
      )
      .placeholder("The .ROBLOSECURITY cookie of the account")
      .required(true),
    );

  let modal_res = ctx.interaction.quick_modal(ctx.serenity_context(), create_modal)
    .await?/* .ok_or(cmd_err!(Command, ctx.interaction, Timeout))?; */;

  // send!(ctx,
  //   .embed(embeds::success("Ping", "pong"))
  // );

  followup!(ctx.interaction, ctx,
    .content("h")
  );

  // let btn = serenity::CreateButton::new("hey").label("hii");
  // let _ = modal_res.unwrap().interaction.create_response(&ctx.http(), serenity::CreateInteractionResponse::Message(serenity::CreateInteractionResponseMessage::default().button(btn))).await?;



  // if let Some(mci) = serenity::ComponentInteractionCollector::new(ctx).await {
  //   crate::update_msg!(mci, ctx,
  //     .content("Hello again")
  //     .ephemeral(true)
  //   );
  // }

  Ok(())
}
