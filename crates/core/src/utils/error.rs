use oxid_roblox::util::OxidError;
use poise::{serenity_prelude as serenity, CreateReply, FrameworkError};
use thiserror::Error;

use crate::{embeds, Data};

#[derive(Debug, Error)]
pub enum CmdError {
  /// The error embed has already been sent to the user directly from within the
  /// command body. The framework error handler does nothing for this variant.
  ///
  /// Pattern:
  /// ```
  /// response!(interaction, ctx, .embed(embeds::error(...)))?;
  /// return Err(CmdError::Handled);
  /// ```
  #[error("Command error")]
  Handled,

  /// An embed to display as the error response. Use this when the error occurs
  /// **before any response has been sent** to the interaction. The framework
  /// error handler will send this embed via `ctx.send()`.
  ///
  /// Do NOT use after `ctx.interaction.quick_modal()` or any direct
  /// `interaction.create_response()` — at that point the token is consumed
  /// and `ctx.send()` will fail. Use `CmdError::Handled` instead.
  ///
  /// Pattern:
  /// ```
  /// return Err(CmdError::Embed(embeds::error("Title", "Something went wrong.")));
  /// ```
  #[error("Command error")]
  Embed(serenity::CreateEmbed),

  /// A bad Roblox API call. Shown to the user as an unexpected error.
  #[error("Roblox API Error: {0}")]
  Roblox(#[from] OxidError),

  /// An error from the Discord API.
  #[error("Discord API Error: {0}")]
  Serenity(#[from] serenity::Error),

  /// A SeaORM database error.
  #[error("Database Error: {0}")]
  SeaOrm(#[from] sea_orm::DbErr),

  /// A user-facing interaction prompt timed out.
  #[error("Prompt timed out due to inactivity")]
  Timeout,

  #[error("On cooldown: {0:?}")]
  OnCooldown(std::time::Duration),

  #[error("Other: {0}")]
  Other(String),

  #[error(transparent)]
  Anyhow(#[from] anyhow::Error),
}

pub async fn handle_error(fw_error: FrameworkError<'_, Data, CmdError>) {
  match fw_error {
    FrameworkError::Command { error, ctx, .. } => {
      // Skip prefix commands (only slash commands are used)
      let poise::Context::Application(_) = ctx else {
        return;
      };

      // Error was already shown to the user from within the command body
      if matches!(error, CmdError::Handled) {
        return;
      }

      let result = match error {
        CmdError::Embed(embed) => ctx
          .send(CreateReply::default().embed(embed).ephemeral(true))
          .await
          .map(|_| ()),

        CmdError::Timeout => ctx
          .send(
            CreateReply::default()
              .embed(embeds::timeout())
              .ephemeral(true),
          )
          .await
          .map(|_| ()),

        CmdError::Roblox(e) => ctx
          .send(
            CreateReply::default()
              .embed(embeds::user_error(e.to_string()))
              .ephemeral(true),
          )
          .await
          .map(|_| ()),

        other => ctx
          .send(
            CreateReply::default()
              .content(other.to_string())
              .ephemeral(true),
          )
          .await
          .map(|_| ()),
      };

      if let Err(e) = result {
        eprintln!("Failed to send error message to user: {e:?}");
      }
    },

    other => {
      let _ = poise::builtins::on_error(other).await;
    },
  }
}
