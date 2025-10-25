use axum::response::Response;
use tower_cookies::Cookies;

use crate::{auth::{AuthError, AUTH_TOKEN}, error::{Error, Result}};


pub async fn require_auth(
    cookies: Cookies,
    response: Response) -> Result<Response> {
    let _user_auth = cookies
        .get(AUTH_TOKEN)
        .map(|c| c.value().to_string())
        .ok_or(Error::AuthError(AuthError::NoToken));
    Ok(response)
}
