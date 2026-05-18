use poise::serenity_prelude as serenity;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::entity::{clan as Clan, user as User};
use crate::{CmdError, CommandContext, Context};

// todo: make a bitfield permission struct which holds conditions that must be true for the command to execute
//       otherwise it returns an error
pub async fn command_context<'a>(ctx: &'a Context<'_>) -> Result<CommandContext<'a>, CmdError> {
  let data = ctx.data();
  let db = &data.db;
  let rankers = &data.rankers;

  let guild_id = ctx.guild().unwrap().id.to_string();
  let author_id = ctx.author().id.to_string();

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

  Ok(CommandContext {
    db,
    clan,
    user,
    rankers,
  })
}

/// Sends a response or followup on an interaction.
/// NOTE: This is not to be used on `InteractionType::Modal`
///
/// ## Examples
/// ```no_run
/// # Creates a response or followup
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
/// # Create a response that sends a new message
/// response!(interaction, ctx,
///   .content("Hello!")
///   .ephemeral(true)
/// );
/// ```
#[macro_export]
macro_rules! response {
  ($interaction:expr, $ctx:expr, $($body:tt)*) => {{
    let result = $interaction.create_response(
      &$ctx.http(),
      serenity::CreateInteractionResponse::Message(
        serenity::CreateInteractionResponseMessage::default()
          .ephemeral(true)
          $($body)*
      )
    ).await;
    if result.is_ok() {
      $ctx.has_sent_initial_response.store(true, ::std::sync::atomic::Ordering::SeqCst);
    }
    result
  }};
}

/// Performs the initial response to an interaction by updating the original message.
///
/// ## Examples
/// ```no_run
/// # Create a response that updates the original message
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
/// # Send a followup to the interaction
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
/// # Send an acklowledgement to the interaction
/// acknowledge!(interaction, ctx);
/// ```
#[macro_export]
macro_rules! acknowledge {
  ($interaction:expr, $ctx:expr) => {
    $interaction
      .create_response(&$ctx.http(), serenity::CreateInteractionResponse::Acknowledge)
      .await?
  };
}

/// Responds to an `AnyInteraction` with a new ephemeral message.
/// Equivalent to `response!` but for `AnyInteraction`.
#[macro_export]
macro_rules! any_response {
  ($interaction:expr, $ctx:expr, $($body:tt)*) => {{
    let result = $interaction.respond(
      &$ctx.http(),
      serenity::CreateInteractionResponseMessage::default()
        .ephemeral(true)
        $($body)*
    ).await;
    if result.is_ok() {
      $ctx.has_sent_initial_response.store(true, ::std::sync::atomic::Ordering::SeqCst);
    }
    result
  }};
}

/// Updates the message for an `AnyInteraction`.
/// For component interactions, updates the original message.
/// For command/modal interactions, sends a new message.
/// Clears components by default; override with `.components(vec![...])` in the body.
/// Equivalent to `update_msg!` but for `AnyInteraction`.
#[macro_export]
macro_rules! any_update {
  ($interaction:expr, $ctx:expr, $($body:tt)*) => {{
    let result = $interaction.update_message(
      &$ctx.http(),
      serenity::CreateInteractionResponseMessage::default()
        .components(vec![])
        .ephemeral(true)
        $($body)*
    ).await;
    if result.is_ok() {
      $ctx.has_sent_initial_response.store(true, ::std::sync::atomic::Ordering::SeqCst);
    }
    result
  }};
}

/// Sends a followup message to an `AnyInteraction`.
/// Equivalent to `followup!` but for `AnyInteraction`.
#[macro_export]
macro_rules! any_followup {
  ($interaction:expr, $ctx:expr, $($body:tt)*) => {
    $interaction.followup(
      &$ctx.http(),
      serenity::CreateInteractionResponseFollowup::default()
        .ephemeral(true)
        $($body)*
    ).await?
  };
}

/// Acknowledges an `AnyInteraction` with no visible response.
/// Only meaningful for component interactions.
/// Equivalent to `acknowledge!` but for `AnyInteraction`.
#[macro_export]
macro_rules! any_acknowledge {
  ($interaction:expr, $ctx:expr) => {
    $interaction.acknowledge(&$ctx.http()).await?
  };
}

