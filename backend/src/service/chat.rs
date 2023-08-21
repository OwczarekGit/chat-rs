use axum::http::StatusCode;
use axum_macros::FromRef;
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter};
use serde::{Deserialize, Serialize};

use crate::entities::{prelude::*, *};

#[derive(Clone, FromRef)]
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

    pub async fn change_chat_name(&self, user_id: i64, chat_id: i64, name: &str) -> Result<(), StatusCode> {
        let cx = ChatXChatRoleXProfile::find()
            .filter(chat_x_chat_role_x_profile::Column::ChatId.eq(chat_id))
            .filter(chat_x_chat_role_x_profile::Column::ProfileId.eq(user_id))
            .one(&self.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .ok_or(StatusCode::BAD_REQUEST)?
        ;

        let user_role = ChatRole::find()
            .filter(chat_role::Column::Id.eq(cx.chat_role_id))
            .one(&self.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .ok_or(StatusCode::BAD_REQUEST)?
        ;

        if !user_role.name.eq("Admin") {
            return Err(StatusCode::UNAUTHORIZED);
        }

        let mut chat: chat::ActiveModel = Chat::find_by_id(chat_id)
            .one(&self.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .ok_or(StatusCode::BAD_REQUEST)?
            .into()
        ;

        chat.name = ActiveValue::Set(name.to_string());
        let _ = chat.update(&self.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        ;

        Ok(())
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

    pub async fn invite_to_chat(&self, inviter_id: i64, user_id: i64, chat_id: i64) ->  Result<(), StatusCode> {
        let cx = ChatXChatRoleXProfile::find()
            .filter(chat_x_chat_role_x_profile::Column::ChatId.eq(chat_id))
            .filter(chat_x_chat_role_x_profile::Column::ProfileId.eq(inviter_id))
            .one(&self.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .ok_or(StatusCode::BAD_REQUEST)?
        ;

        let already_member = ChatXChatRoleXProfile::find()
            .filter(chat_x_chat_role_x_profile::Column::ChatId.eq(chat_id))
            .filter(chat_x_chat_role_x_profile::Column::ProfileId.eq(user_id))
            .one(&self.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        if already_member.is_some() {
            return Err(StatusCode::BAD_REQUEST);
        }

        let cx = chat_x_chat_role_x_profile::ActiveModel {
            chat_id: ActiveValue::Set(cx.chat_id),
            profile_id: ActiveValue::Set(user_id),
            chat_role_id: ActiveValue::Set(2),
            ..Default::default()
        };

        ChatXChatRoleXProfile::insert(cx)
            .exec(&self.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        Ok(())
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