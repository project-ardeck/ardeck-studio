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

pub mod core;
pub mod manager;
pub mod tauri;

use core::PluginCore;
use manager::PluginManager;

use axum::extract::ws::WebSocket;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::sync::{Arc, Mutex};

use tokio::sync::Mutex as TokioMutex;

use super::action::{Action, SwitchId, SwitchType};

pub static PLUGIN_DIR: &'static str = "./plugins";

#[derive(Clone, Debug)]
pub struct Plugin {
    pub manifest: PluginManifest, //TODO: PluginManifest
    pub actions: PluginActionList,
    pub process: Arc<Mutex<std::process::Child>>,
    pub session: Option<Arc<TokioMutex<WebSocket>>>,
}

impl Plugin {
    pub fn new(
        manifest: PluginManifest,
        actions: PluginActionList,
        process: Arc<Mutex<std::process::Child>>,
        // session: Arc<Mutex<WebSocket>>
    ) -> Plugin {
        Plugin {
            manifest,
            actions,
            process,
            session: None,
        }
    }

    pub fn set_session(&mut self, session: Arc<TokioMutex<WebSocket>>) {
        self.session = Some(session);
    }

    pub async fn put_action(&mut self, action_id: String, action: Action) {
        if self.session.is_none() {
            // Error!: Plugin session has not started yet.
            return;
        }

        let action_message = PluginMessage {
            op: PluginOpCode::Action,
            data: PluginMessageData {
                action_id: Some(action_id),
                action_data: Some(action),
                ..Default::default()
            },
        };

        let action_str = serde_json::to_string(&action_message).unwrap();
        self.session
            .as_mut()
            .unwrap()
            .lock()
            .await
            .send(axum::extract::ws::Message::Text(action_str))
            .await
            .unwrap();
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PluginManifest {
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

type PluginActionList = Vec<PluginAction>;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PluginMessage {
    pub op: PluginOpCode,
    pub data: PluginMessageData,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PluginMessageData {
    pub ardeck_plugin_web_socket_version: Option<String>,

    // op0
    pub ardeck_studio_version: Option<String>,

    // op1
    pub plugin_version: Option<String>,
    pub plugin_id: Option<String>,

    // op8
    pub action_id: Option<String>,
    pub action_data: Option<Action>,

    pub log: Option<String>,
}

impl Default for PluginMessageData {
    fn default() -> Self {
        PluginMessageData {
            ardeck_plugin_web_socket_version: None,

            ardeck_studio_version: None,

            plugin_version: None,
            plugin_id: None,

            action_id: None,
            action_data: None,

            log: None,
        }
    }
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[repr(i32)]
pub enum PluginOpCode {
    // ardeck plugin websocket 0.0.1
    Hello = 0,     // host -> plugin
    Challenge = 1, // plugin -> host
    Success = 2,   // host -> plugin
    Error = 3,     // host <-> plugin
    Action = 8,    // host -> plugin
    Message = 9,   // host <-> plugin
}