use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TelegramGuest::Table)
                    .if_not_exists()
                    .col(pk_auto(TelegramGuest::Id))
                    .col(integer(TelegramGuest::TelegramChatId))
                    .col(boolean(TelegramGuest::Authorized))
                    .col(
                        ColumnDef::new(TelegramGuest::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(TelegramGuest::UpdatedAt)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TelegramGuest::DeletedAt)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TelegramGuest::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum TelegramGuest {
    Table,
    Id,
    TelegramChatId,
    Authorized,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}
