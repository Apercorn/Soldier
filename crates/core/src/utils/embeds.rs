use crate::utils::emojis::*;
use poise::serenity_prelude::{Color, CreateEmbed};

pub fn emoji(
  title: impl Into<String>,
  description: impl Into<String>,
  emoji: Emoji,
  color: Color,
) -> CreateEmbed {
  let emoji_string = use_emoji(emoji);

  CreateEmbed::new()
    .title(title)
    .description([emoji_string, "\u{2007}".to_string(), description.into()].concat())
    .color(color)
}

pub fn success(title: impl Into<String>, description: impl Into<String>) -> CreateEmbed {
  self::emoji(title, description, Emoji::Check, Color::DARK_GREEN)
}

pub fn warn(title: impl Into<String>, description: impl Into<String>) -> CreateEmbed {
  self::emoji(title, description, Emoji::Warn, Color::ORANGE)
}

pub fn error(title: impl Into<String>, description: impl Into<String>) -> CreateEmbed {
  self::emoji(title, description, Emoji::Error, Color::DARK_RED)
}

pub fn info(title: impl Into<String>, description: impl Into<String>) -> CreateEmbed {
  self::emoji(title, description, Emoji::Info, Color::BLURPLE)
}

// ---------------------------------------

pub fn not_verified() -> CreateEmbed {
  self::error("", "You are not verified. Run </verify:> to continue.")
}

pub fn timeout() -> CreateEmbed {
  self::warn("", "This prompt has timed out due to inactivity.")
}

pub fn cancelled() -> CreateEmbed {
  self::warn("", "You cancelled the operation.")
}

pub fn not_found(item: &str, query: Option<&str>) -> CreateEmbed {
  self::error(
    format!("{item} Not Found"),
    match query {
      Some(q) => format!("The {} → `{q}` was not found", item.to_lowercase()),
      None => format!("The {item} was not found."),
    },
  )
}

pub fn user_error(raw_error: String) -> CreateEmbed {
  self::error(
    "Error",
    "An unexpected error occured while executing the command.",
  )
  .field("Raw Error", format!("{raw_error}"), false)
  .field(
    "Error Code",
    "The provided error code can be given to the support team.\n```todo```",
    false,
  )
}
