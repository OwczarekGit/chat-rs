use axum::{Extension, response::sse::{Event, Sse}, Router};
use futures::stream::{Stream};
use std::{convert::Infallible, time::Duration};
use std::sync::Arc;
use axum::extract::State;
use axum::routing::get;
use tokio::sync::Mutex;
use crate::endpoints::account::UserAccount;
use crate::service::notification::NotificationService;

pub fn routes(
    notification_service: Arc<Mutex<NotificationService>>,
) -> Router {
    Router::new()
        .route("/subscribe", get(subscribe_to_notifications))
        .with_state(notification_service)
}

pub async fn subscribe_to_notifications(
    State(notification_service): State<Arc<Mutex<NotificationService>>>,
    Extension(user): Extension<UserAccount>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let stream = notification_service.lock().await.subscribe_to_notifications(user.id).await;

    Sse::new(stream)
        .keep_alive(
            axum::response::sse::KeepAlive::new()
                .interval(Duration::from_secs(1))
                .text("keep-alive")
        )
}