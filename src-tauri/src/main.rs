// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod ardeck_serial;

use std::{
    collections::HashMap,
    sync::{
        mpsc::{channel, Receiver, Sender},
        Mutex, OnceLock,
    },
    thread::{self, park_timeout},
    time::Duration,
};

use ardeck_serial::ArdeckSerial;

use serde::{Deserialize, Serialize};
use serialport::SerialPort;
use tauri::{
    CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
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

// シリアル通信中のSerialportトレイトを格納する
// TODO: Boxの中身をSerialPort生ではなく、オリジナルのトレイト、その中のAtomicBoolでステートの管理を行うことによってブロッキングを回避する。
static SERIAL_MAP: OnceLock<Mutex<HashMap<String, ArdeckSerial>>> = OnceLock::new(); // TODO: SERIAL_MAPって名前よくないね
static TAURI_APP: OnceLock<Option<tauri::AppHandle>> = OnceLock::new();

// SERIAL_MAPのデータを取り出す
fn get_serial_map() -> &'static Mutex<HashMap<String, ArdeckSerial>> {
    SERIAL_MAP.get_or_init(|| Mutex::new(HashMap::new()))
}

// TAURI_APPのデータを取り出す
fn get_tauri_app() -> &'static tauri::AppHandle {
    TAURI_APP.get_or_init(|| None).as_ref().unwrap()
}

// 指定されたポート名が、現在接続中のポート一覧に存在するかどうかを確認する
fn is_connecting_serial(port_name: &String) -> bool {
    let serials = get_serial_map().lock().unwrap();

    let tryget = serials.get(port_name);

    if !tryget.is_none() {
        return true;
    } else {
        return false;
    }
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
async fn open_port(port_name: &str, baud_rate: u32) -> Result<u32, u32> {
    
    println!("[{}] Opening", port_name);
    
    // すでに接続中であればエラーを投げて終わる
    if is_connecting_serial(&port_name.to_string()) {
        println!("[{}] Already Opened.", port_name);
        return Err(501);
    }
        
    // 接続開始
    let serial = ArdeckSerial::open(&port_name.to_string(), baud_rate);

    match serial {
        Ok(ardeck_serial) => {
            println!("[{}] Opened", port_name);

            
            ardeck_serial.port().lock().unwrap().set_timeout(Duration::from_millis(5000)).unwrap();
            ardeck_serial.port_data().lock().unwrap().on_collect(move |data| {
                get_tauri_app()
                    .emit_all("on-message-serial", data)
                    .unwrap();
            });

            let port_name_for_thread = port_name.to_string().clone();
            thread::spawn(move || loop {
                // println!("[{}] Thread Start", port_name_for_thread);

                let mut serials = get_serial_map().lock().unwrap().clone();
                let serial = serials.get_mut(&port_name_for_thread.to_string());

                if serial.is_none() {
                    return;
                }

                let mut serial_buf: Vec<u8> = vec![0; 1];

                let port_arc = serial.unwrap().port();
                let mut port = port_arc.lock().unwrap();
                
                // println!("[{}] Reading...", port_name_for_thread);

                let try_read = port.read(&mut serial_buf);

                match try_read {
                    Ok(_) => {
                        // println!("[{}] Readed", port_name_for_thread);

                        let ardeck_data = serials.get_mut(&port_name_for_thread.to_string()).unwrap().port_data();
                        ardeck_data.lock().unwrap().on_data(serial_buf);
                    }
                    Err(_) => {
                        println!("[{}] Connection Err, Connetion Stoped.", port_name_for_thread.to_string());


                        get_serial_map().lock().unwrap().remove(&port_name_for_thread.to_string()).unwrap();

                        get_tauri_app()
                            .emit_all("on-close-serial", port_name_for_thread.clone())
                            .unwrap();

                        println!("[{}] Connection Stoped.", port_name_for_thread.to_string());

                        break;
                    }
                }
            });

            let mut serials = get_serial_map().lock().unwrap();
            serials.insert(port_name.to_string(), ardeck_serial);
            get_tauri_app()
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
    let mut serials = get_serial_map().lock().unwrap();

    let target_port = port_name.to_string();
    let serial = serials.get_mut(&target_port);
    if !serial.is_none() {
        println!("[{}] closing...", target_port);
        let try_break = serial.unwrap().port().lock().unwrap().set_break();
        match try_break {
            Ok(()) => {
                println!("[{}] closed.", target_port);

                serials.remove(&target_port).unwrap();

                get_tauri_app()
                    .emit_all("on-close-serial", target_port)
                    .unwrap();
            }
            Err(_) => {}
        }
    }

    Ok(200)
}

// 現在接続中のポートの名前一覧を取得する
#[tauri::command]
fn get_connecting_serials() -> Vec<String> {
    let serials = get_serial_map().lock().unwrap();

    let keys = serials.keys();
    keys.cloned().collect()
}

fn serial() {
    // ポートリストを定期的に更新し、イベントを発火する
    let refresh_fps = 1000 / 5;
    let tauri_app_port_list = get_tauri_app().clone();
    thread::spawn(move || loop {

        let ports = serialport::available_ports().unwrap();
        tauri_app_port_list.emit_all("on-ports", ports).unwrap();

        park_timeout(Duration::from_millis(refresh_fps));
    });
}

fn main() {
    // システムトレイアイコンの設定
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let tray_menu = SystemTrayMenu::new()
        .add_item(hide)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);
    let tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .setup(|app| {
            let for_serial_app = app.app_handle();
            TAURI_APP.get_or_init(|| Some(for_serial_app.clone()));
            // serial(for_serial_app);
            serial();

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
            close_port
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// [{"port_name":"COM3","port_type":{"UsbPort":{"vid":9025,"pid":32822,"serial_number":"HIDPC","manufacturer":"Arduino LLC (www.arduino.cc)","product":"Arduino Leonardo (COM3)"}}}]
