use sea_orm_migration::{
  prelude::{extension::postgres::Type, *},
  schema::*,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    // 1. Create Account Table
    manager
      .create_table(
        Table::create()
          .table(Account::Table)
          .if_not_exists()
          .col(pk_uuid(Account::Id).default(SimpleExpr::Custom("gen_random_uuid()".into())))
          .col(text(Account::RobloxName))
          .col(big_integer_uniq(Account::RobloxId))
          .col(text(Account::Password))
          .col(text(Account::Cookie))
          .to_owned(),
      )
      .await?;

    // 2. Create User Table
    manager
      .create_table(
        Table::create()
          .table(User::Table)
          .if_not_exists()
          .col(pk_uuid(User::Id).default(SimpleExpr::Custom("gen_random_uuid()".into())))
          .col(text_uniq(User::DiscordId))
          .col(big_integer_uniq(User::RobloxId))
          .to_owned(),
      )
      .await?;

    // 3. Create Clan Table
    manager
      .create_table(
        Table::create()
          .table(Clan::Table)
          .if_not_exists()
          .col(pk_uuid(Clan::Id).default(SimpleExpr::Custom("gen_random_uuid()".into())))
          .col(text_uniq(Clan::DiscordGuildId))
          .col(big_integer_uniq(Clan::RobloxGroupId))
          .col(big_integer_uniq(Clan::RankerRobloxId))
          .col(uuid_uniq(Clan::ApiAuthToken))
          .col(boolean(Clan::XpEnabled).default(false))
          .col(boolean(Clan::ApiEnabled).default(false))
          .col(
            ColumnDef::new(Clan::RankingAccessRoles)
              .array(ColumnType::Text)
              .not_null(),
          )
          .col(
            ColumnDef::new(Clan::RankingRequesterRoles)
              .array(ColumnType::Text)
              .not_null(),
          )
          .col(text_null(Clan::VerificationRole))
          .col(text(Clan::RankingLogChannel))
          .col(text_null(Clan::RankingRequestChannel))
          .col(text_null(Clan::EventChannel))
          .to_owned(),
      )
      .await?;

    // 4. Create ClanUser Table
    manager
      .create_table(
        Table::create()
          .table(ClanUser::Table)
          .if_not_exists()
          .col(pk_uuid(ClanUser::Id).default(SimpleExpr::Custom("gen_random_uuid()".into())))
          .col(uuid(ClanUser::UserId))
          .col(uuid(ClanUser::ClanId))
          .col(integer(ClanUser::XpLevel).default(0))
          .foreign_key(
            ForeignKey::create()
              .from(ClanUser::Table, ClanUser::ClanId)
              .to(Clan::Table, Clan::Id)
              .on_delete(ForeignKeyAction::Cascade),
          )
          .foreign_key(
            ForeignKey::create()
              .from(ClanUser::Table, ClanUser::UserId)
              .to(User::Table, User::Id)
              .on_delete(ForeignKeyAction::Cascade),
          )
          .to_owned(),
      )
      .await?;

    // 5. Create Medal Table
    manager
      .create_table(
        Table::create()
          .table(Medal::Table)
          .if_not_exists()
          .col(pk_uuid(Medal::Id).default(SimpleExpr::Custom("gen_random_uuid()".into())))
          .col(uuid(Medal::ClanId))
          .col(text(Medal::Name))
          .col(text(Medal::Description))
          .col(text(Medal::DiscordRoleId))
          .to_owned(),
      )
      .await?;
    
    // 6. Create reward_type Type
    manager
      .create_type(
        Type::create()
          .as_enum(Alias::new("reward_type"))
          .values([
            Alias::new("Promote"),
            Alias::new("Medal"),
            Alias::new("Xp"),
            Alias::new("Rank"),
          ])
          .to_owned(),
      )
      .await?;

    // 7. Create Event Table
    manager
      .create_table(
        Table::create()
          .table(Event::Table)
          .if_not_exists()
          .col(pk_uuid(Event::Id).default(SimpleExpr::Custom("gen_random_uuid()".into())))
          .col(uuid(Event::ClanId))
          .col(text(Event::DiscordEventId))
          .col(text(Event::AuthorId))
          .col(
            ColumnDef::new(Event::Awardees)
              .array(ColumnType::Text)
              .not_null(),
          )
          .col(ColumnDef::new(Event::RewardType).custom("reward_type"))
          .col(string_null(Event::RewardData))
          .col(uuid(Event::ConcludedBy))
          .col(timestamp_null(Event::ConcludedAt))
          .foreign_key(
            ForeignKey::create()
              .from(Event::Table, Event::ClanId)
              .to(Clan::Table, Clan::Id)
              .on_delete(ForeignKeyAction::Cascade),
          )
          .to_owned(),
      )
      .await?;

    // 8. Create RoleLevel Table
    manager
      .create_table(
        Table::create()
          .table(RoleLevel::Table)
          .if_not_exists()
          .col(pk_uuid(RoleLevel::Id).default(SimpleExpr::Custom("gen_random_uuid()".into())))
          .col(uuid(RoleLevel::ClanId))
          .col(text(RoleLevel::Name))
          .col(text(RoleLevel::Description))
          .col(text(RoleLevel::DiscordRoleId))
          .col(boolean(RoleLevel::PDirectRanking))
          .col(boolean(RoleLevel::PRankingRequest))
          .col(boolean(RoleLevel::PExileAccess))
          .col(boolean(RoleLevel::PMedalAccess))
          .col(boolean(RoleLevel::PXpAccess))
          .col(boolean(RoleLevel::PXpRequests))
          .col(boolean(RoleLevel::PEventsAccess))
          .col(boolean(RoleLevel::PEventsAdmin))
          .col(boolean(RoleLevel::PChangeShout))
          .col(boolean(RoleLevel::PJoinRequestsAccess))
          .col(boolean(RoleLevel::PJoinRequestsAdmin))
          .col(boolean(RoleLevel::PBulkRequests))
          .col(boolean(RoleLevel::PAdministrator))
          .col(integer(RoleLevel::RankingRangeMin))
          .col(integer(RoleLevel::RankingRangeMax))
          .foreign_key(
            ForeignKey::create()
              .from(RoleLevel::Table, RoleLevel::ClanId)
              .to(Clan::Table, Clan::Id)
              .on_delete(ForeignKeyAction::Cascade),
          )
          .to_owned(),
      )
      .await?;

    Ok(())
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    // 1. Drop Account Table
    manager
      .drop_table(Table::drop().table(Account::Table).to_owned())
      .await?;

    // 2. Drop User Table
    manager
      .drop_table(Table::drop().table(User::Table).to_owned())
      .await?;

    // 3. Drop Clan Table
    manager
      .drop_table(Table::drop().table(Clan::Table).to_owned())
      .await?;

    // 4. Drop ClanUser Table
    manager
      .drop_table(Table::drop().table(ClanUser::Table).to_owned())
      .await?;

    // 5. Drop Medal Table
    manager
      .drop_table(Table::drop().table(Medal::Table).to_owned())
      .await?;

    // 6. Drop Event Table
    manager
      .drop_table(Table::drop().table(Event::Table).to_owned())
      .await?;

    // 7. Drop RoleLevel Table
    manager
      .drop_table(Table::drop().table(RoleLevel::Table).to_owned())
      .await?;

    // 8. Drop reward_type Type
    manager
      .drop_type(Type::drop().name(Alias::new("reward_type")).to_owned())
      .await?;

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
enum Clan {
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
  EventChannel,
}

#[derive(DeriveIden)]
enum ClanUser {
  Table,
  Id,
  UserId,
  ClanId,
  XpLevel,
}

#[derive(DeriveIden)]
enum Medal {
  Table,
  Id,
  ClanId,

  Name,
  Description,
  DiscordRoleId,
}

#[derive(DeriveIden)]
enum Event {
  Table,
  Id,
  ClanId,

  DiscordEventId,
  AuthorId,

  Awardees,
  RewardType,
  RewardData,

  ConcludedBy,
  ConcludedAt,
}

// /event add/set/remove awardee
// /event info
// /event start/stop

#[derive(DeriveIden)]
enum RoleLevel {
  Table,
  Id,
  ClanId,

  Name,
  Description,
  DiscordRoleId,

  PDirectRanking,
  PRankingRequest,
  PExileAccess,
  PMedalAccess,
  PXpAccess,
  PXpRequests,
  PEventsAdmin,
  PEventsAccess,
  PChangeShout,
  PJoinRequestsAccess,
  PJoinRequestsAdmin,
  PBulkRequests,
  PAdministrator,

  RankingRangeMin,
  RankingRangeMax,
}
