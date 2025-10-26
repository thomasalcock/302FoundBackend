use crate::{
    auth::{AuthError, AUTH_TOKEN},
    error::{Error, Result}, AppState,
};
use axum::{Json, Router, routing::post};
use serde::Deserialize;
use serde_json::{Value, json};
use tower_cookies::{Cookie, Cookies, cookie::SameSite};

#[derive(Debug, Deserialize)]
pub struct LoginPayload {
    user_name: String,
    password: String,
}

pub fn routes() -> Router<AppState> {
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

    let mut cookie = Cookie::new(AUTH_TOKEN, "user-1.exp.sign");
    cookie.set_path("/");
    cookie.set_same_site(SameSite::Lax);
    cookie.set_secure(false);
    cookies.add(cookie);

    Ok(Json(json!({
        "result": {
            "success": true
        }
    })))
}
