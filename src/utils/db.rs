use sqlx::postgres::PgPoolOptions;
use crate::utils::env;
use crate::utils::log::{debug, info, panic};

pub async fn initialize_db() -> sqlx::Pool<sqlx::Postgres> {
    let database_url = env::get_database_url();
    debug(format!("Database URL: {}", database_url).as_str());
    match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            info("Connection to the database is successful!");
            pool
        }
        Err(_) => {
            panic("Failed to connect to the database")
        }
    }
}