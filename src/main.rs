use axum::{/*extract::Path,*/ routing::get, Router};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let users_routes = Router::new().route("/list", get(users_list));
    let posts_routes = Router::new().route("/all", get(posts_all));

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .nest("/users", users_routes)
        .nest("/posts", posts_routes);

    // run our app with hyper, listening globally on port 3000
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn users_list() -> String {
    "List of users".to_string()
}

async fn posts_all() -> String {
    "List of posts".to_string()
}
/*
async fn user_handler(Path(user_id): Path<u32>) -> String {
    format!("User ID: {}", user_id)
}
*/
