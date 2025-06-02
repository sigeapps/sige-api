use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateBasicLookUpDTO {
    pub name: String,
    pub phone: Option<String>,
    pub state: Option<i32>,
    pub brand: Option<i32>,
    pub format: Option<String>,
}
