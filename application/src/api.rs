use sea_orm::DatabaseConnection;

use crate::auth::UserClaims;

#[derive(Debug, Clone)]
pub struct ApiContext {
    pub db: DatabaseConnection,
    pub claims: UserClaims,
}
