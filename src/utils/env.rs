use ctor::ctor;
use dotenv::dotenv;
use crate::utils::log::{panic,error};

#[ctor]
fn init() {
    dotenv().ok();
}

fn get_env(key: &str, default: &str) -> String {
    match std::env::var(key) {
        Ok(val) => val,
        Err(_) => {
            if default.is_empty() {
                panic(format!("{} is not set", key).as_str());
            }
            error(format!("{} is not set, using default value", key).as_str());
            default.to_string()
        }
    }
}
pub fn get_database_url() -> String {
    get_env("DATABASE_URL", "postgres://postgres:password@localhost:5432/notes")
}

pub fn get_port() -> String {
    get_env("PORT", "8000")
}