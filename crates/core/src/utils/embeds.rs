use ::serenity::all::{Color, CreateEmbed};
use crate::utils::emojis::*;

pub fn emoji(title: impl Into<String>, description: impl Into<String>, emoji: Emoji, color: Color) -> CreateEmbed {
  let emoji_string = *EMOJIS.get(&emoji).unwrap_or(&"");

  CreateEmbed::new()
    .title(title)
    .description([emoji_string, "\u{2007}", &description.into()].concat())
    .color(color)
}

pub fn success(title: impl Into<String>, description: impl Into<String>) -> CreateEmbed {
  self::emoji(title, description, Emoji::Check, Color::DARK_GREEN)
}

pub fn warn(title: impl Into<String>, description: impl Into<String>) -> CreateEmbed {
  self::emoji(title, description, Emoji::Warn, Color::DARK_ORANGE)
}

pub fn error(title: impl Into<String>, description: impl Into<String>) -> CreateEmbed {
  self::emoji(title, description, Emoji::Error, Color::DARK_RED)
}

pub fn info(title: impl Into<String>, description: impl Into<String>) -> CreateEmbed {
  self::emoji(title, description, Emoji::Error, Color::DARK_RED)
}