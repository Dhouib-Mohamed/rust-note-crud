use std::sync::Arc;
use serde::{Deserialize, Serialize};
use serde_json::json;
use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::IntoResponse,
};
use crate::AppState;
use crate::router::model::NoteModel;
use crate::utils::log::{debug, error};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateNoteSchema {
    pub title: String,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published: Option<bool>,
}

pub async fn create_note_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreateNoteSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as_unchecked!(
        NoteModel,
        "INSERT INTO notes (title,content,category) VALUES ($1, $2, $3) RETURNING *",
        body.title.to_string(),
        body.content.to_string(),
        body.category.to_owned().unwrap_or("".to_string())
    )
        .fetch_one(&data.db)
        .await;

    return match query_result {
        Ok(note) => {

            let note_response = json!({"status": "success","data": json!({
                "note": note
            })});

            debug(format!("Note with title {} created", note.title).as_str());

            Ok((StatusCode::CREATED, Json(note_response)))
        }
        Err(e) => {
            if e.to_string()
                .contains("duplicate key value violates unique constraint")
            {
                let error_response = serde_json::json!({
                    "status": "fail",
                    "message": "Note with that title already exists",
                });
                error(format!("Note with title {} already exists", body.title).as_str());
                return Err((StatusCode::CONFLICT, Json(error_response)));
            }
            error("Unexpected Error creating note");
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": format!("{:?}", e)})),
            ))
        }
    }
}