pub use crate::{
  acknowledge, any_acknowledge, any_followup, any_response, any_update, followup, response, send,
  update_msg,
};

/// A unified wrapper over the three Discord interaction types.
///
/// Avoids the need to match on `serenity::Interaction` at every call site when
/// a function is reachable from both a slash command and a component collector.
///
/// ## Construction
/// ```no_run
/// // From a slash command context:
/// let i = helper::AnyInteraction::Command(ctx.interaction.clone());
///
/// // From a component collector:
/// let i = helper::AnyInteraction::Component(mci);
///
/// // From a modal submission:
/// let i = helper::AnyInteraction::Modal(modal_res.interaction);
///
/// // From a raw serenity::Interaction:
/// let i = helper::AnyInteraction::from(raw);
/// ```
pub enum AnyInteraction {
  Command(serenity::CommandInteraction),
  Component(serenity::ComponentInteraction),
  Modal(serenity::ModalInteraction),
}

impl AnyInteraction {
  /// Sends a new ephemeral message in response to this interaction.
  /// Valid as the first response for any interaction type.
  /// For component interactions this creates a *new* message rather than
  /// updating the component's parent message — use `update_message` for that.
  pub async fn respond(
    &self,
    http: impl serenity::CacheHttp,
    msg: serenity::CreateInteractionResponseMessage,
  ) -> serenity::Result<()> {
    let r = serenity::CreateInteractionResponse::Message(msg);
    match self {
      Self::Command(i) => i.create_response(http, r).await,
      Self::Component(i) => i.create_response(http, r).await,
      Self::Modal(i) => i.create_response(http, r).await,
    }
  }

  /// Updates the component's parent message for `Component` interactions,
  /// or sends a new message for `Command` and `Modal` interactions.
  ///
  /// This is the method to use for functions reachable from both a slash
  /// command and a component collector, since it does the contextually
  /// correct thing for each interaction type.
  ///
  /// Pass `.components(vec![])` in `msg` if you want to clear existing buttons
  /// or menus — this method does not do so automatically.
  pub async fn update_message(
    &self,
    http: impl serenity::CacheHttp,
    msg: serenity::CreateInteractionResponseMessage,
  ) -> serenity::Result<()> {
    match self {
      Self::Component(i) => {
        i.create_response(http, serenity::CreateInteractionResponse::UpdateMessage(msg))
          .await
      },
      Self::Command(i) => {
        i.create_response(http, serenity::CreateInteractionResponse::Message(msg))
          .await
      },
      Self::Modal(i) => {
        i.create_response(http, serenity::CreateInteractionResponse::Message(msg))
          .await
      },
    }
  }

  /// Sends a followup message. The interaction must already have been responded to.
  pub async fn followup(
    &self,
    http: impl serenity::CacheHttp,
    data: serenity::CreateInteractionResponseFollowup,
  ) -> serenity::Result<serenity::Message> {
    match self {
      Self::Command(i) => i.create_followup(http, data).await,
      Self::Component(i) => i.create_followup(http, data).await,
      Self::Modal(i) => i.create_followup(http, data).await,
    }
  }

  /// Silently acknowledges the interaction with no visible response.
  /// Only meaningful for `Component` interactions; other types are a no-op.
  pub async fn acknowledge(&self, http: impl serenity::CacheHttp) -> serenity::Result<()> {
    match self {
      Self::Component(i) => {
        i.create_response(http, serenity::CreateInteractionResponse::Acknowledge)
          .await
      },
      _ => Ok(()),
    }
  }
}

impl From<serenity::Interaction> for AnyInteraction {
  fn from(i: serenity::Interaction) -> Self {
    match i {
      serenity::Interaction::Command(i) => Self::Command(i),
      serenity::Interaction::Component(i) => Self::Component(i),
      serenity::Interaction::Modal(i) => Self::Modal(i),
      _ => panic!("Unsupported interaction variant for AnyInteraction"),
    }
  }
}
