use axum::Router;
use tokio::net::TcpListener;

mod user;
use crate::user::user_routes;

#[tokio::main]
async fn main() {
    println!("STARTING WEB SERVER");

    //merge routes here
    let app = Router::new()
        .merge(user_routes());
    

    let listener = TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();

    println!("SERVER IS SHUT DOWN");
}
