use poise::serenity_prelude as serenity;
use sea_orm::ActiveValue::{NotSet, Set};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QuerySelect};
use sea_query::{extension::postgres::PgExpr, Expr};

use crate::entity::account as Account;
use crate::utils::emojis::*;
use crate::{
  components, embeds, helper, CmdError, CommandContext, CommandResult, Context, RankerAccount,
};

#[poise::command(
  slash_command,
  description_localized("en-US", "Manages ranker accounts"),
  subcommands("view", "create")
)]
pub async fn accounts(_: Context<'_>) -> CommandResult {
  //* Just a placeholder for the subcommands
  Ok(())
}

#[poise::command(
  slash_command,
  description_localized("en-US", "Views and manages existing ranker accounts")
)]
pub async fn view(
  ctx: Context<'_>,
  #[description = "The name of the ranker account"]
  #[autocomplete = "ranker_autocomplete"]
  name: Option<String>,
) -> CommandResult {
  let CommandContext { db, rankers, .. } = helper::command_context(&ctx).await?;

  if name.is_some() {
    display_account(
      ctx,
      helper::AnyInteraction::Command(ctx.interaction.clone()),
      rankers,
      name.unwrap(),
    )
    .await?;

    return Ok(());
  }

  let accounts = Account::Entity::find().all(db).await?;

  if accounts.is_empty() {
    helper::response!(ctx.interaction, ctx,
      .embed(embeds::error("Account Manager", "No accounts could be found in the database."))
    )?;
    return Err(CmdError::Handled);
  }

  let acc_options = accounts
    .iter()
    .map(|acc| {
      let ranker = rankers.iter().find(|r| r.roblox.id == acc.roblox_id);
      let opt = components::option(acc.roblox_name.clone(), acc.roblox_name.clone());

      match ranker {
        Some(r) => opt
          .description(format!(
            "Groups: {} | Age: {}",
            r.groups.len(),
            r.roblox.created.format("%d/%m/%Y")
          ))
          .emoji(serenity::ReactionType::Unicode(String::from("✅"))),
        None => opt
          .description("Groups: N/A | Age: N/A")
          .emoji(serenity::ReactionType::Unicode(String::from("❌"))),
      }
    })
    .collect();

  let mut conn_status: [Vec<String>; 2] = [vec![], vec![]];

  accounts.iter().enumerate().for_each(|(i, acc)| {
    let ranker = rankers.iter().find(|r| r.roblox.id == acc.roblox_id);

    let conn = match ranker {
      Some(r) => {
        format!("{}\u{2007}`{}` `[{}]`", use_emoji(Emoji::Tick), r.roblox.name, r.groups.len())
      },
      None => format!("{}\u{2007}`{}` `[{}]`", use_emoji(Emoji::Cross), acc.roblox_name, "N/A"),
    };

    if i < (rankers.len() + 1) / 2 {
      conn_status[0].push(conn);
    } else {
      conn_status[1].push(conn);
    }
  });

  let info_embed =
    embeds::info("Account Manager", format!("Showing all {} ranker accounts.", accounts.len()))
      .field("Accounts", conn_status[0].join("\n"), true)
      .field("\u{200b}", conn_status[1].join("\n"), true);

  let account_select = components::select("accounts::selector", acc_options);

  helper::response!(ctx.interaction, ctx,
    .embed(info_embed)
    .select_menu(account_select)
    .ephemeral(true)
  )?;

  if let Some(mci) = serenity::ComponentInteractionCollector::new(ctx)
    .timeout(std::time::Duration::from_secs(120))
    .message_id(ctx.interaction.get_response(ctx.http()).await?.id)
    .await
  {
    let acc_name = match &mci.data.kind {
      serenity::ComponentInteractionDataKind::StringSelect { values } => values.get(0).cloned(),
      _ => None,
    };

    display_account(ctx, helper::AnyInteraction::Component(mci), rankers, acc_name.unwrap())
      .await?;
  }

  Ok(())
}

