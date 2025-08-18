use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::{CmdError, Context, CommandContext};
use crate::entity::{clan as Clan, user as User};

// todo: make a bitfield permission struct which holds conditions that must be true for the command to execute
//       otherwise it returns an error
pub async fn command_context<'a>(ctx: &'a Context<'_>) -> Result<CommandContext<'a>, CmdError> {
  let data = ctx.data();
  let db = &data.db;
  let rankers = &data.rankers;

  let guild_id = ctx.guild().unwrap().id.get();
  let author_id = ctx.author().id.get();

  let clan = Clan::Entity::find()
    .filter(Clan::Column::DiscordGuildId.eq(guild_id))
    .one(db)
    .await
    .ok()
    .flatten();

  let user = User::Entity::find()
    .filter(User::Column::DiscordId.eq(author_id))
    .one(db)
    .await
    .ok()
    .flatten();

  let guild = ctx.guild().unwrap();

  Ok(
    CommandContext {
      db,
      clan,
      user,
      rankers
    }
  )
}

/// Sends a response or followup on an interaction.
/// NOTE: This is not to be used on `InteractionType::Modal`
///
/// ## Examples
/// ```no_run
/// // 1. Creates a response or followup
/// send!(ctx,
///   .content("Hello!")
///   .ephemeral(true)
/// );
/// ```
#[macro_export]
macro_rules! send {
  ($ctx:expr, $($body:tt)*) => {
    $ctx.send(
      poise::CreateReply::default()
        $($body)*
    ).await
  };
}

/// Performs the initial response to an interaction by responding to it with a new message.
///
/// ## Examples
/// ```no_run
/// // 1. Create a response that sends a new message
/// response!(interaction, ctx,
///   .content("Hello!")
///   .ephemeral(true)
/// );
/// ```
#[macro_export]
macro_rules! response {
  ($interaction:expr, $ctx:expr, $($body:tt)*) => {
    $interaction.create_response(
      &$ctx.http(),
      serenity::CreateInteractionResponse::Message(
        serenity::CreateInteractionResponseMessage::default()
          .ephemeral(true)
          $($body)*
      )
    ).await
  };
}

/// Performs the initial response to an interaction by updating the original message.
///
/// ## Examples
/// ```no_run
/// // 1. Create a response that updates the original message
/// update_msg!(interaction, ctx,
///   .content("Hello again!")
///   .ephemeral(true)
/// );
/// ```
#[macro_export]
macro_rules! update_msg {
  ($interaction:expr, $ctx:expr, $($body:tt)*) => {
    $interaction.create_response(
      &$ctx.http(),
      serenity::CreateInteractionResponse::UpdateMessage(
        serenity::CreateInteractionResponseMessage::default()
          .components(vec![])
          $($body)*
      )
    ).await
  };
}

/// Sends a followup message to an interaction.<br>
/// Followups can be sent to the interaction during the 15 minute interaction lifetime<br>
/// assuming that the interaction has already been replied to.
///
/// ## Example
/// ```no_run
/// // 1. Send a followup to the interaction
/// followup!(interaction, ctx,
///   .content("Followup message!")
///   .ephemeral(true)
/// );
/// ```
#[macro_export]
macro_rules! followup {
  ($interaction:expr, $ctx:expr, $($body:tt)*) => {
    $interaction.create_followup(
      &$ctx.http(),
      serenity::CreateInteractionResponseFollowup::default()
        .ephemeral(true)
        $($body)*
    ).await?
  }
}

/// Acknowledges the interaction. The user does not see a loading state.<br>
/// Only valid for component-based interactions
///
/// ## Example
/// ```no_run
/// // 1. Send an acklowledgement to the interaction
/// acknowledge!(interaction, ctx);
/// ```
#[macro_export]
macro_rules! acknowledge {
  ($interaction:expr, $ctx:expr) => {
    $interaction.create_response(
      &$ctx.http(),
      serenity::CreateInteractionResponse::Acknowledge
    ).await?
  }
}

