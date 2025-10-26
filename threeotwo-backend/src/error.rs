use axum::{http::StatusCode, response::{IntoResponse, Response}};

use crate::auth::AuthError;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Clone, Debug)]
pub enum Error {
    AuthError(AuthError),
    DatabaseError,
    UnknownError
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::AuthError(e) => e.into_response(),
            Error::DatabaseError => (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_DATABASE_ERROR").into_response(),
            Error::UnknownError => (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response(),
        }
    }
}
