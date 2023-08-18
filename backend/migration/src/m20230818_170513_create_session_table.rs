use sea_orm_migration::prelude::*;
use super::m20230818_164458_create_account_table::Account;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(Session::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Session::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Session::Key).string().not_null())
                    .col(ColumnDef::new(Session::AccountId).big_integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Session::Table, Session::AccountId)
                            .to(Account::Table, Account::Id)
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(Session::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Session {
    Table,
    Id,
    Key,
    AccountId,
}
