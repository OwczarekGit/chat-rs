use axum::{Extension, Json, Router};
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use crate::endpoints::account::UserAccount;
use crate::MessageAppState;
use crate::service::message::ChatMessage;

pub fn routes(
    state: MessageAppState
) -> Router {
    Router::new()
        .route("/:id", post(send_message))
        .route("/:id/all", get(get_all_for_chat))
        .with_state(state)
}

pub async fn send_message(
    State(state): State<MessageAppState>,
    Extension(user_account): Extension<UserAccount>,
    Path(chat_id): Path<i64>,
    Json(request): Json<SendMessageRequest>
) -> Result<impl IntoResponse, StatusCode> {
    let message = ChatMessage::from(state.message_service.send_message(chat_id, user_account.id, &request.message).await?);
    let json = serde_json::to_string(&message).expect("");
    state.notification_service.lock().await.send_notification(1, &json).await;
    Ok(())
}

pub async fn get_all_for_chat(
    State(_state): State<MessageAppState>,
    Path(_chat_id): Path<i64>,
) -> impl IntoResponse {

}

#[derive(Serialize, Deserialize, Debug)]
pub struct SendMessageRequest {
    pub message: String,
}
