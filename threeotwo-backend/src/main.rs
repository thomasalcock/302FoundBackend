use std::env;

use axum::response::Response;
use axum::routing::any;
use axum::{Router, middleware};
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use tower_http::cors::CorsLayer;

mod error;
use crate::error::Error;

mod auth;
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

    //merge routes here
    let app = Router::new()
        .nest("/users", user::routes(app_state))
        .layer(middleware::map_response(auth::require_auth))
        .nest("/auth", auth::routes())
        .layer(middleware::map_response(map_all_responses))
        .layer(CookieManagerLayer::new())
        .layer(CorsLayer::permissive())
        .route("/{*wildcard}", any(static_error));

    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
<<<<<<< HEAD
=======
    
    println!("SERVER LISTING ON {}", listener.local_addr().unwrap());

>>>>>>> 3e22d19 (server start message)
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
