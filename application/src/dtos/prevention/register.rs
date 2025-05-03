use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegisterExit {
    pub observations: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CreateRegister {
    pub photo: Option<String>,
    pub ci: String,
    pub last_name: String,
    pub first_name: String,
    pub organism: Option<i32>,
    pub division: Option<i32>,
    pub is_official: Option<bool>,
    pub visit_reason: String,
    pub observations: Option<String>,
}
