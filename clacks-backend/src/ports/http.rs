use crate::config::Environment;
use crate::errors::{Error, Result};
use crate::{adapters, app, config};
use anyhow::anyhow;
use axum::body::Body;
use axum::handler::Handler;
use axum::http::Request;
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use axum::{routing::get, Router};
use prometheus::TextEncoder;
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};

pub struct Server {}

impl Default for Server {
    fn default() -> Self {
        Self::new()
    }
}

impl Server {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn run<A, M>(&self, config: &config::Config, deps: Deps<A, M>) -> Result<()>
    where
        A: App + Sync + Send + Clone + 'static,
        M: Metrics + Sync + Send + Clone + 'static,
    {
        let trace = TraceLayer::new_for_http();
        let cors = match config.environment() {
            Environment::Production => {
                CorsLayer::new()
            }
            Environment::Development => {
                CorsLayer::new().allow_origin(Any)
            }
        };

        let compression = CompressionLayer::new();

        let app = Router::new()
            .route("/metrics", get(handle_get_metrics))
            .with_state(deps)
            .layer(
                ServiceBuilder::new()
                    .layer(trace.clone())
                    .layer(compression.clone())
                    .layer(cors.clone()),
            );

        let listener = tokio::net::TcpListener::bind(config.address()).await?;
        axum::serve(listener, app).await?;
        Ok(())
    }
}

async fn handle_get_metrics<A, M>(
    State(deps): State<Deps<A, M>>,
) -> std::result::Result<String, AppError>
where
    A: App,
    M: Metrics,
{
    let encoder = TextEncoder::new();
    let families = deps.metrics.registry().gather();
    Ok(encoder.encode_to_string(&families)?)
}

#[derive(Clone)]
pub struct Deps<A, M> {
    app: A,
    metrics: M,
}

impl<A, M> Deps<A, M> {
    pub fn new(app: A, metrics: M) -> Self {
        Self { app, metrics }
    }
}

pub trait App {
    fn add_message_to_queue(&self) -> &impl app::AddMessageToQueueHandler;
}

pub trait Metrics {
    fn registry(&self) -> &prometheus::Registry;
}

impl Metrics for adapters::Metrics {
    fn registry(&self) -> &prometheus::Registry {
        self.registry()
    }
}

enum AppError {
    NotFound,
    UnknownError,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::NotFound => (StatusCode::NOT_FOUND, "Not found").into_response(),
            AppError::UnknownError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
            }
        }
    }
}

impl<E> From<E> for AppError
where
    E: Into<Error>,
{
    fn from(_err: E) -> Self {
        Self::UnknownError
    }
}
