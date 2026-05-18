use poise::serenity_prelude as serenity;

use crate::{embeds, helper, CmdError, CommandResult, Context};

// ─── Helpers ─────────────────────────────────────────────────────────────────

fn btn(id: &str, label: &str) -> serenity::CreateActionRow {
  serenity::CreateActionRow::Buttons(vec![
    serenity::CreateButton::new(id)
      .label(label)
      .style(serenity::ButtonStyle::Primary),
  ])
}

async fn wait_click(
  ctx: Context<'_>,
  msg_id: serenity::MessageId,
) -> Option<serenity::ComponentInteraction> {
  serenity::ComponentInteractionCollector::new(ctx)
    .timeout(std::time::Duration::from_secs(30))
    .message_id(msg_id)
    .await
}

// ─── Parent ───────────────────────────────────────────────────────────────────

#[poise::command(
  slash_command,
  description_localized("en-US", "Tests each interaction response scenario"),
  subcommands(
    "test_send",
    "test_modal",
    "test_component_update",
    "test_component_response",
    "test_component_ack",
    "test_followup",
    "test_any_direct",
    "test_any_component",
    "test_err_embed",
    "test_err_handled",
    "test_err_timeout",
    "test_err_system",
    "test_err_after_response",
  )
)]
pub async fn test(_: Context<'_>) -> CommandResult {
  Ok(())
}

// ─── Response scenarios ───────────────────────────────────────────────────────

/// helper::send!(ctx, ...) — initial response routed through poise
#[poise::command(slash_command, rename = "send", description_localized("en-US", "helper::send!(ctx, ...) — initial response via poise"))]
async fn test_send(ctx: Context<'_>) -> CommandResult {
  helper::send!(ctx,
    .embed(embeds::success("send!", "Initial response via `helper::send!(ctx, ...)`."))
    .ephemeral(true)
  )?;

  Ok(())
}

/// quick_modal → helper::response!(modal_res.interaction, ctx, ...)
#[poise::command(slash_command, rename = "modal", description_localized("en-US", "quick_modal → helper::response!(modal_res.interaction, ctx, ...)"))]
async fn test_modal(ctx: Context<'_>) -> CommandResult {
  let res = ctx.interaction
    .quick_modal(
      ctx.serenity_context(),
      serenity::CreateQuickModal::new("Test Modal")
        .timeout(std::time::Duration::from_secs(60))
        .field(
          serenity::CreateInputText::new(
            serenity::InputTextStyle::Short,
            "Your input",
            "test::modal::input",
          )
          .required(true),
        ),
    )
    .await?
    .ok_or(CmdError::Timeout)?;

  let value = res.inputs.first().unwrap();

  helper::response!(res.interaction, ctx,
    .embed(embeds::success("modal", format!("Submitted. Input: `{value}`")))
  )?;

  Ok(())
}

/// helper::update_msg!(mci, ctx) — component click updates the existing message in-place
#[poise::command(slash_command, rename = "component-update", description_localized("en-US", "helper::update_msg!(mci, ctx) — component click updates the existing message"))]
async fn test_component_update(ctx: Context<'_>) -> CommandResult {
  helper::response!(ctx.interaction, ctx,
    .embed(embeds::info("component-update", "Click the button. The message will be updated in-place."))
    .components(vec![btn("test::update", "Update message")])
  )?;

  let msg_id = ctx.interaction.get_response(ctx.http()).await?.id;

  let Some(mci) = wait_click(ctx, msg_id).await else {
    return Err(CmdError::Timeout);
  };

  helper::update_msg!(mci, ctx,
    .embed(embeds::success("component-update", "Updated in-place via `helper::update_msg!(mci, ctx)`."))
  )?;

  Ok(())
}

/// helper::response!(mci, ctx) — component click sends a new ephemeral message
#[poise::command(slash_command, rename = "component-response", description_localized("en-US", "helper::response!(mci, ctx) — component click sends a new ephemeral message"))]
async fn test_component_response(ctx: Context<'_>) -> CommandResult {
  helper::response!(ctx.interaction, ctx,
    .embed(embeds::info("component-response", "Click the button. A new ephemeral message will appear."))
    .components(vec![btn("test::response", "New message")])
  )?;

  let msg_id = ctx.interaction.get_response(ctx.http()).await?.id;

  let Some(mci) = wait_click(ctx, msg_id).await else {
    return Err(CmdError::Timeout);
  };

  helper::response!(mci, ctx,
    .embed(embeds::success("component-response", "New message via `helper::response!(mci, ctx)`."))
  )?;

  Ok(())
}

/// helper::acknowledge!(mci, ctx) — component click silently acknowledged
#[poise::command(slash_command, rename = "component-ack", description_localized("en-US", "helper::acknowledge!(mci, ctx) — silently acknowledges a component click"))]
async fn test_component_ack(ctx: Context<'_>) -> CommandResult {
  helper::response!(ctx.interaction, ctx,
    .embed(embeds::info("component-ack", "Click the button. It will be silently acknowledged — no loading spinner, no new message."))
    .components(vec![btn("test::ack", "Acknowledge")])
  )?;

  let msg_id = ctx.interaction.get_response(ctx.http()).await?.id;

  let Some(mci) = wait_click(ctx, msg_id).await else {
    return Err(CmdError::Timeout);
  };

  helper::acknowledge!(mci, ctx);

  Ok(())
}

