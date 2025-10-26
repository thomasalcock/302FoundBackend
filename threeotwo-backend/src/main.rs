use std::env;

use axum::http::{header, Method};
use axum::response::Response;
use axum::routing::any;
use axum::{Router, middleware};

use sqlx::migrate::MigrateDatabase;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::Sqlite;
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use tower_http::cors::{AllowOrigin, CorsLayer};

mod error;
use crate::app_state::AppState;
use crate::error::Error;

mod auth;
mod context;
mod user;
mod trust;
mod location;

mod app_state;

#[tokio::main]
async fn main() {
    println!("STARTING WEB SERVER");

    let sqlite_url = "sqlite://db.sqlite";
    if !Sqlite::database_exists(sqlite_url).await.unwrap_or(false) {
        Sqlite::create_database(sqlite_url).await.unwrap();
    }
    let pool = SqlitePoolOptions::new().connect(sqlite_url).await.unwrap();

    let app_state = AppState::new(pool);
    app_state.init_database().await.unwrap();

    let port = match env::var("BACKEND_PORT") {
        Ok(p) => p,
        Err(_) => "3000".to_string(),
    };

    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::exact("http://localhost:3000".parse().unwrap()))
        .allow_credentials(true)
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
        .allow_private_network(true);

    //merge routes here
    let app = Router::new()
        .nest("/trusts", trust::routes())
        .nest("/users", user::routes())
        .nest("/location", location::routes())
        .layer(middleware::map_response(map_all_responses))
        .layer(middleware::from_fn(auth::require_auth))
        .nest("/auth", auth::routes())
        .with_state(app_state)
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
