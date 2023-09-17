use axum::{
    http::{HeaderValue, Method},
    routing::{get, post},
    Router,
};
use dotenv::dotenv;

use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

mod handlers;
use handlers::{
    connection_checks::{add_test_user, check_backend, check_mongo},
    jwt::check_given_token,
    users::{login, register_user},
};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_client = mongodb::Client::with_uri_str(std::env::var("DATABASE_URI").unwrap())
        .await
        .unwrap();

    let cors_layer: CorsLayer = CorsLayer::new()
        .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(vec![
            axum::http::header::CONTENT_TYPE,
            axum::http::header::AUTHORIZATION,
        ]);

    let app = Router::new()
        .route("/check_token", get(check_given_token))
        .route("/add_test_user", post(add_test_user))
        .route("/check_mongo", get(check_mongo))
        .route("/check_backend", get(check_backend))
        .route("/register_user", post(register_user))
        .route("/login", post(login))
        .route("/upload_notes", post(handlers::sync::upload_notes))
        .route("/download_notes", get(handlers::sync::download_notes))
        .layer(cors_layer)
        .with_state(database_client);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
