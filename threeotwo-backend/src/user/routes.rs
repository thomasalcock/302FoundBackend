use crate::{error::Result, user::user::{UserForCreate, UserStore}};

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
}

async fn create_user(State(mut app_state) : State<AppState>, Json(user): Json<UserForCreate>) -> Result<Json<Value>> {
    app_state.create_user(user).await?;
    println!("HANDLER: CREATING USER");

    Ok(Json(json!({})))
}

async fn update_user(Path(_id) : Path<String>) {
    todo!();
}

async fn read_users() -> Result<Json<Value>>{
    println!("HANDLER: READ ALL USERS");
    Ok(Json(json!({"status": "works"})))
}

async fn read_user(Path(_id): Path<String>) -> Result<Json<Value>>{
    println!("HANDLER: READ USER BY ID {}", _id);
    Ok(Json(json!({"status": "works"})))
}

async fn delete_user() {}

