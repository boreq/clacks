use crate::app::AddMessageToQueue;
use crate::config::Environment;
use crate::domain::Message;
use crate::errors::{Error, Result};
use crate::{app, config};
use app::AddMessageToQueueHandler;
use axum::{
    Router,
    routing::{get, post},
};
use axum::{
    extract::Json,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use prometheus::TextEncoder;
use serde::Deserialize;
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

    pub async fn run<D>(&self, config: &config::Config, deps: D) -> Result<()>
    where
        D: Deps + Sync + Send + Clone + 'static,
    {
        let trace = TraceLayer::new_for_http();
        let cors = match config.environment() {
            Environment::Production => CorsLayer::new(),
            Environment::Development => CorsLayer::new().allow_origin(Any),
        };

        let compression = CompressionLayer::new();

        let app = Router::new()
            .route("/metrics", get(handle_get_metrics::<D>))
            .route("/queue", post(handle_post_queue::<D>))
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

async fn handle_get_metrics<D>(State(deps): State<D>) -> std::result::Result<String, AppError>
where
    D: Deps,
{
    let encoder = TextEncoder::new();
    let families = deps.metrics().gather();
    Ok(encoder.encode_to_string(&families)?)
}

async fn handle_post_queue<D>(
    State(deps): State<D>,
    Json(json_body): Json<PostQueueRequest>,
) -> std::result::Result<(), AppError>
where
    D: Deps,
{
    let message = Message::new(json_body.message).map_err(|_| AppError::BadRequest)?;
    let command = AddMessageToQueue::new(message);
    deps.add_message_to_queue_handler().handle(command)?;
    Ok(())
}

#[derive(Deserialize)]
struct PostQueueRequest {
    message: String,
}

pub trait Deps {
    fn add_message_to_queue_handler(&self) -> &impl AddMessageToQueueHandler;
    fn metrics(&self) -> &prometheus::Registry;
}

enum AppError {
    NotFound,
    BadRequest,
    UnknownError,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::NotFound => (StatusCode::NOT_FOUND, "Not found").into_response(),
            AppError::BadRequest => (StatusCode::BAD_REQUEST, "Bad request").into_response(),
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
