use moka::future::Cache;
use sea_orm::DatabaseConnection;
use serde_json::Value;

use crate::auth::UserClaims;

#[derive(Debug, Clone)]
pub struct ApiContext {
    pub db: DatabaseConnection,
    pub claims: Option<UserClaims>,
    pub cache: Cache<String, Value>,
}
