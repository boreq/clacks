use crate::app::{AddMessageToQueue, Config, GetConfigHandler, GetStateHandler};
use crate::config::Environment;
use crate::domain::{
    CurrentMessage, EncodedMessage, EncodedMessagePart, Message, MessageComponent, ShutterLocation,
    ShutterPositions,
};
use crate::errors::{Error, Result};
use crate::{adapters, app, config};
use anyhow::anyhow;
use app::AddMessageToQueueHandler;
use axum::body::Body;
use axum::extract::ws::WebSocket;
use axum::extract::{WebSocketUpgrade, ws};
use axum::handler::Handler;
use axum::http::Request;
use axum::routing::any;
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
use futures_util::{sink::SinkExt, stream::StreamExt};
use include_dir::File;
use log::debug;
use prometheus::TextEncoder;
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;
use tokio::sync::broadcast::Receiver;
use tokio::task;
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};

#[cfg(feature = "serve_frontend")]
static FRONTEND_DIR: include_dir::Dir = include_dir::include_dir!("$FRONTEND_DIST_DIRECTORY");

#[cfg(not(feature = "serve_frontend"))]
static FRONTEND_DIR: include_dir::Dir = include_dir::Dir::new("fakedir", &[]);

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
            Environment::Development => CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        };

        let compression = CompressionLayer::new();

        let app = Router::new()
            .route("/metrics", get(handle_get_metrics::<D>))
            .route("/api/queue", post(handle_post_queue::<D>))
            .route("/api/state-updates", any(handle_state_updates::<D>))
            .route("/api/config", get(handle_get_config::<D>))
            .layer(
                ServiceBuilder::new()
                    .layer(trace.clone())
                    .layer(compression.clone())
                    .layer(cors.clone()),
            )
            .with_state(deps)
            .fallback(
                serve_frontend
                    .layer(trace.clone())
                    .layer(compression.clone())
                    .layer(cors.clone()),
            );

        let listener = tokio::net::TcpListener::bind(config.address()).await?;
        axum::serve(listener, app).await?;
        Ok(())
    }
}

async fn serve_frontend(request: Request<Body>) -> std::result::Result<Response<Body>, AppError> {
    let path = request
        .uri()
        .path()
        .strip_prefix("/")
        .ok_or_else(|| anyhow!("it seems that the path didn't start with / which is odd"))?;
    let f = get_path_or_index_html(path)?;
    let mime = get_mime(f.path());

    let mut resp = Response::builder().status(StatusCode::OK);
    if let Some(mime) = mime {
        resp = resp.header("Content-Type", mime);
    }
    let body = Body::from(f.contents());
    let resp = resp.body(body)?;
    Ok(resp)
}

fn get_path_or_index_html(path: &str) -> Result<&'static File<'static>> {
    match FRONTEND_DIR.get_file(path) {
        Some(file) => Ok(file),
        None => match FRONTEND_DIR.get_file("index.html") {
            Some(file) => Ok(file),
            None => Err(anyhow!("no index.html found?!").into()),
        },
    }
}

fn get_mime(path: &std::path::Path) -> Option<&str> {
    if path.extension().is_some_and(|v| v == "js") {
        return Some("text/javascript");
    }

    if path.extension().is_some_and(|v| v == "css") {
        return Some("text/css");
    }

    if path.extension().is_some_and(|v| v == "html") {
        return Some("text/html");
    }

    None
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
    let message = Message::new(json_body.message)
        .map_err(|_| AppError::BadRequest("invalid message".into()))?;
    let command = AddMessageToQueue::new(message);
    deps.add_message_to_queue_handler().handle(command)?;
    Ok(())
}

async fn handle_state_updates<D>(ws: WebSocketUpgrade, State(deps): State<D>) -> Response
where
    D: Deps + Send + 'static,
{
    ws.on_upgrade(move |websocket| handle_socket(websocket, deps))
}

async fn handle_socket<D>(websocket: WebSocket, deps: D)
where
    D: Deps + Send + 'static,
{
    let (cancel, _) = broadcast::channel(1);
    let mut cancel_1 = cancel.subscribe();

    let (mut socket_sender, mut socket_receiver) = websocket.split();

    task::spawn(async move {
        let mut s1 = deps.subscriber().subscribe_to_message_added_to_queue();
        let mut s2 = deps.subscriber().subscribe_to_clacks_updated();

        let state = deps.get_state_handler().get_state().unwrap();
        let transport_state: TransportState = (&state).into();
        let string_state = serde_json::to_string(&transport_state).unwrap();
        let message = ws::Message::text(string_state);
        match socket_sender.send(message).await {
            Ok(_) => {}
            Err(_) => {
                return;
            }
        };

        loop {
            tokio::select! {
                _ = s1.recv() => {
                    let state= deps.get_state_handler().get_state().unwrap();
                    let transport_state: TransportState = (&state).into();
                    let string_state = serde_json::to_string(&transport_state).unwrap();
                    let message = ws::Message::text(string_state);
                    match socket_sender.send(message).await {
                        Ok(_) => {
                        },
                        Err(_) => {
                            return;
                        },
                    };
                }
                _ = s2.recv() => {
                    let state= deps.get_state_handler().get_state().unwrap();
                    let transport_state: TransportState = (&state).into();
                    let string_state = serde_json::to_string(&transport_state).unwrap();
                    let message = ws::Message::text(string_state);
                    match socket_sender.send(message).await {
                        Ok(_) => {
                        },
                        Err(_) => {
                            return;
                        },
                    };
                },
                _ = cancel_1.recv() => {
                    return;
                }
            }
        }
    });

    match socket_receiver.next().await {
        None => {}
        Some(_) => {
            debug!("frontend sent us something which is odd")
        }
    }

    cancel.send(()).unwrap();
}

