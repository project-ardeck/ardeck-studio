// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod ardeck_serial;
use std::{sync::{Mutex, WaitTimeoutResult}, thread::{self, park_timeout}, time::Duration};

use ardeck_serial::ArdeckSerial;
use once_cell::sync::Lazy;
use tauri::{
    CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
};
use window_shadows::set_shadow;

// ArdeckSerialを格納するグローバル変数
static ARDECK_SERIAL: Lazy<Mutex<ArdeckSerial>> = Lazy::new(|| Mutex::new(ArdeckSerial::new()));

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// ポートの一覧を取得する
#[tauri::command]
fn get_ports() -> Vec<serialport::SerialPortInfo> {
    println!("get_ports");
    let ports = ARDECK_SERIAL.lock().unwrap().refresh_ports();
    // let ports = ArdeckSerial::get_ports();
    println!("got.");
    ports
}

// 指定されたCOMポートに接続して、リッスンする
#[tauri::command]
fn open_port(port_name: &str) {
    // 結果が返るようにする
    println!("port!!!");
    let mut sp = ARDECK_SERIAL.lock().unwrap();

    if sp.get_state() == 1 {
        println!("SP Using");
        return;
    }

    let mut port = sp.open(port_name.to_string());

    let mut port = match port {
        Ok(p) => p,
        Err(error) => {
            match error.kind() {
                serialport::ErrorKind::NoDevice => {
                    println!("device Not found!")
                }
                serialport::ErrorKind::InvalidInput => {
                    println!("device invalid input")
                }
                serialport::ErrorKind::Unknown => {
                    println!("Unknown Error, F*ck")
                }
                serialport::ErrorKind::Io(io_err) => {
                    println!("IO Error")
                }
                _ => {
                    panic!("WTF Panic!!!")
                }
            }

            return;
        }
    };

    thread::spawn(move || loop {
        let mut serial_buf: Vec<u8> = vec![0; 1];
        let serial_msg = port.read(&mut serial_buf.as_mut_slice());

        match serial_msg {
            Ok(msg) => msg,
            Err(error) => {
                let mut _1sp = ARDECK_SERIAL.lock().unwrap();
                _1sp.reset();
                println!("Connection Err, Connetion Stoped.");
                break;
            }
        };

        let is_str = String::from_utf8_lossy(&serial_buf);


        // println!("{}", x);
    });
}

#[tauri::command]
async fn reset_port() {
    let mut sp = ARDECK_SERIAL.lock().unwrap();

    if sp.get_state() == 0 {
        println!("already stoped");
        return;
    }

    sp.reset();


}

fn serial(app: tauri::AppHandle) {
    // TODO: Arduinoを接続したときのシリアル情報を読み取る
    // TODO: シリアル情報にオリジナル情報を追加できないか？
    // TODO: Arduino ファームウェア書き換え
    let refresh_fps = 1000 / 15;

    thread::spawn(move || loop {
        let ports = ArdeckSerial::get_ports();
        app.emit_all("on_ports", ports).unwrap();
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

    let serialport = ArdeckSerial::new();

    tauri::Builder::default()
        .setup(|app| {
            // serialが来たら処理する場所を追加
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
            SystemTrayEvent::MenuItemClick { tray_id, id, .. } => match id.as_str() {
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
            greet, get_ports, open_port, reset_port
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// [{"port_name":"COM3","port_type":{"UsbPort":{"vid":9025,"pid":32822,"serial_number":"HIDPC","manufacturer":"Arduino LLC (www.arduino.cc)","product":"Arduino Leonardo (COM3)"}}}]