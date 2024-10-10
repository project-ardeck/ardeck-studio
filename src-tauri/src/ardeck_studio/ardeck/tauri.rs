use std::{
    collections::HashMap, io, sync::{Arc, Mutex}, thread::park_timeout, time::Duration
};

use once_cell::sync::Lazy;
use serialport::{SerialPort, SerialPortInfo};
use tauri::{
    generate_handler,
    plugin::{Builder, Plugin, TauriPlugin},
    AppHandle, Invoke, Manager, Runtime, State as TauriState,
};

use crate::ardeck_studio::plugin;

use super::{manager::ArdeckManager, Ardeck};

static ARDECK_MANAGER: Lazy<Mutex<ArdeckManager>> = Lazy::new(|| Mutex::new(ArdeckManager::new()));
// static ACTION_MANAGER: Lazy<Mutex<ArdeckManager>> = Lazy::new(|| Mutex::new(ActionManager::new()));

// 現在接続中のポートの名前一覧を取得する
// invoke("plugin:ardeck|get_connecting_serials");
#[tauri::command]
fn get_connecting_serials() -> Vec<String> {
    let serials = ARDECK_MANAGER.lock().unwrap();

    let keys = serials.keys();
    keys.cloned().collect()
}

fn close<R: Runtime>(app: tauri::AppHandle<R>, port_name: &str) {
    let mut ardeck_manager = ARDECK_MANAGER.lock().unwrap();
    ardeck_manager.remove(port_name);

    app.emit_all("on-close-serial", port_name).unwrap();

    println!("[{}] closed.", port_name);
}

fn get_port(port_name: &str) -> io::Result<Arc<Mutex<Box<dyn SerialPort>>>> {
    let mut am = ARDECK_MANAGER.lock().unwrap();
    match am.get_mut(port_name) {
        Some(a) => Ok(a.port()),
        None => Err(io::Error::new(io::ErrorKind::NotFound, "error")),
    }
}

async fn port_read<R: Runtime>(app: tauri::AppHandle<R>, port_name: &str) {
    let port_name = port_name.to_string();
    tokio::spawn(async move {
        loop {

            if !ARDECK_MANAGER.lock().unwrap().get(&port_name).unwrap().is_continue() {
                // drop(am);
                close(app.app_handle(), &port_name);
                break;
            }

            let mut serial_buf: Vec<u8> = vec![0; 1];
            // serial_buf.fill(0);

            let port = get_port(&port_name).unwrap();
            let try_read = port.lock().unwrap().read(&mut serial_buf);
            match try_read {
                Ok(_) => {
                    drop(port);
                    let port_data = ARDECK_MANAGER.lock().unwrap().get(&port_name).unwrap().port_data();
                    port_data.lock().unwrap().on_data(serial_buf);
                }
                Err(kind) => {
                    println!(
                        "[{}] Connection error. Connection stoped.\nKind: {}",
                        &port_name, kind
                    );
                    drop(port);
                    close(app.app_handle(), &port_name);

                    break;
                }
            }
        }
    });
}

// invoke("plugin:ardeck|close_port");
#[tauri::command]
async fn close_port<R: Runtime>(app: tauri::AppHandle<R>, port_name: &str) -> Result<u32, u32> {
    match ARDECK_MANAGER.lock().unwrap().get_mut(port_name) {
        Some(a) => {
            a.close_requset();

            return Ok(200);
        }
        None => {
            return Err(501);
        }
    };
}

// invoke("plugin:ardeck|open_port");
#[tauri::command]
async fn open_port<R: Runtime>(
    // app: tauri::AppHandle
    app: tauri::AppHandle<R>,
    port_name: &str,
    baud_rate: u32,
) -> Result<u32, u32> {
    println!("[{}] plugin:ardeck|open_port", port_name);
    // 接続済みのポートならば何もしない
    if ARDECK_MANAGER.lock().unwrap().get(port_name).is_some() {
        println!("[{}] Already Opened.", port_name);
        return Err(501);
    }

    let ardeck = match Ardeck::open(port_name, baud_rate) {
        Ok(f) => f,
        Err(e) => {
            println!("Open Error !!!!!! {}", port_name);

            return Err(500);
        }
    };

    // 5秒間何も受け取れなければ通信を終了する
    ardeck
        .port()
        .lock()
        .unwrap()
        .set_timeout(Duration::from_millis(5000))
        .unwrap();

    // データを受信し、1回分のデータが完成した時の処理
    let app_for_data = app.app_handle();

    ardeck.port_data().lock().unwrap().on_complete(move |data| {
        println!("\n\n[] ardeck.portdata.on_complete\n\n");

        app_for_data
            .emit_all("on-message-serial", data.clone())
            .unwrap();
    });

    ardeck.port_data().lock().unwrap().on_change_action(move |data| {
        println!("\n\n[] ardeck.portdata.on_complete\n\n");

        plugin::tauri::put_action(data);
    });

    ARDECK_MANAGER
        .lock()
        .unwrap()
        .insert(port_name.to_string(), ardeck);

    app.emit_all("on-open-serial", port_name).unwrap();

    port_read(app.app_handle(), port_name).await;

    Ok(200)
}

fn serial_watch<R: Runtime>(tauri_app: tauri::AppHandle<R>) {
    println!("serial.watch");
    let refresh_fps = 1000 / 4;

    tokio::spawn(async move {
        let mut last_ports: Vec<SerialPortInfo> = Vec::new();

        loop {
            let ports = serialport::available_ports().unwrap();

            if last_ports.clone() != ports.clone() {
                println!("serial.watch");
                tauri_app.emit_all("on-ports", ports.clone()).unwrap();
            }

            last_ports = ports;

            park_timeout(Duration::from_millis(refresh_fps));
        }
    });
}

// ポートの一覧を取得する
#[tauri::command]
fn get_ports() -> Vec<serialport::SerialPortInfo> {
    println!("get_ports");
    let ports = serialport::available_ports().unwrap();
    println!("got.");
    ports
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("ardeck")
        .invoke_handler(tauri::generate_handler![
            open_port,
            close_port,
            get_connecting_serials,
            get_ports
        ])
        .setup(|app| {
            serial_watch(app.app_handle());
            // app.manage(Mutex::new(ArdeckManager::new()));
            Ok(())
        })
        .build()
}
