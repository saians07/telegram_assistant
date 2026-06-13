use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TelegramChatQueue::Table)
                    .if_not_exists()
                    .col(pk_auto(TelegramChatQueue::Id))
                    .col(integer(TelegramChatQueue::ChatHistoryId))
                    .col(integer(TelegramChatQueue::ReplyThresholdInSeconds))
                    .col(
                        ColumnDef::new(TelegramChatQueue::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(TelegramChatQueue::UpdatedAt)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_telegram_chat_queue_chat_history_id")
                            .from(TelegramChatQueue::Table, TelegramChatQueue::ChatHistoryId)
                            .to(Alias::new("telegram_chat_history"), Alias::new("id")),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TelegramChatQueue::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum TelegramChatQueue {
    Table,
    Id,
    ChatHistoryId,
    ReplyThresholdInSeconds, // we define when to reply from this. only reply when threshold hit!
    CreatedAt,               // if the
    UpdatedAt,               // let's hard delete this
}
