use axum::RequestPartsExt;
use axum::body::Body;
use axum::extract::FromRequestParts;
use axum::http::Request;
use axum::http::request::Parts;
use axum::middleware::Next;
use axum::response::Response;
use lazy_regex::regex_captures;
use tower_cookies::Cookies;

use crate::auth::{AUTH_TOKEN, AuthError};
use crate::context::Context;
use crate::error::{Error, Result};

pub async fn require_auth(
    context: Result<Context>,
    req: Request<Body>,
    next: Next,
) -> Result<Response> {

    context?;
    Ok(next.run(req).await)
}

fn parse_token(token: String) -> Result<(u64, String, String)> {
    let (_whole, user_id, exp, sign) = regex_captures!(r#"^user-(\d+)\.(.+)\.(.+)"#, &token)
        .ok_or(Error::AuthError(AuthError::LoginFailed))?;
    let user_id: u64 = user_id
        .parse()
        .map_err(|_| Error::AuthError(AuthError::LoginFailed))?;
    Ok((user_id, exp.to_string(), sign.to_string()))
}

impl<S: Sync + Send> FromRequestParts<S> for Context {
    type Rejection = crate::error::Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        let cookies = parts.extract::<Cookies>().await.unwrap();
        let (user_id, _, _) = cookies
            .get(AUTH_TOKEN)
            .map(|c| c.value().to_string())
            .ok_or(Error::AuthError(AuthError::NoCookies))
            .and_then(parse_token)?;
        Ok(Context::new(user_id))
    }
}
