use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LookupItemDto {
    pub id: i32,
    pub name: String,
}
