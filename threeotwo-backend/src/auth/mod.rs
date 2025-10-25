mod routes;
mod mw;
use axum::{http::StatusCode, response::{IntoResponse, Response}};

pub use routes::routes;
pub use mw::require_auth;

#[derive(Clone, Debug)]
pub enum AuthError {
    LoginFailed,
    NoCookies,
    NoToken
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        match self {
            AuthError::LoginFailed => (StatusCode::INTERNAL_SERVER_ERROR, "LOGIN_FAILED").into_response(),
            AuthError::NoToken => (StatusCode::INTERNAL_SERVER_ERROR, "TOKEN_MISSING").into_response(),
            AuthError::NoCookies => (StatusCode::INTERNAL_SERVER_ERROR, "NO_COOKIES").into_response(),
            //_ => (StatusCode::BAD_REQUEST, "LOGIN_FAILED").into_response()
        }
    }
}



pub const AUTH_TOKEN: &str = "user-auth";


