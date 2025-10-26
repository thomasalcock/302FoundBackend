use crate::location::location::{LocationForCreate, LocationStore};
use crate::error::{Error, Result};
use crate::app_state::AppState;

use axum::{Json, Router};
use axum::extract::{Path,State};
use axum::routing::{delete,get,post};

use serde_json::{json, Value};



pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", post(create_location))
        .route("/", get(get_locations))
        .route("/{id}", get(get_location))
        .route("/{id}", delete(delete_location))
}

async fn create_location(State(app_state): State<AppState>, Json(payload): Json<LocationForCreate>) -> Result<Json<Value>> {
    println!("ADDING LOCATION");
    app_state.create_location(payload).await?;
    
    Ok(Json(json!({"status": {"success": true}})))
}

async fn get_location(State(app_state): State<AppState>, Path(id): Path<u64>) -> Result<Json<Value>> {
    println!("GETTING LOCATION");
    let location = app_state.location_by_id(id).await?;
    
    Ok(Json(serde_json::to_value(location).map_err(|_| Error::DatabaseError)?))
}

async fn get_locations(State(app_state): State<AppState>) -> Result<Json<Value>> {
    println!("GETTING LOCATIONS");
    let locations = app_state.get_location().await?;
    
    Ok(Json(serde_json::to_value(locations).map_err(|_| Error::DatabaseError)?))
}

async fn delete_location(State(app_state): State<AppState>, Path(id): Path<u64>) -> Result<Json<Value>> {
    println!("DELETING LOCATION");
    app_state.delete_location(id).await?;

    Ok(Json(json!({"status": {"success": true}})))
}


