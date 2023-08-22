use axum::response::IntoResponse;
use axum::{Extension, Json, Router};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::{get, post, put};
use serde::{Deserialize, Serialize};
use crate::AppState;
use crate::endpoints::account::UserAccount;
use crate::endpoints::notification::{AppNotification, AppNotificationType};
use crate::service::chat::ChatService;
use crate::service::notification::NotificationService;

pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/create", post(create_chat))
        .route("/list", get(get_all_chats))
        .route("/invite", post(invite_to_chat))
        .route("/:id/name", put(change_chat_name))
        .with_state(state)
}

pub async fn change_chat_name(
    Extension(user): Extension<UserAccount>,
    State(service): State<ChatService>,
    Path(chat_id): Path<i64>,
    Json(request): Json<ChangeChatNameRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    service.change_chat_name(user.id, chat_id, &request.name).await
}

pub async fn create_chat(
    Extension(user): Extension<UserAccount>,
    State(service): State<ChatService>,
    Json(request): Json<CreateChatRequest>
) -> impl IntoResponse {
    service.create_chat(user.id, &request.name).await
}

pub async fn get_all_chats(
    Extension(user): Extension<UserAccount>,
    State(service): State<ChatService>,
) -> Result<impl IntoResponse, StatusCode>{
    Ok(Json(service.get_chats_for_user(user.id).await?))
}

pub async fn invite_to_chat(
    Extension(user): Extension<UserAccount>,
    State(chat_service): State<ChatService>,
    State(mut notification_service): State<NotificationService>,
    Json(request): Json<InviteToChatRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    chat_service.invite_to_chat(user.id, request.user_id, request.chat_id).await?;

    let message = AppNotification {
        notification_type: AppNotificationType::ChatInvitation,
        body: {}
    };

    notification_service.send_notification(request.user_id, message).await;

    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InviteToChatRequest {
    pub chat_id: i64,
    pub user_id: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateChatRequest {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChangeChatNameRequest {
    pub name: String,
}
