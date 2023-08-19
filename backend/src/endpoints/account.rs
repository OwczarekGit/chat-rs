use axum::{Router, Json, extract::State, response::IntoResponse, routing::post};
use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};

use crate::{entities::account, service::account::AccountService};

pub fn routes(state: AccountService) -> Router {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .with_state(state)
}

pub async fn register (
    State(service): axum::extract::State<AccountService>,
    Json(request): Json<RegistrationRequest>,
) -> impl IntoResponse {
    service.register_user(&request.email, &request.password).await
}

pub async fn login (
    State(service): State<AccountService>,
    Json(request): Json<LoginRequest>,
) -> impl IntoResponse {
    service.login_user(&request.email, &request.password).await
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RegistrationRequest {
    email: String,
    password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserAccount {
    pub id: i64,
    pub email: String,
    pub joined: NaiveDateTime,
}

impl From<account::Model> for UserAccount {
    fn from(value: account::Model) -> Self {
        Self { id: value.id, email: value.email.to_owned(), joined: value.joined }
    }
}
