use crate::trust::TrustStore;
use crate::{app_state::AppState, trust::Trust};
use crate::error::{Error, Result};

use axum::{extract::State, routing::{delete, get, post}, Json, Router};
use serde_json::{json, Value};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", post(create_trust))
        .route("/", get(get_trusts))
        .route("/", delete(delete_trust))
}

async fn create_trust(State(app_state) : State<AppState>, Json(payload): Json<Trust>) -> Result<Json<Value>> {
    println!("ADDING TRUST");
    app_state.create_trust(payload.user(), payload.trustee()).await?;
    Ok(Json(json!({"status": {"success": true}})))
}

async fn get_trusts(State(app_state) : State<AppState>) -> Result<Json<Value>> {
    println!("READING ALL TRUSTS");
    let trusts = app_state.get_trusts().await?;
    
    Ok(Json(serde_json::to_value(trusts).map_err(|_| Error::DatabaseError)?))
}

async fn delete_trust(State(app_state) : State<AppState>, Json(payload): Json<Trust>) -> Result<Json<Value>> {
    println!("DELETING TRUST");
    app_state.delete_trust(payload.user(), payload.trustee()).await?;
    Ok(Json(json!({"status": {"success": true}})))
}


