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

use std::fs::File;
use std::net::SocketAddr;
use std::path::Path;
use std::sync::Arc;

use futures_util::stream::SplitSink;
use futures_util::{SinkExt, StreamExt};
use once_cell::sync::Lazy;
use tokio::sync::Mutex;

use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::Utf8Bytes;
use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message};

use crate::ardeck_studio::action::Action;
use crate::service::dir::Directories;

use super::manager::PluginManager;

use super::{
    Plugin, PluginAction, PluginManifestJSON, PluginMessage, PluginMessageData, PluginOpCode,
    PLUGIN_DIR,
};

static PLUGIN_MANAGER: Lazy<Mutex<PluginManager>> = Lazy::new(|| Mutex::new(PluginManager::new()));

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

    pub async fn start(&mut self) -> Result<(), std::io::Error> {
        println!("plugin1");
        let plugin_manager = Arc::clone(&self.plugin_manager);
        // 接続待ち
        self.listener = Some(tokio::spawn(async move {
            let tcp = TcpListener::bind("127.0.0.1:6725").await.unwrap();
            println!("plugin server started.");

            // 接続
            while let Ok((stream, _)) = tcp.accept().await {
                let peer = stream
                    .peer_addr()
                    .expect("connected streams should have a peer address");

                println!("Peer address: {}", peer);

                tokio::spawn(handle_connection(peer, stream, plugin_manager.clone()));
            }
        }));

        Ok(())
    }

    pub async fn execute_plugin_all(&self) {
        // let dir = Directories::get_or_init(Path::new(PLUGIN_DIR)).unwrap();
        let dir = match Directories::get(Path::new(PLUGIN_DIR)) {
            Ok(read_dir) => read_dir,
            Err(_) => {
                println!("[plugin.core]: plugins dir is not found");
                return;
            }
        };

        for entry in dir {
            if entry.is_err() {
                continue;
                // TODO: Error
            }

            let path = entry.unwrap().path();

            // マニフェストファイルを取得
            let manifest_file = File::open(format!("{}/manifest.json", path.display()));
            if manifest_file.is_err() {
                println!("Failed to open manifest.json");
                continue;
            }

            // アクションファイルを取得
            let actions_file = File::open(format!("{}/actions.json", path.display()));
            if manifest_file.is_err() {
                println!("Failed to open actions.json");
                continue;
            }

            let manifest: PluginManifestJSON =
                serde_json::from_reader(manifest_file.unwrap()).unwrap();
            let actions: Vec<PluginAction> =
                serde_json::from_reader(actions_file.unwrap()).unwrap();

            // プラグインの実行ファイルのパスを取得
            let plugin_main_path = format!("{}/{}", path.display(), manifest.main);

            // プラグインを実行
            let process = std::process::Command::new(plugin_main_path)
                .spawn()
                .expect("Failed to execute plugin");

            // プラグイン情報とプロセスをマネージャーに登録
            self.plugin_manager
                .lock()
                .await
                .insert(
                    manifest.clone().id,
                    Plugin::new(manifest, actions, Arc::new(Mutex::new(process))),
                )
                .unwrap();
        }
    }

    pub fn put_action(&self, action: Action) {
        // TODO: switch_typeとswitch_idからマッピングの設定を見つけ、そのプラグインに（あれば）put_actionする
    }
}

// セッション
async fn handle_connection(
    peer: SocketAddr,
    stream: TcpStream,
    plugin_manager: Arc<Mutex<PluginManager>>,
) {
    let mut ws_stream = accept_async(stream).await.expect("Failed to accept");

    let (mut sink, mut stream) = ws_stream.split();

    while let Some(msg) = stream.next().await {
        if let Ok(msg) = msg {
            if msg.is_text() {
                let msg_str = msg.to_text().unwrap();

                println!("Received: {}", msg_str);

                let message: PluginMessage = serde_json::from_str(msg_str).unwrap();

                match message.data {
                    PluginMessageData::Hello {
                        plugin_version,
                        ardeck_plugin_web_socket_version,
                        plugin_id,
                    } => {
                        println!("Hello:\n\t{}", plugin_id);

                        let data = PluginMessageData::Success {
                            ardeck_studio_version: "0.1.4".to_string(),
                            ardeck_studio_web_socket_version: "0.0.1".to_string(),
                        };

                        sink
                            .send(Message::Text(Utf8Bytes::from(
                                &serde_json::to_string(&data).unwrap(),
                            )))
                            .await
                            .unwrap();
                    }
                    // PluginMessageData::Success { .. } => (),
                    PluginMessageData::Message { .. } => (),
                    // PluginMessageData::Action { .. } => (),
                    _ => (),
                }
            }
        }
    }
}
