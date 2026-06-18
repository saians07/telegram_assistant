pub use sea_orm_migration::prelude::*;

mod m20260613_104356_create_table_user;
mod m20260613_121722_create_telegram_user;
mod m20260613_170216_create_unauthorized_access;
mod m20260613_170229_create_unauthorized_chat_history;
mod m20260613_170701_create_telegram_chat_history;
mod m20260613_172643_create_telegram_chat_queue;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260613_104356_create_table_user::Migration),
            Box::new(m20260613_121722_create_telegram_user::Migration),
            Box::new(m20260613_170216_create_unauthorized_access::Migration),
            Box::new(m20260613_170229_create_unauthorized_chat_history::Migration),
            Box::new(m20260613_170701_create_telegram_chat_history::Migration),
            Box::new(m20260613_172643_create_telegram_chat_queue::Migration),
        ]
    }

    fn migration_table_name() -> DynIden {
        sea_query::Alias::new("seaql_migrations_swan").into_iden()
    }
}
