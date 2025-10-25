use axum::body::Body;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use axum::RequestPartsExt;
use lazy_regex::regex_captures;
use tower_cookies::Cookies;

use crate::auth::{AuthError, AUTH_TOKEN};
use crate::error::{Error, Result};
use crate::context::Context;

pub async fn require_auth(
    context: Result<Context>,
    cookies: Cookies,
    req: Request<Body>,
    next: Next) -> Result<Response> {
    
    println!("{} cookies", cookies.list().len());

    context?;
    Ok(next.run(req).await)
}

fn parse_token(token: String) -> Result<(u64, String, String)> {
   let (_whole, user_id, exp, sign) = regex_captures!(
    r#"^user-(\d+)\.(.+)\.(.+)"#,
    &token
        )
   .ok_or(Error::AuthError(AuthError::LoginFailed))?;
    let user_id: u64 = user_id.parse()
        .map_err(|_| Error::AuthError(AuthError::LoginFailed))?;
    Ok((user_id,exp.to_string(),sign.to_string()))
}

impl <S: Sync + Send> FromRequestParts<S> for Context {
    type Rejection = crate::error::Error;
    
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        let cookies = parts.extract::<Cookies>().await.unwrap();
        println!("size: {}", cookies.list().len());
        let (user_id, _, _) = cookies
            .get(AUTH_TOKEN)
            .map(|c| c.value().to_string())
            .ok_or(Error::AuthError(AuthError::NoCookies))
            .and_then(parse_token)?;
        println!("user_id: {}", user_id);
        Ok(Context::new(user_id))
    }
}

