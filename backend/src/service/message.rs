use axum::http::StatusCode;
use axum_macros::FromRef;
use chrono::{NaiveDateTime, Utc};
use sea_orm::{ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter, QueryOrder, QuerySelect};
use serde::{Deserialize, Serialize};
use crate::entities::{prelude::*, *};
use crate::entities::message::Model;

#[derive(Clone, FromRef)]
pub struct MessageService {
    db: DatabaseConnection
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChatMessage {
    pub id: i64,
    pub message: String,
    pub author_id: i64,
    pub author_name: String,
    pub chat_id: i64,
    pub created: NaiveDateTime
}

impl From<((message::Model, Option<profile::Model>), i64)> for ChatMessage {
    fn from(value: ((Model, Option<profile::Model>), i64)) -> Self {
        let profile = value.0.1.expect("Profile");

        Self {
            id: value.0.0.id,
            created: value.0.0.created,
            message: value.0.0.content,
            author_id: profile.id,
            author_name: profile.username,
            chat_id: value.1
        }
    }
}

impl From<(message::Model, profile::Model, chat::Model)> for ChatMessage {
    fn from(value: (Model, profile::Model, chat::Model)) -> Self {
        Self {
            id: value.0.id,
            message: value.0.content,
            author_id: value.1.id,
            author_name: value.1.username,
            chat_id: value.2.id,
            created: value.0.created,
        }
    }
}

impl MessageService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn send_message(&self, chat_id: i64, author_id: i64, message: &str)
        -> Result<(message::Model, profile::Model, chat::Model), StatusCode> {
        let chat = Chat::find_by_id(chat_id)
            .one(&self.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .ok_or(StatusCode::NOT_FOUND)?
        ;

        let author = Profile::find_by_id(author_id)
            .one(&self.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .ok_or(StatusCode::NOT_FOUND)?
        ;

        let message = message::ActiveModel {
            chat_id: ActiveValue::Set(chat.id),
            profile_id: ActiveValue::Set(author.id),
            content: ActiveValue::Set(message.to_string()),
            created: ActiveValue::Set(Utc::now().naive_utc()),
            ..Default::default()
        };

        let msg = Message::insert(message)
            .exec_with_returning(&self.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        ;

        Ok((msg, author, chat))
    }

    pub async fn get_all_for_chat(&self, chat_id: i64) -> Result<Vec<(message::Model, profile::Model, chat::Model)>, StatusCode> {
        let chat = Chat::find_by_id(chat_id)
            .one(&self.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .ok_or(StatusCode::NOT_FOUND)?
        ;

        let messages = chat.find_related(Message)
            .all(&self.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        ;

        let mut results = vec![];
        for message in messages {
            let author = message.find_related(Profile)
                .one(&self.db)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
                .ok_or(StatusCode::NOT_FOUND)?
            ;

            results.push((message, author, chat.clone()));
        }

        Ok(results)
    }

    pub async fn get_for_chat_paginated(&self, user_id: i64, chat_id: i64, offset: u64, count: u64)
        -> Result<( Vec<(message::Model, Option<profile::Model>)>, i64 ), StatusCode> {
        let _ = Chat::find_by_id(chat_id)
            .one(&self.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .ok_or(StatusCode::BAD_REQUEST)?
            .find_related(ChatXChatRoleXProfile)
            .filter(chat_x_chat_role_x_profile::Column::ProfileId.eq(user_id))
            .one(&self.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .ok_or(StatusCode::UNAUTHORIZED)?
        ;

        let messages: Vec<(message::Model, Option<profile::Model>)> = Message::find()
            .find_also_related(Profile)
            .filter(message::Column::ChatId.eq(chat_id))
            .order_by_desc(message::Column::Created)
            .offset(offset)
            .limit(count)
            .all(&self.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            ;

        Ok((messages, chat_id))
    }
}