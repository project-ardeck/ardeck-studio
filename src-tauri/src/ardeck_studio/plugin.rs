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

pub mod manager;
pub mod server;
pub mod tauri;

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use server::PluginServerSink;
use std::sync::Arc;

use tokio::{net::TcpStream, sync::Mutex};

use super::{action::Action, switch_info::SwitchInfo};

pub static PLUGIN_DIR: &'static str = "./plugins";

#[derive(Debug)]
pub struct Plugin {
    pub manifest: PluginManifestJSON, //TODO: PluginManifest
    pub actions: PluginActionJSON,
    pub process: Arc<Mutex<std::process::Child>>,
    pub session: Option<Arc<Mutex<TcpStream>>>,
    pub server_sink: Option<Arc<Mutex<PluginServerSink>>>,
}

impl Plugin {
    pub fn new(
        manifest: PluginManifestJSON,
        actions: PluginActionJSON,
        process: Arc<Mutex<std::process::Child>>,
        // session: Arc<Mutex<WebSocket>>
    ) -> Plugin {
        Plugin {
            manifest,
            actions,
            process,
            session: None,
            server_sink: None,
        }
    }

    pub fn set_session(&mut self, session: Arc<Mutex<TcpStream>>) {
        self.session = Some(session);
    }

    pub async fn put_action(&mut self, action_id: String, action: SwitchInfo) {
        if self.session.is_none() {
            // Error!: Plugin session has not started yet.
            return;
        }

        // let action_message = PluginMessage {
        //     op: PluginOpCode::Action,
        //     data: PluginMessageData::Action {
        //         action_id,
        //         action_data: ActionMap::from(action),
        //     },
        // };

        // let action_str = serde_json::to_string(&action_message).unwrap();
        // self.session
        //     .as_mut()
        //     .unwrap()
        //     .lock()
        //     .await
        //     .send(axum::extract::ws::Message::Text(action_str))
        //     .await
        //     .unwrap();
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PluginManifestJSON {
    pub name: String,
    pub version: String,
    pub id: String,
    pub description: Option<String>,
    pub author: String,
    pub main: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PluginAction {
    name: String,
    id: String,
    description: Option<String>,
}

type PluginActionJSON = Vec<PluginAction>;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PluginMessage {
    pub op: PluginOpCode,
    pub data: PluginMessageData,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase", tag = "op", content = "data")] // TODO: opが数字でなく文字列で変換されてしまう問題
pub enum PluginMessageData {
    #[serde(rename = "0")]
    Hello {
        // OP0: Hello
        plugin_version: String,
        ardeck_plugin_web_socket_version: String,
        plugin_id: String,
    },
    #[serde(rename = "1")]
    Success {
        // OP1: Success
        ardeck_studio_version: String,
        ardeck_studio_web_socket_version: String,
    },
    #[serde(rename = "2")]
    Message {
        // OP2: Message
        message_id: String,
        message: String,
    },
    #[serde(rename = "3")]
    Action(Action),
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[repr(i32)]
pub enum PluginOpCode {
    Hello,
    Success,
    Message,
    Action,
}

// TODO: add host.rs
