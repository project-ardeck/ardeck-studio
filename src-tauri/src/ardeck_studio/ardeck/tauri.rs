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
use serialport::{SerialPort, SerialPortInfo};
use std::{io, sync::Arc, time::Duration};
use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};
use tokio::sync::Mutex;

use crate::ardeck_studio::plugin;

use super::{manager::ArdeckManager, Ardeck};

static ARDECK_MANAGER: Lazy<Mutex<ArdeckManager>> = Lazy::new(|| Mutex::new(ArdeckManager::new()));
// static ACTION_MANAGER: Lazy<Mutex<ArdeckManager>> = Lazy::new(|| Mutex::new(ActionManager::new()));

// 現在接続中のポートの名前一覧を取得する
// invoke("plugin:ardeck|get_connecting_serials");
#[tauri::command]
async fn get_connecting_serials() -> Vec<String> {
    let serials = ARDECK_MANAGER.lock().await;

    let keys = serials.keys();
    keys.cloned().collect()
}

async fn close<R: Runtime>(app: tauri::AppHandle<R>, port_name: &str) {
    let mut ardeck_manager = ARDECK_MANAGER.lock().await;
    ardeck_manager.remove(port_name);

    // TODO: DELETE
    app.emit_all("on-close-serial", port_name).unwrap();

    log::info!("closed: {}", port_name);
}

async fn get_port(port_name: &str) -> io::Result<Arc<Mutex<Box<dyn SerialPort>>>> {
    let mut am = ARDECK_MANAGER.lock().await;
    match am.get_mut(port_name) {
        Some(a) => Ok(a.port().clone()),
        None => Err(io::Error::new(io::ErrorKind::NotFound, "error")),
    }
}

async fn port_read<R: Runtime>(app: tauri::AppHandle<R>, port_name: &str) {
    let port_name = port_name.to_string();
    tokio::spawn(async move {
        let port = get_port(port_name.as_str()).await.unwrap();
        loop {
            // 継続フラグがfalseならば切断する
            if !ARDECK_MANAGER
                .lock()
                .await
                .get(&port_name)
                .unwrap()
                .is_continue()
                .await
            {
                // drop(am);
                close(app.app_handle(), &port_name).await;
                break;
            }

            let mut serial_buf: Vec<u8> = vec![0; 1];
            let port = port.clone().lock().await.read(&mut serial_buf);
            // let try_read = port;
            match port {
                Ok(_) => {
                    let port_data = ARDECK_MANAGER
                        .lock()
                        .await
                        .get(&port_name)
                        .unwrap()
                        .port_data();
                    port_data.lock().await.put_data(serial_buf);
                }
                Err(kind) => {
                    log::error!("Connection error. Connection stoped.\nKind: {}", kind);
                    close(app.app_handle(), &port_name).await;

                    break;
                }
            }
        }
    });
}

// invoke("plugin:ardeck|close_port");
#[tauri::command]
async fn close_port<R: Runtime>(_app: tauri::AppHandle<R>, port_name: &str) -> Result<u32, u32> {
    // 要求されたデバイスの処理継続フラグを折る
    match ARDECK_MANAGER.lock().await.get_mut(port_name) {
        Some(a) => {
            a.close_request().await;

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
    // print!("\x1B[2J\x1B[1;1H"); // ! コンソールをクリア
    log::info!("Ardeck Connect Request: {}", port_name);
    // 接続済みのポートならば何もしない
    if ARDECK_MANAGER.lock().await.get(port_name).is_some() {
        log::warn!("[{}] Already Opened.", port_name);
        return Err(501);
    }

    // デバイスへ接続する
    let ardeck = match Ardeck::open(port_name, baud_rate) {
        Ok(f) => f,
        Err(_e) => {
            log::error!("Open Error: {}", port_name);

            return Err(500);
        }
    };

    // 5秒間何も受け取れなければ通信を終了する
    ardeck
        .port()
        .lock()
        .await
        .set_timeout(Duration::from_millis(5000))
        .unwrap();

    let app_for_data = app.app_handle();

    // データを受信し、1回分のデータが完成した時の処理
    ardeck
        .port_data()
        .lock()
        .await
        .on_complete_action(move |data| {
            log::trace!("# Ardeck::on_complete_action\n\tdata: {:#?}", data);

            app_for_data
                .emit_all("on-message-serial", data.clone())
                .unwrap();
        });

    // TODO: async crosure
    // 1回前のデータから値が変わったときの処理
    ardeck
        .port_data()
        .lock()
        .await
        .on_change_action(move |data| {
            log::trace!(
                "# Ardeck::on_change_action\n\tswitch_id: {}\n\tswitch_state: {}",
                data.switch_id,
                data.switch_state
            );

            // let (tx, mut rx) = std::sync::mpsc::channel::<bool>();
            // let atomic_bool = Arc::new(Mutex::new(AtomicBool::new(false)));

            // let atomic_bool_spawn = atomic_bool.clone();
            tokio::spawn(async move {
                // TODO

                // println!(
                //     "# Ardeck::on_change_action/tokio::spawn\n\tswitch_id: {}\n\tswitch_state: {}",
                //     data.switch_id, data.switch_state
                // );

                plugin::tauri::send_action_to_plugins(data.clone()).await;

                // println!(
                //     "----- send_action_to_plugins end -----\tswitch_id: {}\n",
                //     data.switch_id
                // );
            });
        });

    // マネージャーにデバイスを追加
    ARDECK_MANAGER
        .lock()
        .await
        .insert(port_name.to_string(), ardeck);

    app.emit_all("on-open-serial", port_name).unwrap();

    // 受信データの読み取り開始
    port_read(app.app_handle(), port_name).await;

    Ok(200)
}

fn serial_watch<R: Runtime>(tauri_app: tauri::AppHandle<R>) {
    let refresh_fps = 1000 / 4;
    log::info!("Serial port watching: {}ms", refresh_fps);

    tokio::spawn(async move {
        let mut last_ports: Vec<SerialPortInfo> = Vec::new();

        loop {
            let ports = serialport::available_ports().unwrap();

            if last_ports.clone() != ports.clone() {
                log::info!("Ports list changed: {:?}", ports);
                tauri_app.emit_all("on-ports", ports.clone()).unwrap();
            }

            last_ports = ports;

            tokio::time::sleep(Duration::from_millis(refresh_fps)).await;
        }
    });
}

// ポートの一覧を取得する
#[tauri::command]
fn get_ports() -> Vec<serialport::SerialPortInfo> {
    let ports = serialport::available_ports().unwrap();
    ports
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    log::info!("Initializing Ardeck Tauri Plugin");

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
