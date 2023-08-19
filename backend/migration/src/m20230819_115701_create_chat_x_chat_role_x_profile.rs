use sea_orm_migration::prelude::*;

use super::m20230819_115126_create_profile_table::Profile;
use super::m20230819_115446_create_chat_role_table::ChatRole;
use super::m20230818_222729_create_chat_table::Chat;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(ChatXChatRoleXProfile::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ChatXChatRoleXProfile::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ChatXChatRoleXProfile::ChatId).big_integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(ChatXChatRoleXProfile::Table, ChatXChatRoleXProfile::ChatId)
                            .to(Chat::Table, Chat::Id)
                    )
                    .col(ColumnDef::new(ChatXChatRoleXProfile::ProfileId).big_integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(ChatXChatRoleXProfile::Table, ChatXChatRoleXProfile::ProfileId)
                            .to(Profile::Table, Profile::Id)
                    )
                    .col(ColumnDef::new(ChatXChatRoleXProfile::ChatRoleId).big_integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(ChatXChatRoleXProfile::Table, ChatXChatRoleXProfile::ChatRoleId)
                            .to(ChatRole::Table, ChatRole::Id)
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(ChatXChatRoleXProfile::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum ChatXChatRoleXProfile {
    Table,
    Id,
    ChatId,
    ProfileId,
    ChatRoleId,
}
