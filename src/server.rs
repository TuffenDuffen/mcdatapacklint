use axum::{Router, extract::State, routing::get};
use tower_http::trace::TraceLayer;
use tracing::{debug, error, info, warn};
use tracing_subscriber::EnvFilter;

pub async fn run_server() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
        }))
        .init();

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    info!("Listening on 127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}


struct HookState

async fn handle_lint_hook(State(state): State<HookState>)
