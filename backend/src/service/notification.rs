use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;
use axum::http::StatusCode;
use axum::response::sse::Event;
use axum_macros::FromRef;
use futures::{SinkExt};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use serde::Serialize;
use tokio::sync::{RwLock};
use crate::entities::{*, prelude::*};

#[derive(Clone, FromRef)]
pub struct NotificationService {
    db: DatabaseConnection,
    notification_channels: Arc<RwLock<HashMap<i64, futures::channel::mpsc::UnboundedSender<Result<Event, Infallible>>>>>,
}

impl NotificationService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self {
            db,
            notification_channels: Arc::default()
        }
    }

    pub async fn subscribe_to_notifications(&mut self, user_id: i64) -> futures::channel::mpsc::UnboundedReceiver<Result<Event, Infallible>> {
        let (tx,rx) = futures::channel::mpsc::unbounded::<Result<Event, Infallible>>();
        let _ = self.notification_channels.write().await.insert(user_id, tx);

        rx
    }

    pub async fn send_notification(&mut self, user_id: i64, message: impl Serialize + Clone) {
        if let Some(channel) = self.notification_channels.write().await.get_mut(&user_id) {
            if let Ok(event) = Event::default().json_data(message) {
                let _ = channel.send(Ok(event)).await;
            }
        }
    }

    pub async fn notify_all_chat_members(&mut self, chat_id: i64, message: impl Serialize + Clone) -> Result<(), StatusCode> {
        let chat = Chat::find_by_id(chat_id)
            .one(&self.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .ok_or(StatusCode::NOT_FOUND)?
            ;


        let chats = ChatXChatRoleXProfile::find()
            .filter(chat_x_chat_role_x_profile::Column::ChatId.eq(chat.id))
            .all(&self.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            ;

        for chat in chats {
            self.send_notification(chat.profile_id, message.clone()).await;
        }

        Ok(())
    }
}