async fn display_account(
  ctx: Context<'_>,
  interaction: helper::AnyInteraction,
  rankers: &Vec<RankerAccount>,
  name: String,
) -> CommandResult {
  let account = Account::Entity::find()
    .filter(Expr::col(Account::Column::RobloxName).ilike(format!("%{}%", name)))
    .one(&ctx.data().db)
    .await?;

  let account = match account {
    Some(a) => a,
    None => {
      helper::any_update!(interaction, ctx,
        .embed(embeds::not_found("Account", Some(&name)))
      )?;
      return Err(CmdError::Handled);
    },
  };

  let ranker_info = match rankers.iter().find(|r| r.roblox.name.contains(&name)) {
    Some(r) => (
      format!("Connection: {}", use_emoji(Emoji::Tick)),
      format!(
        "**Creation Date:** {}\n**Groups:** {}",
        r.roblox.created.format("%d/%m/%Y"),
        r.groups.len()
      ),
      false,
    ),
    None => (
      format!("Connection: {}", use_emoji(Emoji::Cross)),
      String::from("**Creation Date:** N/A\n**Groups:** N/A"),
      false,
    ),
  };

  let ranker_embed = embeds::info(
    "Account Manager",
    format!("Showing information for ranker account: `{}`.", account.roblox_name),
  )
  .fields([ranker_info]);

  helper::any_update!(interaction, ctx,
    .embed(ranker_embed)
    .components(vec![components::buttons(vec![
      components::button_secondary("accounts::0::view_password", "View Password"),
      components::button_secondary("accounts::0::view_cookie", "View Cookie"),
      components::button_danger("accounts::0::change_cookie", "Change Cookie"),
    ])])
  )?;

  let msg_id = ctx.interaction.get_response(ctx.http()).await?.id;

  loop {
    let Some(mci) = serenity::ComponentInteractionCollector::new(ctx)
      .timeout(std::time::Duration::from_secs(120))
      .message_id(msg_id)
      .await
    else {
      break;
    };

    match mci.data.custom_id.as_str() {
      "accounts::0::view_password" => {
        helper::response!(mci, ctx,
          .embed(
            embeds::info("Account Manager", format!("Password for `{}`", account.roblox_name))
              .field("Password", format!("```{}```", account.password), false),
          )
        )?;
      },
      "accounts::0::view_cookie" => {
        helper::response!(mci, ctx,
          .embed(
            embeds::info("Account Manager", format!("Cookie for `{}`", account.roblox_name))
              .field("Cookie", format!("```{}```", account.cookie), false),
          )
        )?;
      },
      "accounts::0::change_cookie" => {
        mci
          .create_response(
            ctx.http(),
            serenity::CreateInteractionResponse::Modal(
              serenity::CreateModal::new("accounts::change_cookie::modal", "Change Cookie")
                .components(vec![serenity::CreateActionRow::InputText(
                  components::text_paragraph("accounts::change_cookie::cookie", "Security Cookie")
                    .placeholder("The new .ROBLOSECURITY cookie")
                    .required(true),
                )]),
            ),
          )
          .await?;

        let Some(modal) = serenity::ModalInteractionCollector::new(ctx)
          .timeout(std::time::Duration::from_secs(300))
          .await
        else {
          return Err(CmdError::Timeout);
        };

        let new_cookie = modal
          .data
          .components
          .first()
          .and_then(|row| row.components.first())
          .and_then(|c| match c {
            serenity::ActionRowComponent::InputText(input) => input.value.clone(),
            _ => None,
          })
          .unwrap_or_default();

        let auth_user = match oxid_roblox::authenticated_user(Some(new_cookie.clone())).await {
          Ok(u) => u,
          Err(_) => {
            helper::response!(modal, ctx,
              .embed(embeds::error("", "Failed to authenticate with the cookie provided."))
            )?;
            return Err(CmdError::Handled);
          },
        };

        if auth_user.id != account.roblox_id {
          helper::response!(modal, ctx,
            .embed(embeds::error("", "The authenticated user does not match this account."))
          )?;
          return Err(CmdError::Handled);
        }

        let mut active: Account::ActiveModel = account.into();
        active.cookie = Set(new_cookie);
        Account::Entity::update(active).exec(&ctx.data().db).await?;

        helper::response!(modal, ctx,
          .embed(embeds::success(
            "Account Manager",
            format!("Cookie updated successfully for `{}`.", auth_user.name),
          ))
        )?;

        break;
      },
      _ => {},
    }
  }

  Ok(())
}

