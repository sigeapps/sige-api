use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateBasicLookUpDTO {
    pub name: String,
}
