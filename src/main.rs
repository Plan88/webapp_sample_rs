use std::net::{Ipv4Addr, SocketAddr};

use axum::{
    Json, Router,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tracing_subscriber::{
    EnvFilter, fmt::time::LocalTime, layer::SubscriberExt, util::SubscriberInitExt,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_logger()?;

    let app = get_router();
    serve(app).await?;
    Ok(())
}

fn init_logger() -> anyhow::Result<()> {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into());
    let subscriber = tracing_subscriber::fmt::layer()
        .with_timer(LocalTime::rfc_3339())
        .with_file(true)
        .with_line_number(true)
        .with_target(false);
    tracing_subscriber::registry()
        .with(env_filter)
        .with(subscriber)
        .try_init()?;
    Ok(())
}

async fn serve(app: Router) -> anyhow::Result<()> {
    let port = get_port();
    let addr = SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), port);
    let listener = TcpListener::bind(addr).await?;
    tracing::info!("Started at {}", addr);
    axum::serve(listener, app).await?;
    Ok(())
}

fn get_port() -> u16 {
    std::env::var("PORT")
        .unwrap_or_else(|_| "80".to_string())
        .parse::<u16>()
        .unwrap_or(80)
}

fn get_router() -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/sample", post(sample_post_endpoint))
}

async fn health() -> &'static str {
    "OK"
}

async fn sample_post_endpoint(Json(req): Json<RequestBody>) -> Json<ResponseBody> {
    tracing::info!(request_id = req.request_id, "request is arrived.");
    Json(ResponseBody {
        request_id: req.request_id,
    })
}

#[derive(Deserialize)]
struct RequestBody {
    request_id: String,
}

#[derive(Serialize)]
struct ResponseBody {
    request_id: String,
}
