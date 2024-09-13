use std::{collections::HashMap, sync::Mutex, time::Duration};

use tauri::{
    generate_handler,
    plugin::{Builder, Plugin, TauriPlugin},
    AppHandle, Invoke, Manager, Runtime, State as TauriState,
};

use super::{manager::ArdeckManager, Ardeck};

fn close() {}

fn port_read<R: Runtime>(app: tauri::AppHandle<R>, port_name: &str, ardeck: ArdeckManager) {
    tokio::spawn(async move {
        loop {
            if ardeck.get_continue_flag() == false {
                // ardeck.close_requset();


            }
        }
    });
}

#[tauri::command]
async fn close_port<R: Runtime>(
    app: tauri::AppHandle<R>,
    ardeck_manager: TauriState<'_, Mutex<HashMap<String, Ardeck>>>,
    port_name: &str,
) -> Result<u32, u32> {
    match ardeck_manager.lock().unwrap().get_mut(port_name) {
        Some(a) => {
            a.close_requset();

            return Ok(200);
        }
        None => {
            return Err(501);
        }
    };
}



#[tauri::command]
fn open_port<R: Runtime>(// app: tauri::AppHandle
    app: tauri::AppHandle<R>,
    ardeck_manager: TauriState<'_, Mutex<HashMap<String, Ardeck>>>,
    port_name: &str,
    baud_rate: u32,
)  -> Result<u32, u32> {
    // 接続済みのポートならば何もしない
    if ardeck_manager.lock().unwrap().get(port_name).is_some() {
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
        app_for_data
            .clone()
            .emit_all("on-message-serial", data)
            .unwrap();

        // TODO: send to plugin manager
    });

    port_read(app.app_handle(), port_name, ardeck_manager);

    Ok(200)
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("awesome")
        .invoke_handler(tauri::generate_handler![open_port, close_port])
        .setup(|app| {


            app.manage(Mutex::new(ArdeckManager::new()));
            Ok(())
        })
        .build()
}
