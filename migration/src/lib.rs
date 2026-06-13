pub use sea_orm_migration::prelude::*;

mod m20260613_104356_create_table_user;
mod m20260613_121715_create_telegram_chat_history;
mod m20260613_121722_create_telegram_user;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260613_104356_create_table_user::Migration),
            Box::new(m20260613_121715_create_telegram_chat_history::Migration),
            Box::new(m20260613_121722_create_telegram_user::Migration),
        ]
    }
}