#[poise::command(
  slash_command,
  description_localized("en-US", "Creates a new ranker account")
)]
pub async fn create(ctx: Context<'_>) -> CommandResult {
  let CommandContext { db, .. } = helper::command_context(&ctx).await?;

  let create_modal = serenity::CreateQuickModal::new("Create Account")
    .timeout(std::time::Duration::from_secs(300))
    .field(
      components::text_paragraph("accounts::create::cookie", "Security Cookie")
        .placeholder("The .ROBLOSECURITY cookie of the account")
        .required(true),
    )
    .field(
      components::text_short("accounts::create::password", "Account Password")
        .placeholder("The password used to login to this account")
        .required(true)
        .min_length(50)
        .max_length(50),
    )
    .field(
      components::text_short("accounts::create::username", "Account Username")
        .placeholder("The username of this account")
        .required(true),
    );

  let modal_res = ctx
    .interaction
    .quick_modal(ctx.serenity_context(), create_modal)
    .await?
    .ok_or(CmdError::Timeout)?;

  let cookie = modal_res.inputs.get(0).unwrap();
  let password = modal_res.inputs.get(1).unwrap();
  let username = modal_res.inputs.get(2).unwrap();

  let user = match oxid_roblox::user_from_username(username).await? {
    Some(u) => u,
    None => {
      helper::response!(modal_res.interaction, ctx,
        .embed(embeds::not_found("User", Some(username)))
      )?;
      return Err(CmdError::Handled);
    },
  };

  if Account::Entity::find()
    .filter(Account::Column::RobloxId.eq(user.id))
    .one(db)
    .await?
    .is_some()
  {
    helper::response!(modal_res.interaction, ctx,
      .embed(embeds::error("Account Manager", format!("The account `{}` has already been added.", user.name)))
    )?;
    return Err(CmdError::Handled);
  }

  let auth_user = match oxid_roblox::authenticated_user(Some(cookie.to_string())).await {
    Ok(u) => u,
    Err(_) => {
      helper::response!(modal_res.interaction, ctx,
        .embed(embeds::error("", "Failed to authenticate with the cookie provided."))
      )?;
      return Err(CmdError::Handled);
    },
  };

  if auth_user.id != user.id {
    helper::response!(modal_res.interaction, ctx,
      .embed(embeds::error("", "The authenticated user did not match the one provided."))
    )?;
    return Err(CmdError::Handled);
  }

  let account = Account::ActiveModel {
    id: NotSet,
    roblox_id: Set(auth_user.id),
    roblox_name: Set(auth_user.name),
    password: Set(password.to_string()),
    cookie: Set(cookie.to_string()),

    ..Default::default()
  };

  Account::Entity::insert(account).exec(db).await?;

  helper::response!(modal_res.interaction, ctx,
    .embed(embeds::success("Account Manager", format!("Successfully stored the credentials for account: `{}`", user.name)))
  )?;

  Ok(())
}

async fn ranker_autocomplete(ctx: Context<'_>, partial: &str) -> Vec<serenity::AutocompleteChoice> {
  let db = &ctx.data().db;

  let accounts = Account::Entity::find()
    .filter(Expr::col(Account::Column::RobloxName).ilike(format!("%{}%", partial)))
    .limit(25)
    .all(db)
    .await
    .unwrap_or_default();

  accounts
    .into_iter()
    .map(|acc| serenity::AutocompleteChoice::new(acc.roblox_name.clone(), acc.roblox_name))
    .collect()
}
