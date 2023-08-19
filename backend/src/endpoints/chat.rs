use axum::response::IntoResponse;
use axum::{Extension, Json, Router};
use axum::extract::State;
use axum::routing::{get, post};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use crate::endpoints::account::UserAccount;
use crate::service::chat::ChatService;

pub fn routes(state: ChatService) -> Router {
    Router::new()
        .route("/create", post(create_chat))
        .route("/list", get(get_all_chats))
        .with_state(state)
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

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateChatRequest {
    pub name: String,
}