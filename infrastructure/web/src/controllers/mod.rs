pub mod auth;
pub mod lookup;
pub mod parking;
pub mod personal;
pub mod prevention;
pub mod transport;
pub mod user;

pub fn get_modules() -> Vec<String> {
    vec![
        "module:personal".to_string(),
        "module:prevention".to_string(),
        "module:users".to_string(),
    ]
}
