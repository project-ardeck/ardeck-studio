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

use once_cell::sync::Lazy;
use tauri::{
    plugin::{Builder, TauriPlugin},
    RunEvent, Runtime,
};
use tokio::sync::Mutex;

use crate::{ardeck_studio::switch_info::SwitchInfo, service::dir::Directories};

use super::server::PluginServer;

static PLUGIN_SERVER: Lazy<Mutex<PluginServer>> = Lazy::new(|| Mutex::new(PluginServer::new()));

async fn server_init() {
    log::info!("Initializing plugin server...");

    let mut server = PLUGIN_SERVER.lock().await;
    // Directories::init(Directories::get_plugin_dir().unwrap()).unwrap();
    let plugin_dir = match Directories::get_plugin_dir() {
        Ok(dir) => dir,
        Err(e) => {
            log::error!("[init]  Failed to get plugin dir: {}", e);
            return;
        }
    };

    if let Err(e) = Directories::init(plugin_dir) {
        log::error!("[init] Failed to init plugin dir: {}", e);
        return;
    };

    match server.start().await {
        Ok(_) => {
            log::info!("Plugin server started.");
            server.execute_plugin_all().await;
        }
        Err(e) => log::error!("Failed to start plugin server: {}", e),
    };
}

pub async fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("ardeck-plugin")
        .setup(|app| {
            tokio::spawn(async {
                server_init().await;
            });

            Ok(())
        })
        .build()
}

pub async fn send_action_to_plugins(data: SwitchInfo) {
    PLUGIN_SERVER.lock().await.put_action(data.clone()).await;
}
