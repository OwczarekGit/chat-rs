use sea_orm_migration::prelude::*;
use sea_orm_migration::prelude::extension::postgres::Type;
use crate::sea_orm::{EnumIter, Iterable};

use super::m20230819_115126_create_profile_table::Profile;
use super::m20230818_222729_create_chat_table::Chat;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        let _ = manager
            .create_type(
                Type::create()
                    .as_enum(ChatRole::Table)
                    .values([ChatRole::Member, ChatRole::Admin])
                    .to_owned()
            ).await;

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
                    .col(ColumnDef::new(ChatXChatRoleXProfile::ChatRole)
                        .enumeration(ChatRole::Table, [ChatRole::Member, ChatRole::Admin])
                        .not_null()
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
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(ChatXChatRoleXProfile::Table).to_owned())
            .await

        // manager
        //     .drop_type(Type::drop().name("ChatRole").to_owned())
        //     .await
    }
}

#[derive(DeriveIden)]
pub enum ChatXChatRoleXProfile {
    Table,
    Id,
    ChatRole,
    ChatId,
    ProfileId,
}

#[derive(Iden, EnumIter)]
pub enum ChatRole {
    Table,
    #[iden = "Member"]
    Member,
    #[iden = "Admin"]
    Admin
}
