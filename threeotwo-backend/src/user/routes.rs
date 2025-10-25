use crate::error::{Error, Result};

use axum::{routing::{get, post}, Json, Router};
use serde_json::{json, Value};

use crate::AppState;
pub fn routes(_app_state : AppState) -> Router {
    Router::new()
        .route("/create", post(create_user))
        .route("/update", post(update_user))
        .route("/read", get(read_user))
        .route("/delete", post(delete_user))
}

async fn create_user() -> Result<Json<Value>> {
    Ok(Json(json!({})))
}

async fn update_user() {
    todo!();
}

async fn read_user() {
    todo!();
}

async fn delete_user() {}

