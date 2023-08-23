use axum_macros::FromRef;
use hyper::StatusCode;
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection, EntityTrait};
use serde::{Deserialize, Serialize};
use crate::entities::{*, prelude::*};
use crate::entities::profile::Model;

#[derive(Clone, FromRef)]
pub struct ProfileService {
    db: DatabaseConnection
}

impl ProfileService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn get_user_profile(&self, user_id: i64) -> Result<UserProfile, StatusCode> {
        let profile = Profile::find_by_id(user_id)
            .one(&self.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .ok_or(StatusCode::NOT_FOUND)?
            ;

        Ok(profile.into())
    }

    pub async fn change_username(&self, user_id: i64, new_username: &str) -> Result<(), StatusCode> {
        let mut profile: profile::ActiveModel = Profile::find_by_id(user_id)
            .one(&self.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .ok_or(StatusCode::BAD_REQUEST)?
            .into()
            ;

        profile.username = ActiveValue::Set(new_username.to_string());

        profile
            .update(&self.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        Ok(())
    }
}

impl From<profile::Model> for UserProfile {
    fn from(value: Model) -> Self {
        Self {
            id: value.id,
            username: value.username
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserProfile {
    id: i64,
    username: String,
}