//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.2

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "chat_role")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::chat_x_chat_role_x_profile::Entity")]
    ChatXChatRoleXProfile,
}

impl Related<super::chat_x_chat_role_x_profile::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ChatXChatRoleXProfile.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
