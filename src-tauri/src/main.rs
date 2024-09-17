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
mod util;

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
    plugin::{self, manager::PluginManager, PluginManifest, PLUGIN_DIR},
    service::settings::{DeviceSettingOptions, DeviceSettings},
};

use chrono::{format, Utc};

use serde::{Deserialize, Serialize};
use serialport::SerialPort;
use tauri::{
    AppHandle, CustomMenuItem, Manager, State as TauriState, SystemTray, SystemTrayEvent,
    SystemTrayMenu, SystemTrayMenuItem,
};
use window_shadows::set_shadow;

// フロントエンドから送られてくるシリアル通信を開く際の要求
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct PortRequest {
    target_port: String,
}

// シリアル通信の状態変更などをスレッドをまたいで指示する際のメッセージ
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct SerialPortThreadMessage {
    event: String,
    target_port: String,
}

// シリアル通信中のデータをフロントエンドに送る際のメッセージ
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct OnMessageSerial {
    data: u8,
    timestamp: i64,
}

// シリアル通信中のSerialportトレイトを格納する
static ARDECK_MANAGER: Lazy<Mutex<HashMap<String, Arc<Mutex<Ardeck>>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));
static TAURI_APP: OnceCell<tauri::AppHandle> = OnceCell::new();
static PLUGIN_MANAGER: OnceCell<Mutex<PluginManager>> = OnceCell::new();

// 指定されたポート名が、現在接続中のポート一覧に存在するかどうかを確認する
fn is_connecting_serial(port_name: &String) -> bool {
    let serials = ARDECK_MANAGER.lock().unwrap();

    let tryget = serials.get(port_name);

    tryget.is_some()
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// ポートの一覧を取得する
#[tauri::command]
fn get_ports() -> Vec<serialport::SerialPortInfo> {
    println!("get_ports");
    let ports = serialport::available_ports().unwrap();
    println!("got.");
    ports
}

#[tauri::command]
fn get_device_settings() -> Vec<DeviceSettingOptions> {
    DeviceSettings::get_settings().unwrap()
}

// #[tauri::command]
// fn test(state1: TauriState<'_, Arc<Mutex<AppHandle>>>, state2: TauriState<Mutex<_AppData>>) {
//     state1.lock().unwrap().emit_all("test", "");
//     println!("{:?}", state2.lock().unwrap().welcome_message);
// }

async fn init_plugin_serve() {
    // let aaa = PLUGIN_MANAGER.get().unwrap();
    // let serve = PluginServe::init(aaa);
}

async fn init_plugin() {
    return;
    PLUGIN_MANAGER.get_or_init(|| Mutex::new(PluginManager::new()));

    tokio::spawn(init_plugin_serve());

    let plugin_dir = fs::read_dir(PLUGIN_DIR).unwrap();

    println!("Loading plugins...");

    for entry in plugin_dir {
        let entry = entry.unwrap(); // TODO: match
        let path = entry.path();

        let manifest_file = File::open(format!("{}/manifest.json", path.display()));
        if manifest_file.is_err() {
            println!("Failed to open manifest.json");
            continue;
        }

        let manifest: PluginManifest = serde_json::from_reader(manifest_file.unwrap()).unwrap();

        println!("Loaded plugin manifest: {}", manifest.name);

        let plugin_main_path = format!("{}/{}", path.display(), manifest.main);

        let plugin_process = std::process::Command::new(plugin_main_path)
            .spawn()
            .expect("Failed to execute plugin");
    }
}

struct _AppData {
    welcome_message: &'static str,
}

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

    tokio::spawn(init_plugin());

    // TODO: Lazy to Arc
    let ardeck_manager: Mutex<HashMap<String, Ardeck>> = Mutex::new(HashMap::new());

    tauri::Builder::default()
        .manage(ardeck_manager)
        .setup(|app| {
            let for_serial_app = app.app_handle();
            TAURI_APP.get_or_init(|| for_serial_app);
            // serial(for_serial_app);
            // serial();
            let for_manage = app.app_handle();
            app.manage(Mutex::new(for_manage));
            let _app_data = _AppData {
                welcome_message: "WelcmeToTOTOTOtTToToTOt",
            };
            app.manage(Mutex::new(_app_data));

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
        .invoke_handler(tauri::generate_handler![
            greet,
            get_ports,
            // test
        ])
        .plugin(ardeck_studio::ardeck::tauri::init())
        .plugin(ardeck_studio::plugin::tauri::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// [{"port_name":"COM3","port_type":{"UsbPort":{"vid":9025,"pid":32822,"serial_number":"HIDPC","manufacturer":"Arduino LLC (www.arduino.cc)","product":"Arduino Leonardo (COM3)"}}}]