/// send! for initial response, then helper::followup!(ctx.interaction, ctx)
#[poise::command(slash_command, rename = "followup", description_localized("en-US", "send! then helper::followup!(ctx.interaction, ctx) — sends a second message"))]
async fn test_followup(ctx: Context<'_>) -> CommandResult {
  helper::send!(ctx,
    .embed(embeds::info("followup", "Initial response via `send!`. A followup message will appear below."))
    .ephemeral(true)
  )?;

  helper::followup!(ctx.interaction, ctx,
    .embed(embeds::success("followup", "Followup via `helper::followup!(ctx.interaction, ctx)`."))
  );

  Ok(())
}

/// any_update! called directly from the slash command (AnyInteraction::Command path)
#[poise::command(slash_command, rename = "any-direct", description_localized("en-US", "AnyInteraction::Command path — any_update! from the slash command directly"))]
async fn test_any_direct(ctx: Context<'_>) -> CommandResult {
  _test_any_fn(ctx, helper::AnyInteraction::Command(ctx.interaction.clone())).await
}

/// any_update! reached via a button click (AnyInteraction::Component path)
#[poise::command(slash_command, rename = "any-component", description_localized("en-US", "AnyInteraction::Component path — any_update! reached via a component click"))]
async fn test_any_component(ctx: Context<'_>) -> CommandResult {
  helper::response!(ctx.interaction, ctx,
    .embed(embeds::info("any-component", "Click the button to reach the shared function via `AnyInteraction::Component`."))
    .components(vec![btn("test::any", "Go to shared function")])
  )?;

  let msg_id = ctx.interaction.get_response(ctx.http()).await?.id;

  let Some(mci) = wait_click(ctx, msg_id).await else {
    return Err(CmdError::Timeout);
  };

  _test_any_fn(ctx, helper::AnyInteraction::Component(mci)).await
}

async fn _test_any_fn(ctx: Context<'_>, interaction: helper::AnyInteraction) -> CommandResult {
  helper::any_update!(interaction, ctx,
    .embed(embeds::success(
      "any_update!",
      "Reached via `_test_any_fn()`. Works identically from both `AnyInteraction::Command` and `AnyInteraction::Component`.",
    ))
  )?;

  Ok(())
}

// ─── Error scenarios ──────────────────────────────────────────────────────────

/// CmdError::Embed — error handler sends the embed as the initial response via ctx.send()
#[poise::command(slash_command, rename = "err-embed", description_localized("en-US", "CmdError::Embed — error handler sends the embed as the initial response"))]
async fn test_err_embed(_: Context<'_>) -> CommandResult {
  Err(CmdError::Embed(embeds::error(
    "err-embed",
    "`CmdError::Embed`: sent by the error handler via `ctx.send()`. No prior response was made.",
  )))
}

/// CmdError::Handled — error embed sent manually, error handler does nothing
#[poise::command(slash_command, rename = "err-handled", description_localized("en-US", "CmdError::Handled — error embed sent manually, handler does nothing"))]
async fn test_err_handled(ctx: Context<'_>) -> CommandResult {
  helper::response!(ctx.interaction, ctx,
    .embed(embeds::error(
      "err-handled",
      "Sent manually via `helper::response!()`. `CmdError::Handled` tells the error handler to do nothing — you should see only this one embed.",
    ))
  )?;

  Err(CmdError::Handled)
}

/// CmdError::Timeout — standard timeout embed sent by the error handler
#[poise::command(slash_command, rename = "err-timeout", description_localized("en-US", "CmdError::Timeout — standard timeout embed from the error handler"))]
async fn test_err_timeout(_: Context<'_>) -> CommandResult {
  Err(CmdError::Timeout)
}

/// CmdError::Other — unhandled error formatted as plain content by the error handler
#[poise::command(slash_command, rename = "err-system", description_localized("en-US", "CmdError::Other — unhandled error, formatted as plain content by the error handler"))]
async fn test_err_system(_: Context<'_>) -> CommandResult {
  Err(CmdError::Other(
    "Simulated system error. The error handler should display this as plain content.".into(),
  ))
}

/// Unhandled error after helper::response!() — error handler must use followup
// (verifies the has_sent_initial_response sync fix in helper::response!())
#[poise::command(slash_command, rename = "err-after-response", description_localized("en-US", "Unhandled error after helper::response!() — error handler must send it as a followup"))]
async fn test_err_after_response(ctx: Context<'_>) -> CommandResult {
  helper::response!(ctx.interaction, ctx,
    .embed(embeds::info(
      "err-after-response",
      "Initial `helper::response!()` sent. Propagating an unhandled error — the error handler must use `create_followup`, not `create_response`.",
    ))
  )?;

  // Simulate an unexpected error after the initial response was already consumed.
  // The helper::response!() macro syncs poise's has_sent_initial_response flag, so
  // ctx.send() in the error handler correctly issues a followup instead of
  // trying create_response again (which would fail with Discord error 40060).
  Err(CmdError::Other(
    "Error after helper::response!(). You should see this as a second ephemeral message — a followup.".into(),
  ))
}
