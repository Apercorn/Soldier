use once_cell::sync::Lazy;
use std::collections::HashMap;

pub static EMOJIS: Lazy<HashMap<Emoji, &'static str>> = Lazy::new(|| {
  use Emoji::*;
  let mut m = HashMap::new();

  m.insert(Check, "<:Check:867931890437476353>");
  m.insert(Error, "<:Error:868113159737720912>");
  m.insert(Warn, "<:Warn:868113114221121586>");
  m.insert(Info, "<:Info:1053145199167164479>");
  m.insert(Dash, "<:greendash:903084370066300928>");
  m.insert(Tick, "<:Tick:868113233981100123>");
  m.insert(Cross, "<:X_:868113200934174801>");
  m
});

#[derive(Eq, Hash, PartialEq)]
pub enum Emoji {
  Check,
  Error,
  Warn,
  Dash,
  Info,
  Tick,
  Cross
}


pub fn use_emoji(emoji: Emoji) -> String {
  EMOJIS.get(&emoji).unwrap_or(&"").to_string()
}