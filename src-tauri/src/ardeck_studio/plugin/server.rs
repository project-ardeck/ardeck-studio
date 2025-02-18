/*
Ardeck studio - The ardeck command mapping software.
Copyright (C) 2024 Project Ardeck

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

use std::fs::File;
use std::net::SocketAddr;
use std::sync::Arc;

use futures_util::stream::SplitSink;
use futures_util::{SinkExt, StreamExt};
use tokio::sync::Mutex;

use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc::channel;
use tokio_tungstenite::tungstenite::Utf8Bytes;
use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message};

use crate::ardeck_studio::action::Action;
use crate::ardeck_studio::switch_info::SwitchInfo;
use crate::service::dir::Directories;

use super::manager::PluginManager;

use super::{Plugin, PluginAction, PluginManifestJSON, PluginMessage};

// static PLUGIN_MANAGER: Lazy<Mutex<PluginManager>> = Lazy::new(|| Mutex::new(PluginManager::new()));

pub type PluginServerSink = SplitSink<WebSocketStream<TcpStream>, Message>;

pub struct PluginServer {
    plugin_manager: Arc<Mutex<PluginManager>>,
    listener: Option<tokio::task::JoinHandle<()>>,
}

impl PluginServer {
    pub fn new() -> Self {
        Self {
            plugin_manager: Arc::new(Mutex::new(PluginManager::new())),
            listener: None,
        }
    }

    pub async fn start(&mut self) -> std::io::Result<()> {
        let plugin_manager = Arc::clone(&self.plugin_manager);

        let (tx, mut rx) = channel::<bool>(100);

        // 起動
        self.listener = Some(tokio::spawn(async move {
            // TODO: ポート番号を設定可能にする
            let tcp = TcpListener::bind("127.0.0.1:6725").await.unwrap();

            // 起動失敗
            if let Err(_) = tx.send(true).await {
                return;
            }

            log::trace!("listening on {}", tcp.local_addr().unwrap());

            // 接続待ち
            while let Ok((stream, _)) = tcp.accept().await {
                let peer = stream
                    .peer_addr()
                    .expect("connected streams should have a peer address");

                tokio::spawn(handle_connection(peer, stream, plugin_manager.clone()));
            }
        }));

        // 起動待ち
        while let Some(b) = rx.recv().await {
            if b {
                log::trace!("plugin server OK.");

                return Ok(());
            } else {
                log::trace!("plugin server failed.");

                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Failed to start plugin server.",
                ));
            }
        }

        log::error!("Failed to start plugin server.");
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to start plugin server.",
        ))
    }

    pub async fn execute_plugin_all(&self) {
        log::info!("Executing plugin all...");

        let dir = match Directories::get(Directories::get_plugin_dir().unwrap()) {
            Ok(read_dir) => read_dir,
            Err(_) => {
                log::error!("Failed to get plugin dir.");
                return;
            }
        };

        for entry in dir {
            if entry.is_err() {
                continue;
                // TODO: Error
            }

            let path = entry.unwrap().path();

            if path.is_file() {
                continue;
            }

            // マニフェストファイルを取得
            let manifest_file = match File::open(path.clone().join("manifest.json")) {
                Ok(file) => file,
                Err(e) => {
                    log::error!("Failed to open manifest.json: {}", e);

                    continue;
                }
            };

            // アクションファイルを取得
            let actions_file = match File::open(path.clone().join("actions.json")) {
                Ok(file) => file,
                Err(e) => {
                    log::error!("Failed to open actions.json: {}", e);

                    continue;
                }
            };

            let manifest: PluginManifestJSON = serde_json::from_reader(manifest_file).unwrap();
            let actions: Vec<PluginAction> = serde_json::from_reader(actions_file).unwrap();

            // プラグインの実行ファイルのパスを取得
            let plugin_main_path = path.clone().join(manifest.clone().main);

            log::info!("Executing plugin: {}", &manifest.name);

            // プラグインを実行
            let process = std::process::Command::new(plugin_main_path)
                .arg("6725")
                .spawn()
                .expect("Failed to execute plugin");

            // プラグイン情報とプロセスをマネージャーに登録
            match self.plugin_manager.lock().await.insert(
                manifest.id.clone(),
                // Plugin::new(manifest, actions, Arc::new(Mutex::new(process))),
                Plugin::new(manifest.clone(), actions),
            ) {
                None => (),
                Some(_) => (),
            }

            log::info!("Registered plugin: {}", &manifest.name);
        }

        log::info!("Plugin all executed.");
    }

    pub async fn put_action(&mut self, switch_info: SwitchInfo) {
        // TODO: switch_typeとswitch_idからマッピングの設定を見つけ、そのプラグインに（あれば）put_actionする

        let actions = Action::from_switch_info(switch_info).await;

        // actionsのtargetの中で、読み込まれているプラグインがあれば、プラグインに渡す
        for action in actions.iter() {
            match self
                .plugin_manager
                .lock()
                .await
                .get_mut(&action.target.plugin_id)
            {
                Some(plugin) => {
                    log::trace!(
                        "\t[plugin.server]: put_action: plugin found: {}",
                        action.target.plugin_id
                    );
                    plugin.put_action(action.clone()).await;
                }
                None => log::trace!("\t[plugin.server]: put_action: plugin not found"),
            }
        }

    }
}

// セッション
async fn handle_connection(
    peer: SocketAddr,
    stream: TcpStream,
    plugin_manager: Arc<Mutex<PluginManager>>,
) {
    let ws_stream = accept_async(stream).await.expect("Failed to accept");

    let (sink, mut stream) = ws_stream.split();
    let sink_arc = Arc::new(Mutex::new(sink));

    while let Some(msg) = stream.next().await {
        if let Ok(msg) = msg {
            if msg.is_text() {
                let msg_str = msg.to_text().unwrap();

                log::trace!("Received: {}", msg_str);

                let message: PluginMessage = serde_json::from_str(msg_str).unwrap();

                match message {
                    PluginMessage::Hello {
                        plugin_version,
                        ardeck_plugin_web_socket_version,
                        plugin_id,
                    } => {
                        log::trace!("Hello:\n\t{}", plugin_id);

                        // TODO: tauri.conf.jsonからのバージョン情報の取得
                        let data = PluginMessage::Success {
                            ardeck_studio_version: "0.2.0".to_string(),
                            ardeck_studio_web_socket_version: "0.0.1".to_string(),
                        };

                        sink_arc
                            .lock()
                            .await
                            .send(Message::Text(Utf8Bytes::from(
                                &serde_json::to_string(&data).unwrap(),
                            )))
                            .await
                            .unwrap();

                        let mut plugin = plugin_manager.lock().await;
                        plugin
                            .get_mut(&plugin_id)
                            .unwrap()
                            .set_server_sink(sink_arc.clone());

                        log::info!("\t[plugin.server]: plugin session started: {}", plugin_id);
                    }
                    // PluginMessageData::Success { .. } => (),
                    PluginMessage::Message { .. } => (),
                    // PluginMessageData::Action { .. } => (),
                    _ => (),
                }
            }
        }
    }
}
