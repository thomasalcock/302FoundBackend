use axum::response::Response;
use axum::routing::any;
use axum::{middleware, Router};
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;

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
    
    let mut app_state = AppState::new("db.sqlite");

    //merge routes here
    let app = Router::new()
        .nest("/auth", auth::routes())
        .nest("/user", user::routes(app_state))
        .layer(middleware::map_response(map_all_responses))
        .layer(CookieManagerLayer::new())
        .route("/{*wildcard}", any(static_error));

    let listener = TcpListener::bind("0.0.0.0:3000")
        .await
.unwrap();
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
