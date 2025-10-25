use axum::{routing::{get, post}, Router};
pub fn routes(AppState) -> Router {
    Router::new()
        .route("/create", post(create_user))
        .route("/update", post(update_user))
        .route("/read", get(read_user))
        .route("/delete", post(delete_user))
}

async fn create_user() {
    todo!();
}

async fn update_user() {
    todo!();
}

async fn read_user() {
    todo!();
}

async fn delete_user() {}

