use application::dtos::auth::LoginRequest;
use application::dtos::user::GetUserDTO;
use application::services::user::UserService;
use async_trait::async_trait;
use axum_login::{AuthUser, AuthnBackend, UserId};
use password_auth::verify_password;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use tracing::debug;

use crate::error::WebError;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    #[serde(flatten)]
    model: GetUserDTO,
}

impl Deref for User {
    type Target = GetUserDTO;

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
pub struct Backend {
    pub users: UserService,
}

#[async_trait]
impl AuthnBackend for Backend {
    type User = User;
    type Credentials = LoginRequest;
    type Error = WebError;

    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        let user = self.users.find_by_username(creds.username).await?;

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
        let user = self.users.find_by_id(*user_id).await?;

        Ok(user.map(|u| User { model: u }))
    }
}

pub type AuthSession = axum_login::AuthSession<Backend>;
