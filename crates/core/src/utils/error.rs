use oxid_roblox::util::OxidError;
use poise::{serenity_prelude as serenity, CreateReply, FrameworkError};
use thiserror::Error;

use crate::{embeds, helper, Data};



#[derive(Debug, Error)]
pub enum CmdError {
  /// A user initiated error returned from the command, formatted in an embed.
  /// These are custom handled and formatted to the user.
  #[error("Command Error")]
  Command(Result<(), serenity::Error>),

  /// An error originating from a bad Roblox Api call.
  /// This is an unexpected error that dumps to the bug handler.
  #[error("{0}")]
  Roblox(#[from] OxidError),

  /// An error originating from performing an invalid action through the Discord Api.
  /// This is an unexpected error that dumps to the bug handler.
  #[error("Discord Error: {0}")]
  Serenity(#[from] serenity::Error),

  /// An error originating from SeaORM database operations.
  #[error("Database Error: {0}")]
  SeaOrm(#[from] sea_orm::DbErr),

  /// An error returned when a prompt times out from inactivity
  #[error("Prompt timed out due to inactivity")]
  Timeout,

  /// todo
  #[error("On cooldown: {0:?}")]
  OnCooldown(std::time::Duration),

  #[error("Other: {0}")]
  Other(String),

  // unimplemented. not sure where i use this tbh
  #[error(transparent)]
  Anyhow(#[from] anyhow::Error),
}

pub async fn handle_error(fw_error: FrameworkError<'_, Data, CmdError>) {
  match fw_error {
    FrameworkError::Command { error, ctx, .. } => {
      // Get the ApplicationContext struct not the Context enum
      let app_ctx = if let poise::Context::Application(app_ctx) = ctx {
        app_ctx
      } else {
        return; // We don't use prefix commands so we return
      };

      // Collects the result of sending the error message
      let err_response = match &error {
        CmdError::Command(_) => {
          // The error message has already been sent to the user from the command
          Ok(())
        }


        // these are still not 100% reliable
        CmdError::Roblox(rblx) => {
          ctx.send(CreateReply::default()
            .embed(embeds::user_error(error.to_string()))
            .ephemeral(true)
          ).await.map(|_| ())
        }

        CmdError::Timeout => {
          ctx.send(CreateReply::default()
            .embed(embeds::timeout())
            .ephemeral(true)
          ).await.map(|_| ())
        }

        _ => {
          ctx.send(CreateReply::default()
            .content(error.to_string())
            .ephemeral(true),
          )
          .await.map(|_| ())
        },
      };

      if err_response.is_err() {
        eprintln!(
          "Error: Failed to send command error message: {:?}\n\nOriginal Error: {:?}",
          err_response.err(),
          error.to_string()
        );
      }
    },

    other => {
      let _ = poise::builtins::on_error(other).await;
    },
  }
}
