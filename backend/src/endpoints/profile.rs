use axum::response::IntoResponse;
use axum::{Extension, Json, Router};
use axum::routing::get;
use crate::endpoints::account::UserAccount;

pub fn routes() -> Router {
    Router::new()
        .route("/me", get(get_my_profile))
}

pub async fn get_my_profile (
    Extension(user): Extension<UserAccount>,
) -> impl IntoResponse {
    Json(user)
}