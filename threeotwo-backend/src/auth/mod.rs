mod routes;
use axum::{http::StatusCode, response::{IntoResponse, Response}};
pub use routes::routes;

pub enum AuthError {
    LoginFailed
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        (StatusCode::BAD_REQUEST, "LOGIN_FAILED").into_response()
    }
}



pub const AUTH_TOKEN: &str = "user-auth";


