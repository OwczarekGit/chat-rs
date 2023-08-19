use std::env;
use std::sync::Arc;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::{Router, Extension, Json};
use axum::middleware::Next;
use axum::routing::get;
use axum_extra::extract::CookieJar;
use dotenvy::dotenv;
use endpoints::account::UserAccount;
use hyper::{StatusCode, Request};
use sea_orm::{DatabaseConnection, Database };
use tokio::sync::Mutex;
use service::account::AccountService;
use tower_http::cors::{Any, CorsLayer};
use crate::service::chat::ChatService;
use crate::service::message::MessageService;
use crate::service::notification::NotificationService;

mod entities;
mod service;
mod endpoints;

pub async fn establish_connection() -> DatabaseConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL to be set");
    let connection = Database::connect(&database_url).await.expect("Connection to be established");
    connection.clone()
}

#[derive(Clone)]
pub struct MessageAppState {
    pub message_service: MessageService,
    pub notification_service: Arc<Mutex<NotificationService>>,
}

#[tokio::main]
async fn main() {
    let connection = establish_connection().await;

    let account_service = AccountService::new(connection.clone());
    let chat_service = ChatService::new(connection.clone());
    let message_service = MessageService::new(connection.clone());
    let notification_service = Arc::new(Mutex::new(NotificationService::new()));

    let message_app_state = MessageAppState {
        message_service: message_service.clone(),
        notification_service: notification_service.clone()
    };

    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_origin(Any);

    let app = Router::new()
        .route("/api/me", get(print_user))
        .nest("/api/chat", endpoints::chat::routes(chat_service.clone()))
        .nest("/api/message", endpoints::message::routes(message_app_state))
        .nest("/api/notification", endpoints::notification::routes(notification_service.clone()))
        .layer(axum::middleware::from_fn_with_state(account_service.clone(), authorize_from_session_cookie))
        .nest("/api/account", endpoints::account::routes(account_service.clone()))
        .layer(cors)
        ;

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn print_user(
    Extension(user): Extension<UserAccount>,
) -> impl IntoResponse {
    Json(user)
}

async fn authorize_from_session_cookie<B>(
    State(service): State<AccountService>,
    cookies: CookieJar,
    request: Request<B>,
    next: Next<B>,
) -> Result<impl IntoResponse, StatusCode> {
    let session_id = cookies.get("JSESSIONID")
        .ok_or(StatusCode::BAD_REQUEST)?;

    let model = service.authorize_from_session_cookie(session_id.value()).await?;

    let mut response = request;
    response.extensions_mut().insert(model);
    Ok(next.run(response).await)
}