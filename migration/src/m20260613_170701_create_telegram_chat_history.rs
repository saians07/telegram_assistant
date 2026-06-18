use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TelegramChatHistory::Table)
                    .if_not_exists()
                    .col(pk_auto(TelegramChatHistory::Id))
                    .col(string(TelegramChatHistory::Author))
                    .col(integer(TelegramChatHistory::TelegramUserId))
                    .col(text(TelegramChatHistory::Message))
                    .col(string(TelegramChatHistory::SessionId))
                    .col(
                        ColumnDef::new(TelegramChatHistory::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(TelegramChatHistory::UpdatedAt)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TelegramChatHistory::DeletedAt)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_telegram_chat_history_telegram_user_id")
                            .from(
                                TelegramChatHistory::Table,
                                TelegramChatHistory::TelegramUserId,
                            )
                            .to(Alias::new("telegram_users"), Alias::new("id")),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TelegramChatHistory::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum TelegramChatHistory {
    Table,
    Id,
    Author,         // user or bot
    TelegramUserId, // related to whom this converstation to not the one who send the message
    Message,        // can not be and should not be empty
    CreatedAt,
    UpdatedAt,
    DeletedAt, // only soft delete
    SessionId,
}
