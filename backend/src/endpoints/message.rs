use axum::{Extension, Json, Router};
use axum::extract::{Path, Query, State};
use axum::response::IntoResponse;
use axum::routing::post;
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use crate::endpoints::account::UserAccount;
use crate::AppState;
use crate::endpoints::notification::{AppNotification, AppNotificationType};
use crate::service::message::{ChatMessage, MessageService};
use crate::service::notification::NotificationService;

pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/:id", post(send_message).get(get_messages_paginated))
        .with_state(state)
}

pub async fn send_message(
    State(message_service): State<MessageService>,
    State(mut notification_service): State<NotificationService>,
    Extension(user_account): Extension<UserAccount>,
    Path(chat_id): Path<i64>,
    Json(request): Json<SendMessageRequest>
) -> Result<impl IntoResponse, StatusCode> {
    let message = ChatMessage::from(message_service.send_message(chat_id, user_account.id, &request.message).await?);

    let notification = AppNotification {
        notification_type: AppNotificationType::ChatMessage,
        body: message
    };

    let _ = notification_service.notify_all_chat_members(chat_id, notification).await;
    Ok(())
}

pub async fn get_messages_paginated(
    Extension(user): Extension<UserAccount>,
    State(message_service): State<MessageService>,
    pagination: Query<Pagination>,
    Path(chat_id): Path<i64>,
) -> Result<impl IntoResponse, StatusCode> {
    let (messages, chat_id) = message_service.get_for_chat_paginated(user.id, chat_id, pagination.offset, pagination.count).await?;

    Ok(
        Json(
            messages.into_iter()
                .map(|e|{
                    ChatMessage::from((e, chat_id))
                })
                .rev()
                .collect::<Vec<_>>()
        )
    )
}


#[derive(Serialize, Deserialize, Debug)]
pub struct SendMessageRequest {
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Pagination {
    offset: u64,
    count: u64,
}
