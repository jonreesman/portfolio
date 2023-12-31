#[allow(dead_code)]

mod markdown;
mod templates;
mod routes;

use std::sync::Arc;
use axum::{
    routing::get,
    Router
};
use tokio::sync::Mutex;
use tower_http::services::ServeDir;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};


#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "with_axum_htmx_askama=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    info!("initializing router...");

    let app_state = Arc::new(routes::AppState {
        todos: Mutex::new(vec![]),
        note_routes: routes::get_notes()
    });



    let assets_path = std::env::current_dir().unwrap();

    let api_router = Router::new()
        .route("/hello", get(routes::hello_from_the_server));

    let app = Router::new()
        .nest("/api", api_router)
        .route("/", get(routes::hello))
        .route("/notes", get(routes::notes))
        .route("/notes/:id", get(routes::notes_md))
        .with_state(app_state)
        .nest_service(
            "/assets",
            ServeDir::new(format!("{}/assets", assets_path.to_str().unwrap())),
        );

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