/// this is kinda attrocious
#[macro_export]
macro_rules! multi_response {
  // Matches .embed
  ($interaction:expr, $ctx:expr, $emb:expr, { $($variant:ident => $reply_macro:ident),+ $(,)? }) => {
    match $interaction {
      $(
        serenity::Interaction::$variant(i) => $reply_macro!(i, $ctx,
          .embed($emb)
        ),
      )+
      _ => panic!("Unexpected interaction type")
    }
  };

  // Matches .embed + .components
  ($interaction:expr, $ctx:expr, $emb:expr, $comp:expr, { $($variant:ident => $reply_macro:ident),+ $(,)? }) => {
    match $interaction {
      $(
        serenity::Interaction::$variant(i) => $reply_macro!(i, $ctx,
          .embed($emb)
          .components($comp)
        ),
      )+
      _ => panic!("Unexpected interaction type")
    }
  };
}



// -------------------------------------

// #[macro_export]
// macro_rules! match_interaction {
//   ($interaction:expr, $body:tt) => {
//     match $interaction {
//       serenity::Interaction::Command(i) => $body,
//       serenity::Interaction::Component(i) => $body,
//       serenity::Interaction::Modal(i) => $body,
//       _ => Err(serenity::Error::Other("Unknown interaction variant")),
//     }
//   };
// }

// #[derive(Debug)]
// pub enum ResponseType {
//   Response,
//   Followup,
//   UpdateMsg,
// }

// /// Deprecated
// /// 
// /// ```no_run
// /// CmdError::Embed(Handle {
// ///   embed: embeds::not_found("User", Some(username)),
// ///   interaction: serenity::Interaction::Modal(modal_res.interaction),
// ///   response_type: helper::ResponseType::Response
// /// })
// /// ```
// #[derive(Debug)]
// struct Handle {
//   pub interaction: serenity::Interaction,
//   pub embed: serenity::CreateEmbed,
//   pub response_type: ResponseType,
// }

// impl Handle {
//   pub async fn respond(&self, ctx: impl serenity::CacheHttp) -> serenity::Result<()> {
//     match self.response_type {
//       ResponseType::Response => {
//         match &self.interaction {
//           serenity::Interaction::Command(i) => Ok(response!(i, ctx,
//             .embed(self.embed.clone())
//             .ephemeral(true)
//           )),
//           serenity::Interaction::Component(i) => Ok(response!(i, ctx,
//             .embed(self.embed.clone())
//             .ephemeral(true)
//           )),
//           serenity::Interaction::Modal(i) => Ok(response!(i, ctx,
//             .embed(self.embed.clone())
//             .ephemeral(true)
//           )),
//           _ => Err(serenity::Error::Other("Unknown interaction variant")),
//         }
//       }

//       ResponseType::Followup => {
//         match &self.interaction {
//           serenity::Interaction::Command(i) => {
//             followup!(i, ctx,
//               .embed(self.embed.clone())
//               .ephemeral(true)
//             );
//             Ok(())
//           }
//           serenity::Interaction::Component(i) => {
//             followup!(i, ctx,
//               .embed(self.embed.clone())
//               .ephemeral(true)
//             );
//             Ok(())
//           }
//           serenity::Interaction::Modal(i) => {
//             followup!(i, ctx,
//               .embed(self.embed.clone())
//               .ephemeral(true)
//             );
//             Ok(())
//           }
//           _ => Err(serenity::Error::Other("Unknown interaction variant")),
//         }
//       }

//       ResponseType::UpdateMsg => {
//         match &self.interaction {
//           serenity::Interaction::Command(i) => Ok(update_msg!(i, ctx,
//             .embed(self.embed.clone())
//             .ephemeral(true)
//           )),
//           serenity::Interaction::Component(i) => Ok(update_msg!(i, ctx,
//             .embed(self.embed.clone())
//             .ephemeral(true)
//           )),
//           serenity::Interaction::Modal(i) => Ok(update_msg!(i, ctx,
//             .embed(self.embed.clone())
//             .ephemeral(true)
//           )),
//           _ => Err(serenity::Error::Other("Unknown interaction variant")),
//         }
//       }
//     }
//   }
// }

