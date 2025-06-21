use application::dtos::user::GetUserDTO;
use domain::auth::permissions::Permission;
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use serde::{Deserialize, Serialize};
use sha2::Sha256;

#[derive(Serialize, Deserialize, Clone)]
pub struct AuthClaims {
    pub user: GetUserDTO,
    pub permissions: Vec<Permission>,
}

impl AuthClaims {
    pub fn to_jwt(&self) -> Result<String, jwt::Error> {
        let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

        let hmac: Hmac<Sha256> = Hmac::new_from_slice(secret.as_bytes())?;

        let token = self.sign_with_key(&hmac)?;

        Ok(token)
    }

    pub fn from_jwt(jwt: String) -> Result<Self, jwt::Error> {
        let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

        let hmac: Hmac<Sha256> = Hmac::new_from_slice(secret.as_bytes())?;

        let user = jwt.verify_with_key(&hmac)?;

        Ok(user)
    }
}
