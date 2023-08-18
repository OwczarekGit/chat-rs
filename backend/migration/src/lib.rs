pub use sea_orm_migration::prelude::*;

mod m20230818_164458_create_account_table;
mod m20230818_170513_create_session_table;
mod m20230818_222729_create_chat_table;
mod m20230818_222947_create_message_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230818_164458_create_account_table::Migration),
            Box::new(m20230818_170513_create_session_table::Migration),
            Box::new(m20230818_222729_create_chat_table::Migration),
            Box::new(m20230818_222947_create_message_table::Migration),
        ]
    }
}
