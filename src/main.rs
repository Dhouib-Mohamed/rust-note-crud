mod router {
    pub mod _index;
    pub mod model;
    pub mod handlers {
        pub mod health;
        pub mod create_note;
        pub mod get_notes;
    }
}
mod utils {
    pub mod env;
    pub mod db;
    pub mod log;
}

use utils::db::initialize_db;
use router::_index::create_router;

use std::sync::Arc;
use sqlx::{Pool, Postgres};
use utils::log::info;

pub struct AppState {
    db: Pool<Postgres>,
}

#[tokio::main]
async fn main() {
    let db = initialize_db().await;

    create_router(Arc::new(AppState { db: db.clone() })).await;

    info("🚀 Server started successfully");
}