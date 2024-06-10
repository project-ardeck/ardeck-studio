// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod ardeck_serial;
mod ardeck_data;

use std::{
    collections::HashMap,
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc, Mutex, OnceLock
    },
    thread::{self, park_timeout},
    time::Duration,
};

// use ardeck_serial::{ArdeckSerial, ArdeckSerial2};
use ardeck_data::ArdeckData;
use serde::{Deserialize, Serialize};
use serialport::SerialPort;
use tauri::{
    CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
};
use window_shadows::set_shadow;


static SERIAL_MAP: OnceLock<Mutex<HashMap<String, Box<dyn SerialPort>>>> = OnceLock::new(); // TODO: SERIAL_MAPって名前よくないね

fn get_serial_map() -> &'static Mutex<HashMap<String, Box<dyn SerialPort>>> {
    SERIAL_MAP.get_or_init(|| Mutex::new(HashMap::new()))
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
fn get_connecting_serials() -> Vec<String> {
    let serials = get_serial_map().lock().unwrap();

    vec!["TEST".to_string(), "aaa".to_string(), "fe".to_string(), ""fef.to_string()]
}

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

fn serial(app: tauri::AppHandle) {
    let (tx, rx): (
        Sender<SerialPortThreadMessage>,
        Receiver<SerialPortThreadMessage>,
    ) = channel();

    
    // TODO: これを関数外において、tauri:commandから呼べるようにできないか？
    let serial_map: Arc<Mutex<HashMap<String, Box<dyn SerialPort>>>> =
    Arc::new(Mutex::new(HashMap::new()));
    
    // 接続の開始要求を受信するリスナー
    let tx_openreq = tx.clone();
    app.listen_global("request-open-serial", move |event| {
        let msg_str = event.payload();
        let msg: PortRequest = serde_json::from_str(&msg_str.unwrap()).unwrap();
        println!("msg!!:: {}", msg.target_port);

        if msg.target_port == "" {
            return;
        }

        tx_openreq
            .send(SerialPortThreadMessage {
                event: "open".to_string(),
                target_port: msg.target_port,
            })
            .unwrap();
    });

    // 接続終了の要求を受信するリスナー
    let tx_closereq = tx.clone();
    app.listen_global("request-close-serial", move |event| {
        let msg_str = event.payload();
        let msg: PortRequest = serde_json::from_str(&msg_str.unwrap()).unwrap();

        if msg.target_port == "" {
            return;
        }

        println!("closing...");

        tx_closereq
            .send(SerialPortThreadMessage {
                event: "close".to_string(),
                target_port: msg.target_port,
            })
            .unwrap();
    });

    
    // ポートリストを定期的に更新する
    let refresh_fps = 1000 / 5;
    let app_port_list = app.clone();
    thread::spawn(move || loop {
        let ports = serialport::available_ports().unwrap();
        app_port_list.emit_all("on-ports", ports).unwrap();

        park_timeout(Duration::from_millis(refresh_fps));
    });

    
    // イベントリスナーからのメッセージを取得してそれなりに処理を行う。
    let app_serial_port = app.clone();
    thread::spawn(move || loop {
        let thread_message = rx.recv();
        
        match thread_message {
            Ok(e) => {
                match e.event.as_str() {
                    "open" => { // 通信の開始要求
                        let target_port = e.target_port.clone();
                        let mut serials = serial_map.lock().unwrap();

                        println!("Opening {}", &e.target_port);
                        let port_check = serials.get(&target_port);
                        if !port_check.is_none() {
                            println!("Already Opened.");
                            app_serial_port
                                .emit_all("on-error-serial", "Already Opened.")
                                .unwrap();

                            continue;
                        }

                        // 接続開始
                        let serial = serialport::new(&target_port, 9600).open();
                        match serial {
                            Ok(sp) => {
                                println!("Opened {}", &target_port);

                                let mut sp_for_listen = sp.try_clone().unwrap();
                                let mut sp_data_fmt = ArdeckData::new();

                                app_serial_port.emit_all("on-open-serial", &sp_for_listen.name().unwrap()).unwrap();

                                let sel = serial_map.clone();
                                let tgn = target_port.clone();

                                let app_serial_collect = app_serial_port.clone();

                                // Ardeckから送信されてきたデータが正常なデータであればイベントを発火する
                                sp_data_fmt.on_collect(move |data| {
                                    println!("OnCollect In Closure");
                                    
                                    // 入力から操作用トレイトに投げる

                                    app_serial_collect.emit_all("on-message-serial", data.to_string()).unwrap();
                                });

                                let app_serial_close = app_serial_port.clone();
                                // Ardeckから送信されてきたデータを受信し、正しいデータかどうかを確認する
                                thread::spawn(move || loop {
                                    let mut serial_buf: Vec<u8> = vec![0; 1];
                                    let serial_msg = sp_for_listen.read(&mut serial_buf.as_mut_slice());

                                    match serial_msg {
                                        Ok(msg) => msg,
                                        Err(_) => {
                                            println!("Connection Err, Connetion Stoped.");
                                            
                                            sel.lock().unwrap().remove(&tgn).unwrap();

                                            app_serial_close.emit_all("app_serial_close", tgn).unwrap();

                                            break;
                                        }
                                    };
                                    
                                    sp_data_fmt.on_data(serial_buf);

                                });


                                // シリアル通信中のリストに追加する。
                                serials.insert(target_port, sp);

                            }
                            Err(_) => {
                                println!("Open Error !!!!!! {}", target_port);
                                app_serial_port
                                    .emit_all("on-error-serial", "WTF Errorro")
                                    .unwrap();
                            }
                        }
                    }
                    "close" => {
                        let target_port = e.target_port.clone();
                        let mut sp = serial_map.lock().unwrap();
                        let serial = sp.get(&target_port);
                        if !serial.is_none() {
                            let try_break = serial.unwrap().set_break();
                            match try_break {
                                Ok(()) => {
                                    sp.remove(&target_port).unwrap();

                                    app_serial_port.emit_all("on-close-serial", target_port).unwrap();
                                }
                                Err(_) => {}
                            }
                        }


                    }
                    _ => {}
                }
            }
            Err(_) => {}
        };
    });

    // mainserial.join().unwrap();
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
            serial(for_serial_app);

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
            greet, get_ports, /*open_port, reset_port*/
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// [{"port_name":"COM3","port_type":{"UsbPort":{"vid":9025,"pid":32822,"serial_number":"HIDPC","manufacturer":"Arduino LLC (www.arduino.cc)","product":"Arduino Leonardo (COM3)"}}}]
