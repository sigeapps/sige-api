use application::auth::UserClaims;
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use sha2::Sha256;

pub trait JwtTrait {
    fn to_jwt(&self) -> Result<String, jwt::Error>;
    fn from_jwt(jwt: String) -> Result<Self, jwt::Error>
    where
        Self: Sized;
}

impl JwtTrait for UserClaims {
    fn to_jwt(&self) -> Result<String, jwt::Error> {
        let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

        let hmac: Hmac<Sha256> = Hmac::new_from_slice(secret.as_bytes())?;

        let token = self.sign_with_key(&hmac)?;

        Ok(token)
    }

    fn from_jwt(jwt: String) -> Result<Self, jwt::Error> {
        let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

        let hmac: Hmac<Sha256> = Hmac::new_from_slice(secret.as_bytes())?;

        let user = jwt.verify_with_key(&hmac)?;

        Ok(user)
    }
}
