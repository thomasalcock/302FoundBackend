use crate::error::{Error, Result};

use axum::{extract::Path, routing::{delete, get, post, put}, Json, Router};
use serde_json::{json, Value};

use crate::AppState;
pub fn routes(_app_state : AppState) -> Router {
    Router::new()
        .route("/", post(create_user))
        .route("/:id", put(update_user))
        .route("/", get(read_users))
        .route("/:id", get(read_user))
        .route("/:id", delete(delete_user))
}

async fn create_user() -> Result<Json<Value>> {
    println!("HANDLER: CREATING USER");

    Ok(Json(json!({})))
}

async fn update_user(Path(id) : Path<String>) {
    todo!();
}

async fn read_users() {}

async fn read_user() {
    todo!();
}

async fn delete_user() {}

