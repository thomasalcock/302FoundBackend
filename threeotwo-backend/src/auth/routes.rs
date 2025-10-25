use crate::{auth::{AuthError, AUTH_TOKEN}, error::{Error, Result}};
use axum::{routing::post, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};

#[derive(Debug, Deserialize)]
pub struct LoginPayload {
    user_name: String,
    password: String
}

pub fn routes() -> Router {
    Router::new()
        .route("/signup", post(signup))
        .route("/login", post(login))
}

async fn signup(_payload: Json<LoginPayload>) -> Result<Json<Value>> {
    Ok(Json(json!({
        "result": {
            "success": true
        }
    })))

}

async fn login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    if payload.user_name != "admin" || payload.password != "password" {
        println!("WRONG PASSWORD OR USER_NAME");
        return Err(Error::AuthError(AuthError::LoginFailed));
    }
    
    cookies.add(Cookie::new(AUTH_TOKEN, "user-1.exp.sign"));

    Ok(Json(json!({
        "result": {
            "success": true
        }
    })))
}
