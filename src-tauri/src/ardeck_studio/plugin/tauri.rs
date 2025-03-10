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
    generate_handler,
    plugin::{Builder, TauriPlugin},
    RunEvent, Runtime,
};
use tokio::sync::Mutex;

use crate::{ardeck_studio::switch_info::SwitchInfo, service::dir::Directories};

use super::{server::PluginServer, PluginActionJSON, PluginManifestJSON};

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

#[tauri::command]
async fn get_plugin_manifests<R: Runtime>(
    _app: tauri::AppHandle<R>,
) -> Result<Vec<PluginManifestJSON>, String> {
    let plugin_manager = PLUGIN_SERVER.lock().await.get_plugin_manager().await;

    let manifests: Vec<PluginManifestJSON> = plugin_manager
        .lock()
        .await
        .iter()
        .map(|(_id, plugin)| plugin.manifest.clone())
        .collect();

    Ok(manifests)
}

#[tauri::command]
async fn get_plugin_actions<R: Runtime>(
    app: tauri::AppHandle<R>,
    plugin_id: String,
) -> Result<PluginActionJSON, String> {
    let plugin_server = PLUGIN_SERVER.lock().await;

    if let Some(plugin) = plugin_server
        .get_plugin_manager()
        .await
        .lock()
        .await
        .get(&plugin_id)
    {
        return Ok(plugin.actions.clone());
    } else {
        return Err("Plugin not found".to_string());
    }
}

pub async fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("ardeck-plugin")
        .setup(|app| {
            tokio::spawn(async {
                server_init().await;
            });

            Ok(())
        })
        .invoke_handler(generate_handler![get_plugin_manifests, get_plugin_actions])
        .build()
}

pub async fn send_action_to_plugins(data: SwitchInfo) {
    PLUGIN_SERVER.lock().await.put_action(data.clone()).await;
}
