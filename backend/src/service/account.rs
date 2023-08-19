use axum::response::IntoResponse;
use axum_extra::extract::{CookieJar, cookie::Cookie};
use chrono::Utc;
use hyper::StatusCode;
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, ActiveValue, ModelTrait};
use crate::{entities::{*, prelude::*}, endpoints::account::UserAccount};

#[derive(Clone)]
pub struct AccountService {
    db: DatabaseConnection,
}

impl AccountService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn register_user(&self, email: &str, password: &str) -> Result<impl IntoResponse, StatusCode> {
        let email_taken = Account::find()
            .filter(account::Column::Email.eq(email))
            .all(&self.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .len()
            .gt(&0usize)
            ;

        if email_taken {
            return Err(StatusCode::BAD_REQUEST);
        }

        let model = account::ActiveModel {
            email: sea_orm::ActiveValue::Set(email.to_string()),
            password: sea_orm::ActiveValue::Set(encode_password(password, 8)),
            joined: ActiveValue::Set(Utc::now().naive_utc()),
            ..Default::default()
        };

        let id = Account::insert(model)
            .exec(&self.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .last_insert_id;

        let model = profile::ActiveModel {
            id: ActiveValue::Set(id),
            username: ActiveValue::Set("Username".to_string())
        };

        Profile::insert(model)
            .exec(&self.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        Ok(StatusCode::CREATED)
    }

    pub async fn login_user(&self, email: &str, password: &str) -> Result<CookieJar, StatusCode> {

        let account = Account::find()
            .filter(account::Column::Email.eq(email))
            .one(&self.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .ok_or(StatusCode::UNAUTHORIZED)?;

        if !verify_encrypted_password(password, &account.password) {
            return Err(StatusCode::UNAUTHORIZED);
        }

        self.create_session(account.id).await
    }

    pub async fn authorize_from_session_cookie(&self, session_id: &str) -> Result<UserAccount, StatusCode> {
        let account = Session::find()
            .filter(session::Column::Key.eq(session_id))
            .one(&self.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .ok_or(StatusCode::UNAUTHORIZED)?
            .find_related(Account).one(&self.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .ok_or(StatusCode::UNAUTHORIZED)?
            ;

        Ok(account.into())
    }

    async fn create_session(&self, user_id: i64) -> Result<CookieJar, StatusCode> {
        let jar = CookieJar::new();

        let model = session::ActiveModel {
            account_id: ActiveValue::Set(user_id),
            key: ActiveValue::Set(uuid::Uuid::new_v4().to_string()),
            ..Default::default()
        };

        let session = Session::insert(model)
            .exec_with_returning(&self.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            ;

        let cookie = Cookie::build("JSESSIONID", session.key)
            .http_only(true)
            .path("/")
            .expires(None)
            .finish();
        
        Ok(jar.add(cookie))
    }
}

pub fn encode_password(password: &str, cost: u32) -> String {
    bcrypt::hash(password, cost).expect("To encrypt correctly")
} 

pub fn verify_encrypted_password(password: &str, hash: &str) -> bool {
    bcrypt::verify(password, hash).expect("To encrypt correctly")
} 