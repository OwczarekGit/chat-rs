use axum::http::StatusCode;
use sea_orm::{ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter};
use serde::{Deserialize, Serialize};

use crate::entities::{prelude::*, *};

#[derive(Clone)]
pub struct ChatService {
    db: DatabaseConnection,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserChat {
    pub id: i64,
    pub name: String,
}

impl From<chat::Model> for UserChat {
    fn from(value: chat::Model) -> Self {
        Self {
            id: value.id,
            name: value.name.to_string(),
        }
    }
}

impl ChatService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self{ db }
    }

    pub async fn get_chats_for_user(&self, user_id: i64) -> Result<Vec<UserChat>, StatusCode> {
        let chat_x_models = Profile::find_by_id(user_id)
            .one(&self.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .ok_or(StatusCode::NOT_FOUND)?
            .find_related(ChatXChatRoleXProfile)
            .all(&self.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            ;

        let mut chats = vec![];
        for cxm in chat_x_models {
            let chat = cxm.find_related(Chat)
                .one(&self.db)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
                .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?
            ;
            chats.push(chat);
        }

        Ok(
            chats.iter()
                .map(|c| UserChat::from(c.clone()))
                .collect::<Vec<_>>()
        )
    }

    pub async fn create_chat(&self, creator_id: i64, name: &str) -> Result<(), StatusCode> {
        let creator = Profile::find_by_id(creator_id)
            .one(&self.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .ok_or(StatusCode::NOT_FOUND)?
            ;

        let role = ChatRole::find()
            .filter(chat_role::Column::Name.eq("Admin"))
            .one(&self.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .ok_or(StatusCode::NOT_FOUND)?
            ;

        let chat_model = chat::ActiveModel {
            name: ActiveValue::Set(name.to_string()),
            ..Default::default()
        };

        let chat_id = Chat::insert(chat_model)
            .exec(&self.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .last_insert_id
            ;

        let chat_x_model = chat_x_chat_role_x_profile::ActiveModel {
            chat_id: ActiveValue::Set(chat_id),
            chat_role_id: ActiveValue::Set(role.id),
            profile_id: ActiveValue::Set(creator.id),
            ..Default::default()
        };

        ChatXChatRoleXProfile::insert(chat_x_model)
            .exec(&self.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            ;

        Ok(())
    }
}