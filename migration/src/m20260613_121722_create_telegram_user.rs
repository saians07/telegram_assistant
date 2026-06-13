use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TelegramUsers::Table)
                    .if_not_exists()
                    .col(pk_auto(TelegramUsers::Id))
                    .col(integer(TelegramUsers::TelegramChatId))
                    .col(uuid(TelegramUsers::UserId))
                    .col(string(TelegramUsers::Tier))
                    .col(integer(TelegramUsers::DailyLimit))
                    .col(text_null(TelegramUsers::ProfileSummary))
                    .col(text_null(TelegramUsers::ChatSummary))
                    .col(
                        ColumnDef::new(TelegramUsers::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(TelegramUsers::UpdatedAt)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TelegramUsers::DeletedAt)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_telegram_users_user_id")
                            .from(TelegramUsers::Table, TelegramUsers::UserId)
                            .to(Alias::new("users"), Alias::new("id")),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TelegramUsers::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum TelegramUsers {
    Table,
    Id,
    TelegramChatId,
    UserId,
    Tier,
    DailyLimit,
    ProfileSummary,
    ChatSummary,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}
