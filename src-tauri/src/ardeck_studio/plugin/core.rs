/*
Ardeck studio - The ardeck command mapping software.
Copyright (C) 2024 project-ardeck

This program is free software; you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation; either version 3 of the License, or 
(at your option) any later version.

This program is distributed in the hope that it will be useful, 
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the 
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program. If not, see <https://www.gnu.org/licenses/>.
*/


use std::borrow::Borrow;
use std::fs::{self, File};
use std::path::Path;
use std::sync::{Arc, Mutex};

use axum::extract::ws::WebSocket;
use axum::extract::{State, WebSocketUpgrade};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{serve, Router};
use once_cell::sync::Lazy;
use tauri::plugin;
use tokio::net::TcpListener;

use crate::ardeck_studio::ardeck::manager::ArdeckManager;
use crate::ardeck_studio::service::dir::Directories;

use super::manager::PluginManager;

use super::{Plugin, PluginManifest, PluginMessage, PluginMessageData, PluginOpCode, PLUGIN_DIR};

static PLUGIN_MANAGER: Lazy<Mutex<ArdeckManager>> = Lazy::new(|| {
    Mutex::new(ArdeckManager::new())
});

pub struct PluginCore {
    plugin: PluginManager,
}

impl PluginCore {
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

    pub async fn execute_plugin_all() {
        let dir = Directories::get_or_init(Path::new(PLUGIN_DIR)).unwrap();

        for entry in dir {
            if entry.is_err() {
                continue;
                // TODO: Error
            }

            let path = entry.unwrap().path();

            let manifest_file = File::open(format!("{}/manifest.json", path.display()));
            if manifest_file.is_err() {
                println!("Failed to open manifest.json");
                continue;
            }

            let manifest: PluginManifest = serde_json::from_reader(manifest_file.unwrap()).unwrap();
            
            let plugin_main_path = format!("{}/{}", path.display(), manifest.main);

            let plugin_process = std::process::Command::new(plugin_main_path)
                .spawn()
                .expect("Failed to execute plugin");
            }
    }

    // pub async fn

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
