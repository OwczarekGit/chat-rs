use sea_orm_migration::prelude::*;
use super::m20230818_222729_create_chat_table::Chat;
use super::m20230819_115126_create_profile_table::Profile;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(Message::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Message::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Message::Content).string().not_null())
                    .col(ColumnDef::new(Message::ChatId).big_integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Message::Table, Message::ChatId)
                            .to(Chat::Table, Chat::Id)
                    )
                    .col(ColumnDef::new(Message::ProfileId).big_integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Message::Table, Message::ProfileId)
                            .to(Profile::Table, Profile::Id)
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(Message::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Message {
    Table,
    Id,
    ProfileId,
    ChatId,
    Content,
}
