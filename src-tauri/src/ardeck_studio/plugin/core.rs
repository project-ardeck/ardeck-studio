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
use std::net::SocketAddr;
use std::path::Path;
use std::sync::{Arc, Mutex};

use axum::extract::ws::Message::Text;
use axum::extract::ws::WebSocket;
use axum::extract::{ConnectInfo, State, WebSocketUpgrade};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::serve::Serve;
use axum::{serve, Router};
use once_cell::sync::Lazy;
use tauri::plugin;
use tokio::net::TcpListener;
use tokio::sync::Mutex as TokioMutex;

use crate::ardeck_studio::service::dir::Directories;

use super::manager::PluginManager;

use super::{Plugin, PluginManifest, PluginMessage, PluginMessageData, PluginOpCode, PLUGIN_DIR};

pub struct PluginCore {
    plugin_manager: Arc<Mutex<PluginManager>>,
    serve: Option<tokio::task::JoinHandle<()>>,
    
}

impl PluginCore {
    pub fn new() -> Self {
        Self {
            plugin_manager: Arc::new(Mutex::new(PluginManager::new())),
            serve: None,
        }
    }

    pub async fn start(&mut self) -> Result<(), std::io::Error> {
        let listener = TcpListener::bind("localhost::3322").await?;

        let app = Router::new()
            .route("/", get(RouteHandler::plugin_socket))
            // .route("/state", get(Self::state))
            .route("/plugin", get(RouteHandler::plugin_list))
            .route("/plugin/:id", get(RouteHandler::plugin_id))
            .fallback(get(RouteHandler::err_404))
            .with_state(Arc::clone(&self.plugin_manager));

        // self.serve = Some(
        tokio::spawn(async move {
            serve(listener, app).await.unwrap();
        });
        // );

        println!("plugin server started.");

        Ok(())
    }

    pub fn execute_plugin_all(&self) {
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

            let process = std::process::Command::new(plugin_main_path)
                .spawn()
                .expect("Failed to execute plugin");

        //     let plugin = Plugin {
        //         manifest,
        //         process: Arc::new(Mutex::new(process)),
        //         session
        //     };

        //     self.plugin.lock().unwrap().insert(manifest.id, plugin_process);
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
    pub async fn plugin_socket(
        ws: WebSocketUpgrade,
        // user_agent/>
        ConnectInfo(addr): ConnectInfo<SocketAddr>,
        State(plugin_manager): State<Arc<Mutex<PluginManager>>>,
    ) -> impl IntoResponse {
        ws.on_upgrade(move |socket| handle_socket(socket, addr, plugin_manager))
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

async fn handle_socket(socket: WebSocket, who: SocketAddr, plugin_manager: Arc<Mutex<PluginManager>>) {
    let socket = Arc::new(TokioMutex::new(socket));
    let hello_message = PluginMessage {
        op: PluginOpCode::Hello,
        data: PluginMessageData::default(),
    };

    let data_string = serde_json::to_string(&hello_message).unwrap();

    socket.lock().await.send(Text(data_string)).await.unwrap();

    loop {
        let recv = socket.lock().await.recv().await;

        if recv.is_none() {
            println!("Plugin session recv is none");
            return;
        }

        match recv.unwrap() {
            Ok(m) => {
                let msg_str = match m.to_text() {
                    Ok(msg) => msg,
                    Err(msg) => continue,
                };
                
                let msg: PluginMessage = match serde_json::from_str(msg_str) {
                    Ok(msg) => msg,
                    Err(msg) => continue,
                };

                let op = msg.op;
                let data = msg.data;

                match op {
                    PluginOpCode::Challenge => {
                        // TODO: データがなかったらError
                        let plugin_version = data.plugin_version.unwrap();
                        let plugin_id = data.plugin_id.unwrap();

                        plugin_manager.lock().unwrap().get_mut(&plugin_id).unwrap().set_session(Arc::clone(&socket));

                        let success_data = PluginMessage {
                            op: PluginOpCode::Success,
                            data: {
                                PluginMessageData::default()
                            }
                        };

                        socket.lock().await.send(Text(serde_json::to_string(&success_data).unwrap())).await.unwrap();
                    }
                    PluginOpCode::Error => {

                    }
                    PluginOpCode::Message => {

                    }
                    _ => {}
                }
            }
            Err(e) => {
                
            }
        }

    }
}
