use poise::serenity_prelude as serenity;
use sea_orm::ActiveValue::{NotSet, Set};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QuerySelect};
use sea_query::{Expr, extension::postgres::PgExpr};

use crate::utils::emojis::*;
use crate::entity::account as Account;
use crate::{acknowledge, embeds, helper, multi_response, response, send, update_msg, CmdError, CommandContext, CommandResult, Context, RankerAccount};

#[poise::command(
  slash_command,
  description_localized("en-US", "Manages ranker accounts"),
  subcommands("view", "create")
)]
pub async fn accounts(_: Context<'_>) -> CommandResult {
  //! Just a placeholder for the subcommands
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
    _display_ranker_details(
      ctx,
      serenity::Interaction::Command(ctx.interaction.clone()),
      rankers,
      name.unwrap()
  ).await?;

    return Ok(())
  }

  let accounts = Account::Entity::find().all(db).await?;

  if accounts.len() == 0 {
    return Err(CmdError::Command(
      response!(ctx.interaction, ctx,
        .embed(embeds::error("Account Manager", "No accounts could be found in the database."))
        .ephemeral(true)
      )
    ))
  }

  let acc_options = accounts
    .iter()
    .map(|acc| {
      let ranker = rankers.iter().find(|r| r.roblox.id == acc.roblox_id);
      let select_option = serenity::CreateSelectMenuOption::new(
        acc.roblox_name.clone(), 
        acc.roblox_name.to_string()
      );

      match ranker {
        Some(r) => {
          select_option
            .description(format!("Groups: {} | Age: {}", r.groups.len(), r.roblox.created.format("%d/%m/%Y")))
            .emoji(serenity::ReactionType::Unicode(String::from("✅")))
        }
        None => {
          select_option
            .description(format!("Groups: N/A | Age: N/A"))
            .emoji(serenity::ReactionType::Unicode(String::from("❌")))
        }
      }
    })
    .collect::<Vec<serenity::CreateSelectMenuOption>>();

  let mut conn_status: [Vec<String>; 2] = [vec![], vec![]];

  accounts
    .iter()
    .enumerate()
    .for_each(|(i, acc)| {
      let ranker = rankers.iter().find(|r| r.roblox.id == acc.roblox_id);

      let conn = match ranker {
        Some(r) => format!("{}\u{2007}`{}` `[{}]`", use_emoji(Emoji::Tick), r.roblox.name, r.groups.len()),
        None => format!("{}\u{2007}`{}` `[{}]`", use_emoji(Emoji::Cross), acc.roblox_name, "N/A"),
      };

      if i < (rankers.len() + 1) / 2 {
        conn_status[0].push(conn);
      } else {
        conn_status[1].push(conn);
      }
    });
  
  let info_embed = embeds::info("Account Manager", format!("Showing all {} ranker accounts.", accounts.len()))
    .field("Accounts", conn_status[0].join("\n"), true)
    .field("\u{200b}", conn_status[1].join("\n"), true);

  let account_select = serenity::CreateSelectMenu::new(
    "accounts::selector",
    serenity::CreateSelectMenuKind::String { options: acc_options }
  );

  response!(ctx.interaction, ctx,
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

    _display_ranker_details(
      ctx,
      serenity::Interaction::Component(mci),
      rankers,
      acc_name.unwrap()
    ).await?;
  }

  Ok(())
}

