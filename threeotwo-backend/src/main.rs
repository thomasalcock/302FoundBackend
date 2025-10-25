use std::env;

use axum::http::Method;
use axum::response::Response;
use axum::routing::any;
use axum::{Router, middleware};
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use tower_http::cors::{AllowOrigin, CorsLayer};

mod error;
use crate::error::Error;

mod auth;
mod context;
mod user;

pub struct AppState {}
impl AppState {
    fn new(_database: &str) -> Self {
        AppState {}
    }
}

#[tokio::main]
async fn main() {
    println!("STARTING WEB SERVER");

    let app_state = AppState::new("db.sqlite");

    let port = match env::var("BACKEND_PORT") {
        Ok(p) => p,
        Err(_) => "3000".to_string(),
    };

    use axum::http::{Method, header};

    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::exact("http://localhost:3000".parse().unwrap()))
        .allow_credentials(true)
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
        .allow_private_network(true);

    //merge routes here
    let app = Router::new()
        .nest("/users", user::routes(app_state))
        .layer(middleware::map_response(map_all_responses))
        .layer(middleware::from_fn(auth::require_auth))
        .nest("/auth", auth::routes())
        .layer(CookieManagerLayer::new())
        .layer(cors)
        .route("/{*wildcard}", any(static_error));

    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();

    println!("SERVER LISTING ON {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();

    println!("SERVER IS SHUT DOWN");
}

async fn map_all_responses(response: Response) -> Response {
    println!("");

    response
}

async fn static_error() -> Error {
    Error::UnknownError
}
