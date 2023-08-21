use std::cmp::Ordering;
use axum::{Extension, Json, Router};
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use crate::endpoints::account::UserAccount;
use crate::AppState;
use crate::endpoints::notification::{AppNotification, AppNotificationType};
use crate::service::message::{ChatMessage, MessageService};
use crate::service::notification::NotificationService;

pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/:id", post(send_message))
        .route("/:id/all", get(get_all_for_chat))
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

    let json = serde_json::to_string(&notification).expect("");
    let _ = notification_service.notify_all_chat_members(chat_id, &json).await;
    Ok(())
}

pub async fn get_all_for_chat(
    State(message_service): State<MessageService>,
    Path(chat_id): Path<i64>,
) -> Result<impl IntoResponse, StatusCode> {
    let mut items = message_service.get_all_for_chat(chat_id)
        .await?
        .into_iter()
        .map(|e| ChatMessage::from(e))
        .collect::<Vec<_>>();

    items
        .sort_by(|a,b| {
            if a.created >= b.created {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        });
    Ok(Json(items))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SendMessageRequest {
    pub message: String,
}
