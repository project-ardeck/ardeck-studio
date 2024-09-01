pub mod core;
pub mod manager;

use manager::{
    PluginManager,
};
use core::{
    PluginServe
};

use std::sync::{
    Arc,
    Mutex,
};
use axum::extract::ws::WebSocket;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

pub static PLUGIN_DIR: &'static str = "./plugins";

#[derive(Clone, Debug)]
pub struct Plugin {
    pub manifest: PluginManifest, //TODO: PluginManifest
    pub process: Arc<Mutex<std::process::Child>>,
    pub session: Arc<Mutex<WebSocket>>,
}

impl Plugin {
    pub fn new(
        manifest: PluginManifest,
        process: Arc<Mutex<std::process::Child>>,
        session: Arc<Mutex<WebSocket>>
    ) -> Plugin {
        Plugin {
            manifest,
            process,
            session
        }
    }
}
// struct Builder {}
// impl Builder {
// }




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

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PluginMessage {
    pub op: PluginOpCode,
    pub data: PluginMessageData
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PluginMessageData {
    pub log: Option<String>,
    pub action_id: Option<String>,
}

impl Default for PluginMessageData {
    fn default() -> Self {
        PluginMessageData {
            log: None,
            action_id: None
        }
    }
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[repr(i32)]
pub enum PluginOpCode {
    Authorize = 0,              // plugin -> host
    AuthorizeSuccess = 1,       // host -> plugin
    Message = 2,                // plugin <-> host
    Action = 3,                 // host -> plugin
    Log = 4,                    // plugin -> host
    Error = -1                 // plugin <-> host
}
