use std::sync::{Arc, Mutex};

use axum::extract::ws::WebSocket;
use axum::extract::{State, WebSocketUpgrade};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{serve, Router};
use once_cell::sync::Lazy;
use tokio::net::TcpListener;

use super::manager::PluginManager;

use super::{Plugin, PluginManifest, PluginMessage, PluginMessageData, PluginOpCode, PLUGIN_DIR};

pub struct PluginServe {}
impl PluginServe {
    pub async fn start(plugins_state: Mutex<PluginManager>) {
        let listener = TcpListener::bind("localhost::3322").await.unwrap();

        let state = Arc::new(plugins_state);
        let app = Router::new()
            .route("/", get(RouteHandler::plugin_socket))
            // .route("/state", get(Self::state))
            .route("/plugin", get(RouteHandler::plugin_list))
            .route("/plugin/:id", get(RouteHandler::plugin_id))
            .fallback(get(RouteHandler::err_404))
            .with_state(Arc::clone(&state));

        serve(listener, app).await.unwrap();
    }

    // async fn socket_handler(ws: WebSocketUpgrade) {
    //     ws.on_upgrade(move |socket: WebSocket| self)
    // }

    // async fn plugin_session(mut socket: WebSocket) {}
}

struct RouteHandler {}
impl RouteHandler {
    pub async fn plugin_socket() -> impl IntoResponse {
        "OK"
    }

    pub async fn plugin_list(State(state): State<Arc<Mutex<PluginManager>>>) -> impl IntoResponse {
        "OK"
    }

    pub async fn plugin_id(State(state): State<Arc<Mutex<PluginManager>>>) -> impl IntoResponse {
        "OK"
    }

    pub async fn err_404() -> impl IntoResponse {
        "Not Found"
    }
}
