use std::env;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::{Router};
use axum::middleware::Next;
use axum_extra::extract::CookieJar;
use axum_macros::FromRef;
use dotenvy::dotenv;
use hyper::{StatusCode, Request};
use sea_orm::{DatabaseConnection, Database };
use service::account::AccountService;
use tower_http::cors::{Any, CorsLayer};
use crate::service::chat::ChatService;
use crate::service::message::MessageService;
use crate::service::notification::NotificationService;
use crate::service::search::SearchService;

mod entities;
mod service;
mod endpoints;

pub async fn establish_connection() -> DatabaseConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL to be set");
    let connection = Database::connect(&database_url).await.expect("Connection to be established");
    connection.clone()
}

#[derive(Clone, FromRef)]
pub struct AppState {
    account_service: AccountService,
    chat_service: ChatService,
    message_service: MessageService,
    search_service:  SearchService,
    notification_service: NotificationService,
}

#[tokio::main]
async fn main() {
    let connection = establish_connection().await;

    let app_state = AppState {
        account_service: AccountService::new(connection.clone()),
        chat_service: ChatService::new(connection.clone()),
        message_service: MessageService::new(connection.clone()),
        notification_service: NotificationService::new(connection.clone()),
        search_service: SearchService::new(connection.clone()),
    };

    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_origin(Any);

    let app = Router::new()
        .nest("/api/profile", endpoints::profile::routes())
        .nest("/api/search", endpoints::search::routes(app_state.clone()))
        .nest("/api/chat", endpoints::chat::routes(app_state.clone()))
        .nest("/api/message", endpoints::message::routes(app_state.clone()))
        .nest("/api/notification", endpoints::notification::routes(app_state.clone()))
        .layer(axum::middleware::from_fn_with_state(app_state.account_service.clone(), authorize_from_session_cookie))
        .nest("/api/account", endpoints::account::routes(app_state.clone()))
        .layer(cors)
        ;

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
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
