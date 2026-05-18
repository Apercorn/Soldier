use poise::serenity_prelude::{
  ButtonStyle, CreateActionRow, CreateButton, CreateInputText, CreateSelectMenu,
  CreateSelectMenuKind, CreateSelectMenuOption, InputTextStyle,
};

// ─── Buttons ─────────────────────────────────────────────────────────────────

pub fn button(id: impl Into<String>, label: impl Into<String>) -> CreateButton {
  CreateButton::new(id).label(label).style(ButtonStyle::Primary)
}

pub fn button_secondary(id: impl Into<String>, label: impl Into<String>) -> CreateButton {
  CreateButton::new(id).label(label).style(ButtonStyle::Secondary)
}

pub fn button_success(id: impl Into<String>, label: impl Into<String>) -> CreateButton {
  CreateButton::new(id).label(label).style(ButtonStyle::Success)
}

pub fn button_danger(id: impl Into<String>, label: impl Into<String>) -> CreateButton {
  CreateButton::new(id).label(label).style(ButtonStyle::Danger)
}

pub fn button_link(url: impl Into<String>, label: impl Into<String>) -> CreateButton {
  CreateButton::new_link(url).label(label)
}

// ─── Action rows ─────────────────────────────────────────────────────────────

pub fn buttons(buttons: Vec<CreateButton>) -> CreateActionRow {
  CreateActionRow::Buttons(buttons)
}

pub fn select_row(
  id: impl Into<String>,
  options: Vec<CreateSelectMenuOption>,
) -> CreateActionRow {
  CreateActionRow::SelectMenu(select(id, options))
}

// ─── Select menus ─────────────────────────────────────────────────────────────

pub fn select(
  id: impl Into<String>,
  options: Vec<CreateSelectMenuOption>,
) -> CreateSelectMenu {
  CreateSelectMenu::new(id, CreateSelectMenuKind::String { options })
}

pub fn option(label: impl Into<String>, value: impl Into<String>) -> CreateSelectMenuOption {
  CreateSelectMenuOption::new(label, value)
}

// ─── Text inputs (modal fields) ──────────────────────────────────────────────

pub fn text_short(id: impl Into<String>, label: impl Into<String>) -> CreateInputText {
  CreateInputText::new(InputTextStyle::Short, label, id)
}

pub fn text_paragraph(id: impl Into<String>, label: impl Into<String>) -> CreateInputText {
  CreateInputText::new(InputTextStyle::Paragraph, label, id)
}
