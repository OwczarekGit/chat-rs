use axum::http::StatusCode;
use sea_orm::{ActiveValue, DatabaseConnection, EntityTrait, ModelTrait};
use serde::{Deserialize, Serialize};
use crate::entities::{prelude::*, *};
use crate::entities::message::Model;

#[derive(Clone)]
pub struct MessageService {
    db: DatabaseConnection
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ChatMessage {
    id: i64,
    message: String,
    author_id: i64,
    author_name: String,
}

impl From<(message::Model, profile::Model)> for ChatMessage {
    fn from(value: (Model, profile::Model)) -> Self {
        Self {
            id: value.0.id,
            message: value.0.content,
            author_id: value.1.id,
            author_name: value.1.username
        }
    }
}

impl MessageService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn send_message(&self, chat_id: i64, author_id: i64, message: &str) -> Result<(message::Model, profile::Model), StatusCode> {
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
            ..Default::default()
        };

        let msg = Message::insert(message)
            .exec_with_returning(&self.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        ;

        Ok((msg, author))
    }

    pub async fn get_all_for_chat(&self, chat_id: i64) -> Result<Vec<(message::Model, profile::Model)>, StatusCode> {
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

            results.push((message, author));
        }

        Ok(results)
    }
}