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

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// mod ardeck_serial;
// mod ardeck_data;

mod ardeck_studio;
mod service;

use core::panic;
use std::{
    collections::HashMap,
    fs::{self, File},
    hash::Hash,
    io,
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc, Mutex, OnceLock,
    },
    thread::{self, park_timeout},
    time::Duration,
};

use once_cell::sync::{Lazy, OnceCell};

use ardeck_studio::{
    ardeck::{
        self, Ardeck 
    },
    plugin::{self, manager::PluginManager, PluginManifestJSON, PLUGIN_DIR},
};

use chrono::{format, Utc};

use serde::{Deserialize, Serialize};
use serialport::SerialPort;
use tauri::{
    AppHandle, CustomMenuItem, Manager, State as TauriState, SystemTray, SystemTrayEvent,
    SystemTrayMenu, SystemTrayMenuItem,
};
use window_shadows::set_shadow;

#[tokio::main]
async fn main() {
    // システムトレイアイコンの設定
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let tray_menu = SystemTrayMenu::new()
        .add_item(hide)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);
    let tray = SystemTray::new().with_menu(tray_menu);

    let ardeck_manager: Mutex<HashMap<String, Ardeck>> = Mutex::new(HashMap::new());

    tauri::Builder::default()
        .manage(ardeck_manager)
        .setup(|app| {
            let for_manage = app.app_handle();
            app.manage(Mutex::new(for_manage));

            let window = app.get_window("main").unwrap();
            window.show().unwrap();

            #[cfg(any(windows, target_os = "macos"))]
            set_shadow(window, true).unwrap(); // Windowに影や角丸などの装飾を施す

            Ok(())
        })
        .system_tray(tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::DoubleClick {
                position: _,
                size: _,
                ..
            } => {
                let window = app.get_window("main").unwrap();
                window.show().unwrap();
                window.set_focus().unwrap();
            }
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "hide" => {
                    let window = app.get_window("main").unwrap();
                    window.hide().unwrap();
                }
                "quit" => {
                    std::process::exit(0);
                }
                _ => {}
            },
            _ => {}
        })
        .plugin(tauri_plugin_log::Builder::default().build())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(ardeck_studio::ardeck::tauri::init())
        .plugin(ardeck_studio::plugin::tauri::init().await)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
