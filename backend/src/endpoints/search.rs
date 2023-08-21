use axum::response::IntoResponse;
use axum::{Extension, Json, Router};
use axum::extract::{Path, State};
use axum::routing::get;
use hyper::StatusCode;
use crate::AppState;
use crate::endpoints::account::UserAccount;
use crate::service::search::SearchService;

pub fn routes(service: AppState) -> Router {
    Router::new()
        .route("/profile/:search_string", get(search_users))
        .with_state(service)
}

pub async fn search_users (
    Extension(_user): Extension<UserAccount>,
    State(search_service): State<SearchService>,
    Path(search_string): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    Ok(Json(search_service.search_users(&search_string).await?))
}