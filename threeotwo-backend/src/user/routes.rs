use crate::{error::{Error, Result}, user::user::{User, UserForCreate, UserStore}};

use axum::{extract::{Path, State}, routing::{delete, get, post, put}, Json, Router};
use serde_json::{json, Value};


use crate::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", post(create_user))
        .route("/{id}", put(update_user))
        .route("/", get(read_users))
        .route("/{id}", get(read_user))
        .route("/{id}", delete(delete_user))
        .route("/{id}/trustees", get(my_trustees))
}

async fn create_user(State(mut app_state) : State<AppState>, Json(user): Json<UserForCreate>) -> Result<Json<Value>> {
    app_state.create_user(user).await?;
    println!("HANDLER: CREATING USER");

    Ok(Json(json!({})))
}

async fn update_user(
    State(mut app_state) : State<AppState>,
    Path(id) : Path<String>,
    Json(user_for_update) : Json<User>) -> Result<Json<Value>> {
    app_state.update_user(id.parse().map_err(|_| Error::UnknownError)?, user_for_update).await?;
    
    Ok(Json(json!({
        "result" : {
            "success" : true
        }
    })))
}

async fn read_users(State(app_state): State<AppState>) -> Result<Json<Value>>{
    println!("HANDLER: READ ALL USERS");
    let results = app_state.users().await?;

    Ok(Json(serde_json::to_value(&results).map_err(|_| Error::UnknownError)?))
}

async fn read_user(State(app_state): State<AppState>, Path(id): Path<String>) -> Result<Json<Value>>{
    println!("HANDLER: READ USER BY ID {}", id);
    let id = id.parse().map_err(|_| Error::UnknownError)?;
    let results = app_state.user_by_id(id).await?;

    Ok(Json(serde_json::to_value(&results).map_err(|_| Error::UnknownError)?))
}

async fn my_trustees(State(app_state): State<AppState>, Path(id): Path<String>) -> Result<Json<Value>>{
    println!("HANDLER: READ USERS TRUSTED BY ID {}", id);
    let id = id.parse().map_err(|_| Error::UnknownError)?;
    let results = app_state.my_trustees(id).await?;

    Ok(Json(serde_json::to_value(&results).map_err(|_| Error::UnknownError)?))
}


async fn delete_user() {}

