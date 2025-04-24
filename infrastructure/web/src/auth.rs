use application::dtos::auth::LoginRequest;
use async_trait::async_trait;
use axum_login::{AuthUser, AuthnBackend, UserId};
use domain::{
    entities::user, error::AuthError as DomainAuthError,
    repositories::user_repository::UserRepository,
};
use password_auth::verify_password;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::Arc;
use tracing::debug;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    #[serde(flatten)]
    model: user::Model,
}

impl Deref for User {
    type Target = user::Model;

    fn deref(&self) -> &Self::Target {
        &self.model
    }
}

impl AuthUser for User {
    type Id = i32;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.password_hash.as_bytes()
    }
}

#[derive(Debug, Clone)]
pub struct AuthBackend<R: UserRepository + Clone> {
    pub user_repository: Arc<R>,
}

impl<R: UserRepository + Clone> AuthBackend<R> {
    pub fn new(user_repository: Arc<R>) -> Self {
        Self { user_repository }
    }
}

#[async_trait]
impl<R> AuthnBackend for AuthBackend<R>
where
    R: UserRepository + Send + Sync + Clone + 'static,
{
    type User = User;
    type Credentials = LoginRequest;
    type Error = DomainAuthError;

    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        let user = self
            .user_repository
            .find_by_username(creds.username)
            .await?;

        match user {
            Some(u) => {
                if verify_password(creds.password, &u.password_hash).is_ok() {
                    Ok(Some(User { model: u }))
                } else {
                    debug!("Invalid password for user");

                    Ok(None)
                }
            }
            None => {
                debug!("User not found");

                Ok(None)
            }
        }
    }

    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        let user = self.user_repository.find_by_id(*user_id).await?;

        Ok(user.map(|u| User { model: u }))
    }
}

pub type AuthSession<R> = axum_login::AuthSession<AuthBackend<R>>;
