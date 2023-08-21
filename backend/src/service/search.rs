use axum::http::StatusCode;
use axum_macros::FromRef;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use crate::{entities::{*, prelude::*}};

#[derive(Clone, FromRef)]
pub struct SearchService {
    db: DatabaseConnection
}

impl SearchService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn search_users(&self, search_string: &str) -> Result<Vec<profile::Model>, StatusCode> {
        Profile::find()
            .filter(profile::Column::Username.contains(search_string))
            .all(&self.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
    }
}