async fn handle_get_config<D>(
    State(deps): State<D>,
) -> std::result::Result<Json<TransportConfig>, AppError>
where
    D: Deps,
{
    let config = deps.get_config_handler().get_config()?;
    let transport_config: TransportConfig = config.into();
    Ok(transport_config.into())
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct TransportConfig {
    supported_characters: Vec<String>,
    max_message_len_in_bytes: usize,
}

impl From<Config> for TransportConfig {
    fn from(value: Config) -> Self {
        Self {
            supported_characters: value.supported_characters().into(),
            max_message_len_in_bytes: value.max_message_len_in_bytes(),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct TransportState {
    current_message: Option<TransportCurrentMessage>,
    queue: Vec<TransportEncodedMessage>,
}

impl From<&app::State> for TransportState {
    fn from(value: &app::State) -> Self {
        Self {
            current_message: value.current_message().map(|v| v.into()),
            queue: value.queue().iter().map(|v| v.into()).collect(),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct TransportCurrentMessage {
    before: Vec<TransportEncodedMessagePart>,
    current: Option<TransportEncodedMessagePart>,
    after: Vec<TransportEncodedMessagePart>,
}

impl From<&CurrentMessage> for TransportCurrentMessage {
    fn from(value: &CurrentMessage) -> Self {
        Self {
            before: value.before().iter().map(|v| v.into()).collect(),
            current: value.current().map(|v| v.into()),
            after: value.after().iter().map(|v| v.into()).collect(),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct TransportEncodedMessage {
    parts: Vec<TransportEncodedMessagePart>,
}

impl From<&EncodedMessage> for TransportEncodedMessage {
    fn from(value: &EncodedMessage) -> Self {
        Self {
            parts: value.parts().iter().map(|x| x.into()).collect(),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct TransportEncodedMessagePart {
    kind: String,
    character: Option<String>,
    open_shutters: TransportShutterPositions,
}

impl From<&EncodedMessagePart> for TransportEncodedMessagePart {
    fn from(value: &EncodedMessagePart) -> Self {
        match value.element() {
            MessageComponent::Character(ch) => Self {
                kind: "CHARACTER".into(),
                character: Some(ch.to_string()),
                open_shutters: value.shutter_positions().into(),
            },
            MessageComponent::End => Self {
                kind: "END".to_string(),
                character: None,
                open_shutters: value.shutter_positions().into(),
            },
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct TransportShutterPositions(Vec<String>);

impl From<&ShutterPositions> for TransportShutterPositions {
    fn from(value: &ShutterPositions) -> Self {
        Self(value.open_shutters().map(|v| v.into()).collect())
    }
}

impl From<&ShutterLocation> for String {
    fn from(value: &ShutterLocation) -> Self {
        match value {
            ShutterLocation::TopLeft => "TOP_LEFT",
            ShutterLocation::TopRight => "TOP_RIGHT",
            ShutterLocation::MiddleLeft => "MIDDLE_LEFT",
            ShutterLocation::MiddleRight => "MIDDLE_RIGHT",
            ShutterLocation::BottomLeft => "BOTTOM_LEFT",
            ShutterLocation::BottomRight => "BOTTOM_RIGHT",
        }
        .into()
    }
}

#[derive(Deserialize)]
struct PostQueueRequest {
    message: String,
}

pub trait Deps {
    fn add_message_to_queue_handler(&self) -> &impl AddMessageToQueueHandler;
    fn get_state_handler(&self) -> &impl GetStateHandler;
    fn get_config_handler(&self) -> &impl GetConfigHandler;

    fn metrics(&self) -> &prometheus::Registry;
    fn subscriber(&self) -> &impl EventSubscriber;
}

pub trait EventSubscriber {
    fn subscribe_to_clacks_updated(&self) -> Receiver<()>;
    fn subscribe_to_message_added_to_queue(&self) -> Receiver<()>;
}

impl EventSubscriber for adapters::PubSub {
    fn subscribe_to_clacks_updated(&self) -> Receiver<()> {
        self.subscribe_to_clacks_updated()
    }

    fn subscribe_to_message_added_to_queue(&self) -> Receiver<()> {
        self.subscribe_to_message_added_to_queue()
    }
}

enum AppError {
    BadRequest(String),
    UnknownError,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::BadRequest(message) => (StatusCode::BAD_REQUEST, message),
            AppError::UnknownError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".into(),
            ),
        };
        (status, Json(TransportError { message })).into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<Error>,
{
    fn from(err: E) -> Self {
        match err.into() {
            Error::QueueIsFull => Self::BadRequest("Queue is full".into()),
            _ => Self::UnknownError,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct TransportError {
    message: String,
}
