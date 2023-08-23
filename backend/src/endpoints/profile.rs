use axum::response::IntoResponse;
use axum::{Extension, Json, Router};
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::{get, put};
use serde::{Deserialize, Serialize};
use crate::AppState;
use crate::endpoints::account::UserAccount;
use crate::service::profile::{ProfileService};

pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/me", get(get_my_profile))
        .route("/", put(change_username))
        .with_state(state)
}

pub async fn get_my_profile (
    Extension(user): Extension<UserAccount>,
    State(profile_service): State<ProfileService>
) -> Result<impl IntoResponse, StatusCode> {
    Ok(Json(profile_service.get_user_profile(user.id).await?))
}

pub async fn change_username (
    Extension(user): Extension<UserAccount>,
    State(profile_service): State<ProfileService>,
    Json(request): Json<ChangeUsernameRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    Ok(Json(profile_service.change_username(user.id, &request.username).await?))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChangeUsernameRequest {
    pub username: String,
}
