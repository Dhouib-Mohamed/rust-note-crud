use std::sync::Arc;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use crate::AppState;
use crate::router::model::FilterOptions;
use crate::router::model::NoteModel;
use crate::utils::log::{debug, error};


pub async fn get_notes_handler(
    opts: Option<Query<FilterOptions>>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let Query(opts) = opts.unwrap_or_default();

    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let query_result = sqlx::query_as_unchecked!(
        NoteModel,
        "SELECT * FROM notes ORDER by id LIMIT $1 OFFSET $2",
        limit as i32,
        offset as i32
    )
        .fetch_all(&data.db)
        .await;

    if query_result.is_err() {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "Something bad happened while fetching all note items",
        });
        error("Something bad happened while fetching all note items");
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
    }

    let notes = query_result.unwrap();

    if notes.is_empty() {
        debug("No notes found");
    } else {
        let length = notes.len();
        debug(format!("{} notes fetched", length).as_str());
    }
    let json_response = serde_json::json!({
        "status": "success",
        "results": notes.len(),
        "notes": notes
    });
    Ok(Json(json_response))
}
