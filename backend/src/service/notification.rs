use std::collections::HashMap;
use std::convert::Infallible;
use axum::response::sse::Event;
use futures::{SinkExt};

#[derive(Clone)]
pub struct NotificationService {
    notification_channels: HashMap<i64, futures::channel::mpsc::UnboundedSender<Result<Event, Infallible>>>
}

impl NotificationService {
    pub fn new() -> Self {
        Self {
            notification_channels: HashMap::new()
        }
    }

    pub async fn subscribe_to_notifications(&mut self, user_id: i64) -> futures::channel::mpsc::UnboundedReceiver<Result<Event, Infallible>> {
        let (tx,rx) = futures::channel::mpsc::unbounded::<Result<Event, Infallible>>();
        let _ = self.notification_channels.insert(user_id, tx);

        rx
    }

    pub async fn send_notification(&mut self, user_id: i64, message: &str) {
        if let Some(channel) = self.notification_channels.get_mut(&user_id) {
            let _ = channel.send(Event::default().data(message).try_into()).await;
        }
    }
}