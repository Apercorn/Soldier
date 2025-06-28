use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    // 1. Create Account table
    let _ = manager
      .create_table(
        Table::create()
          .table(Account::Table)
          .if_not_exists()
          .col(pk_uuid(Account::Id))
          .col(text(Account::RobloxName))
          .col(big_integer_uniq(Account::RobloxId))
          .col(text(Account::Password))
          .col(text(Account::Cookie))
          .to_owned(),
      )
      .await;

    // 2. Create User table
    let _ = manager
      .create_table(
        Table::create()
          .table(User::Table)
          .if_not_exists()
          .col(pk_uuid(User::Id))
          .col(text_uniq(User::DiscordId))
          .col(big_integer_uniq(User::RobloxId))
          .to_owned(),
      )
      .await;

    // 3. Create Guild Table
    let _ = manager
      .create_table(
        Table::create()
          .table(Guild::Table)
          .if_not_exists()
          .col(pk_uuid(Guild::Id))
          .col(text_uniq(Guild::DiscordGuildId))
          .col(big_integer_uniq(Guild::RobloxGroupId))
          .col(big_integer_uniq(Guild::RankerRobloxId))
          .col(uuid_uniq(Guild::ApiAuthToken))
          .col(boolean(Guild::XpEnabled).default(false))
          .col(boolean(Guild::ApiEnabled).default(false))
          .col(ColumnDef::new(Guild::RankingAccessRoles).array(ColumnType::Text).not_null())
          .col(ColumnDef::new(Guild::RankingRequesterRoles).array(ColumnType::Text).not_null())
          .col(text_null(Guild::VerificationRole))
          .col(text(Guild::RankingLogChannel))
          .col(text_null(Guild::RankingRequestChannel))
          .col(text_null(Guild::EventDisplayChannel))
          .col(text_null(Guild::EventPanelChannel))
          .to_owned()
      )
      .await;

    // 4. Create GuildUser Table
    let _ = manager
      .create_table(
        Table::create()
          .table(GuildUser::Table)
          .if_not_exists()
          .col(pk_uuid(GuildUser::Id))
          .col(text(GuildUser::DiscordId))
          .col(text(GuildUser::GuildId))
          .col(integer(GuildUser::XpLevel).default(0))
          .to_owned()
      )
      .await;

    Ok(())
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    // 1. Drop Account table
    let _ = manager
      .drop_table(Table::drop().table(Account::Table).to_owned())
      .await;
    
    // 2. Drop User table
    let _ = manager
      .drop_table(Table::drop().table(User::Table).to_owned())
      .await;

    // 3. Drop Guild table
    let _ = manager
      .drop_table(Table::drop().table(Guild::Table).to_owned())
      .await;

    // 4. Drop GuildUser table
    let _ = manager
      .drop_table(Table::drop().table(GuildUser::Table).to_owned())
      .await;

    Ok(())
  }
}

#[derive(DeriveIden)]
enum Account {
  Table,
  Id,
  RobloxName,
  RobloxId,
  Password,
  Cookie,
}

#[derive(DeriveIden)]
enum User {
  Table,
  Id,
  DiscordId,
  RobloxId,
}

#[derive(DeriveIden)]
enum Guild {
  Table,
  Id,
  DiscordGuildId,
  RobloxGroupId,
  RankerRobloxId,
  ApiAuthToken,

  XpEnabled,
  ApiEnabled,

  RankingAccessRoles,
  RankingRequesterRoles,
  VerificationRole,

  RankingLogChannel,
  RankingRequestChannel,
  EventDisplayChannel,
  EventPanelChannel,
}

#[derive(DeriveIden)]
enum GuildUser {
  Table,
  Id,
  DiscordId,
  GuildId,
  XpLevel
}