// this is not the prettiest
async fn _display_ranker_details(ctx: Context<'_>, interaction: serenity::Interaction, rankers: &Vec<RankerAccount>, name: String) -> CommandResult {
  let account = Account::Entity::find()
    .filter(Expr::col(Account::Column::RobloxName).ilike(format!("%{}%", name)))
    .one(&ctx.data().db)
    .await?;

  let account = match account {
    Some(a) => a,
    None => return Err(CmdError::Command(
      multi_response!(interaction, ctx, embeds::not_found("Account", Some(&name)),
        {
          Command => response,
          Component => update_msg
        }
      )
    ))
  };

  let ranker_info = match rankers.iter().find(|r| r.roblox.name.contains(&name)) {
    Some(r) => {
      (
        format!("Connection: {}", use_emoji(Emoji::Tick)),
        format!("**Creation Date:** {}\n**Groups:** {}", r.roblox.created.format("%d/%m/%Y"), r.groups.len()),
        false
      )
    },
    None => {
      (
        format!("Connection: {}", use_emoji(Emoji::Cross)),
        format!("**Creation Date:** N/A\n**Groups:** N/A"),
        false
      )
    }
  };

  let ranker_embed = embeds::info(
    "Account Manager",
    format!("Showing information for ranker account: `{}`.", account.roblox_name)
  ).fields([ranker_info]);

  multi_response!(interaction, ctx,
    ranker_embed,
    vec![serenity::CreateActionRow::Buttons(vec![
      serenity::CreateButton::new("accounts::0::view_password").label("View Password").style(serenity::ButtonStyle::Secondary),
      serenity::CreateButton::new("accounts::0::view_cookie").label("View Cookie").style(serenity::ButtonStyle::Secondary),
      serenity::CreateButton::new("accounts::0::change_cookie").label("Change Cookie").style(serenity::ButtonStyle::Danger)
    ])],
    {
      Command => response,
      Component => update_msg
    }
  )?;

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
      serenity::CreateInputText::new(
        serenity::InputTextStyle::Paragraph,
        "Security Cookie",
        "accounts::create::cookie",
      )
      .placeholder("The .ROBLOSECURITY cookie of the account")
      .required(true),
    )
    .field(
      serenity::CreateInputText::new(
        serenity::InputTextStyle::Short,
        "Account Password",
        "accounts::create::password",
      )
      .placeholder("The password used to login to this account")
      .required(true)
      .min_length(50)
      .max_length(50),
    )
    .field(
      serenity::CreateInputText::new(
        serenity::InputTextStyle::Short,
        "Account Username",
        "accounts::create::username",
      )
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
    None => return Err(CmdError::Command(
      response!(modal_res.interaction, ctx,
        .embed(embeds::not_found("User", Some(username)))
        .ephemeral(true)
      )
    ))
  };

  if Account::Entity::find()
    .filter(Account::Column::RobloxId.eq(user.id))
    .one(db)
    .await?
    .is_some()
  {
    return Err(CmdError::Command(
      response!(modal_res.interaction, ctx,
        .embed(embeds::error("Account Manager", format!("The account `{}` has already been added.", user.name)))
        .ephemeral(true)
      )
    ));
  }

  let auth_user = match oxid_roblox::authenticated_user(Some(cookie.to_string())).await {
    Ok(u) => u,
    Err(_) => return Err(CmdError::Command(
      response!(modal_res.interaction, ctx,
        .embed(embeds::error("", "Failed to authenticate with the cookie provided."))
        .ephemeral(true)
      )
    ))
  };

  if auth_user.id != user.id {
    return Err(CmdError::Command(
      response!(modal_res.interaction, ctx,
        .embed(embeds::error("", "The authenticated user did not match the one provided."))
        .ephemeral(true)
      )
    ));
  }

  let account = Account::ActiveModel {
    id: NotSet,
    roblox_id: Set(auth_user.id),
    roblox_name: Set(auth_user.name),
    password: Set(password.to_string()),
    cookie: Set(cookie.to_string()),

    ..Default::default()
  };

  Account::Entity::insert(account)
    .exec(db)
    .await?;

  response!(modal_res.interaction, ctx,
    .embed(embeds::success("Account Manager", format!("Successfully stored the credentials for account: `{}`", user.name)))
    .ephemeral(true)
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
    .map(|acc| serenity::AutocompleteChoice::new(
      acc.roblox_name.clone(),
      acc.roblox_name
    ))
    .collect()
}