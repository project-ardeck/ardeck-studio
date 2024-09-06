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
        mpsc::{channel, Receiver, Sender}, Arc, Mutex, OnceLock
    },
    thread::{self, park_timeout},
    time::Duration,
};

use once_cell::sync::{Lazy, OnceCell};

use ardeck_studio::{
    ardeck::{
        command::ArdeckCommand, data::{ActionData, ArdeckData}, manager::ArdeckManager, Ardeck
    }, plugin::{core::PluginServe, manager::PluginManager, PluginManifest, PLUGIN_DIR}, service::settings::{DeviceSettingOptions, DeviceSettings}
};

use chrono::{format, Utc};

use serde::{Deserialize, Serialize};
use serialport::SerialPort;
use tauri::{
    plugin, AppHandle, CustomMenuItem, Manager, State as TauriState, SystemTray, SystemTrayEvent,
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

#[tauri::command]
async fn open_port(
    ardeck_manager: TauriState<>
    port_name: &str,
    baud_rate: u32
) -> Result<u32, u32> {
    // # protocol_version バージョンの考案日付
    // "2024-0-17": [DATA] ... 未実装
    // "2014-06-03": 'A', 'D', 'E', 'C', [DATA] ... Cを受信したタイミングで(リセットなどで)接続が途絶え、再度接続された際にデータがずれる問題が発生する
    // "2024-06-17": 'A', 'D', [DATA], 'E', 'C' ...

    println!("[{}] Opening", port_name);

    // すでに接続中であればエラーを投げて終わる
    if is_connecting_serial(&port_name.to_string()) {
        println!("[{}] Already Opened.", port_name);
        return Err(501);
    }

    // 接続開始
    let serial = Ardeck::open(&port_name.to_string(), baud_rate);

    let command = ArdeckCommand::new();

    match serial {
        Ok(ardeck_serial) => {
            // let ardeck_serial = Arc::new(Mutex::new(ardeck_serial));
            println!("[{}] Opened", port_name);

            // 5秒間受信しなければエラー
            ardeck_serial
                .port()
                .set_timeout(Duration::from_millis(5000))
                .unwrap();

            // 受信したデータが正しければ、プラグインの部分に投げる
            ardeck_serial
                .port_data()
                .on_complete(move |data| {
                    TAURI_APP
                        .get()
                        .unwrap()
                        .emit_all("on-message-serial", data)
                        .unwrap();
                    // command.on_data(data);
                });

            let port_name_for_thread = port_name.to_string().clone();
            tokio::spawn(async move {
                loop {
                    // println!("[{}] Thread Start", port_name_for_thread);

                    let mut serials = ARDECK_MANAGER.lock().unwrap().clone();
                    let serial = serials.get_mut(&port_name_for_thread.to_string());

                    if serial.is_none() {
                        return;
                    }

                    if serial
                        .as_ref()
                        .unwrap()
                        .lock()
                        .unwrap()
                        .continue_flag()
                        .load(std::sync::atomic::Ordering::SeqCst)
                        == false
                    {
                        ARDECK_MANAGER
                            .lock()
                            .unwrap()
                            .remove(&port_name_for_thread.to_string())
                            .unwrap();

                        TAURI_APP
                            .get()
                            .unwrap()
                            .emit_all("on-close-serial", port_name_for_thread.clone())
                            .unwrap();

                        println!(
                            "[{}] Connection Stoped for Bool.",
                            port_name_for_thread.to_string()
                        );

                        break;
                    }

                    let mut serial_buf: Vec<u8> = vec![0; 1];

                    // let port_arc = ;
                    let mut port = serial.unwrap().port();

                    // println!("[{}] Reading...", port_name_for_thread);

                    let try_read = port.read(&mut serial_buf);

                    match try_read {
                        Ok(_) => {
                            // println!("[{}] Readed", port_name_for_thread);

                            let ardeck_data = serials
                                .get_mut(&port_name_for_thread.to_string())
                                .unwrap()
                                .port_data();
                            ardeck_data.on_data(serial_buf);
                        }
                        Err(Kind) => {
                            println!(
                                "[{}] Connection Err, Connetion Stoped.",
                                port_name_for_thread.to_string()
                            );
                            println!("Kind: {:?}", Kind);

                            ARDECK_MANAGER
                                .lock()
                                .unwrap()
                                .remove(&port_name_for_thread.to_string())
                                .unwrap();

                            TAURI_APP
                                .get()
                                .unwrap()
                                .emit_all("on-close-serial", port_name_for_thread.clone())
                                .unwrap();

                            println!(
                                "[{}] Connection Stoped for Error.",
                                port_name_for_thread.to_string()
                            );

                            // TODO: エラーの理由がどうであれ、ディスコネクト要求があるまでエラーイベントを発火させてループを続ける

                            break;
                        }
                    }
                }
            });

            let mut serials = ARDECK_MANAGER.lock().unwrap();
            serials.insert(port_name.to_string(), ardeck_serial);
            TAURI_APP
                .get()
                .unwrap()
                .emit_all("on-open-serial", port_name)
                .unwrap();

            Ok(200)
        }
        Err(_) => {
            println!("Open Error !!!!!! {}", port_name);

            Err(500)
        }
    }
}

#[tauri::command]
async fn close_port(port_name: &str) -> Result<u32, u32> {
    let mut serials = ARDECK_MANAGER.lock().unwrap();

    let target_port = port_name.to_string();
    let serial = serials.get_mut(&target_port);
    if !serial.is_none() {
        println!("[{}] closing...", target_port);

        serial
            .unwrap()
            .continue_flag()
            .store(false, std::sync::atomic::Ordering::SeqCst);
    }

    Ok(200)
}

// 現在接続中のポートの名前一覧を取得する
#[tauri::command]
fn get_connecting_serials() -> Vec<String> {
    let serials = ARDECK_MANAGER.lock().unwrap();

    let keys = serials.keys();
    keys.cloned().collect()
}

#[tauri::command]
fn test(state1: TauriState<Mutex<AppHandle>>, state2: TauriState<Mutex<_AppData>>) {
    state1.lock().unwrap().emit_all("test", "");
    println!("{:?}", state2.lock().unwrap().welcome_message);
}

fn serial() {
    // ポートリストを定期的に更新し、イベントを発火する
    let refresh_fps = 1000 / 4;
    thread::spawn(move || {
        let tauri_app_port_list = TAURI_APP.get().unwrap().clone();
        let mut last_ports: Vec<serialport::SerialPortInfo> = vec![];
        loop {
            let ports = serialport::available_ports().unwrap();

            if last_ports.clone() != ports.clone() {
                tauri_app_port_list
                    .emit_all("on-ports", ports.clone())
                    .unwrap();
            }

            last_ports = ports;

            park_timeout(Duration::from_millis(refresh_fps));
        }
    });
}

async fn init_plugin_serve() {
    // let aaa = PLUGIN_MANAGER.get().unwrap();
    // let serve = PluginServe::init(aaa);
}

async fn init_plugin() {
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
    let ardeck_manager: Arc<Mutex<HashMap<String, Ardeck>>> = Arc::new(Mutex::new(HashMap::new()));

    tauri::Builder::default()
        .setup(|app| {
            let for_serial_app = app.app_handle();
            TAURI_APP.get_or_init(|| for_serial_app);
            // serial(for_serial_app);
            serial();

            let _for_manage = app.app_handle();
            app.manage(Mutex::new(_for_manage));
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
            get_connecting_serials,
            open_port,
            close_port,
            test
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// [{"port_name":"COM3","port_type":{"UsbPort":{"vid":9025,"pid":32822,"serial_number":"HIDPC","manufacturer":"Arduino LLC (www.arduino.cc)","product":"Arduino Leonardo (COM3)"}}}]
