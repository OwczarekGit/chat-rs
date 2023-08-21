use axum::{Extension, response::sse::{Event, Sse}, Router};
use futures::stream::{Stream};
use std::{convert::Infallible, time::Duration};
use axum::extract::State;
use axum::routing::get;
use crate::AppState;
use crate::endpoints::account::UserAccount;
use crate::service::notification::NotificationService;

pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/subscribe", get(subscribe_to_notifications))
        .with_state(state)
}

pub async fn subscribe_to_notifications(
    State(mut notification_service): State<NotificationService>,
    Extension(user): Extension<UserAccount>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let stream = notification_service.subscribe_to_notifications(user.id).await;

    Sse::new(stream)
        .keep_alive(
            axum::response::sse::KeepAlive::new()
                .interval(Duration::from_secs(1))
                .text("keep-alive")
        )
}