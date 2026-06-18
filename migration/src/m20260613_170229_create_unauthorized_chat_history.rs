use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TelegramGuestChatHistory::Table)
                    .if_not_exists()
                    .col(pk_auto(TelegramGuestChatHistory::Id))
                    .col(integer(TelegramGuestChatHistory::TelegramGuestId))
                    .col(string(TelegramGuestChatHistory::Message))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_telegram_unauthorized_chat_history_telegram_guest_id")
                            .from(
                                TelegramGuestChatHistory::Table,
                                TelegramGuestChatHistory::TelegramGuestId,
                            )
                            .to(Alias::new("telegram_guest"), Alias::new("id")),
                    )
                    .col(
                        ColumnDef::new(TelegramGuestChatHistory::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(TelegramGuestChatHistory::DeletedAt)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TelegramGuestChatHistory::UpdatedAt)
                            .timestamp_with_time_zone()
                            .null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(TelegramGuestChatHistory::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum TelegramGuestChatHistory {
    Table,
    Id,
    TelegramGuestId,
    Message,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}
