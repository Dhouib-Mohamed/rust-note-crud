use std::sync::Arc;
use axum::{
    routing::{get, post},
    http::{
        Method,
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    },
    Router,
};
use tower_http::cors::CorsLayer;

use crate::{
    AppState,
    utils::env,
    router::handlers::{
        health::health_handler,
        create_note::create_note_handler,
        get_notes::get_notes_handler,
    },
};
use crate::utils::log::{info, panic};

pub async fn create_router(app_state: Arc<AppState>) {
    let cors = CorsLayer::new()
        // .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let router = Router::new()
        .route("/health", get(health_handler))
        .route("/note", post(create_note_handler))
        .route("/note", get(get_notes_handler))
        .with_state(app_state)
        .layer(cors);

    let port = env::get_port();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:".to_owned() + &*port).await.unwrap();
    match axum::serve(listener, router).await {
        Ok(_) => {
            info(&*("🚀 Server started successfully in port ".to_owned() + &*port))
        }
        Err(_) => {
            panic("Server failed to start");
        }
    }
}