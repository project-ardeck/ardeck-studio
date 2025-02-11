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

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod ardeck_studio;
mod service;

use std::{path::PathBuf, sync::Mutex};

use service::dir::Directories;
use tauri::{
    CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
};
use tokio::fs::File;
use window_shadows::set_shadow;

#[tokio::main]
async fn main() {
    // TODO: ロガーの初期化時のエラーハンドリング

    // ログファイルの作成
    let log_file_path = format!("{}/{}.log", Directories::get_log_dir().unwrap().display(), chrono::Local::now().format("%Y-%m-%d-%H-%M-%S"));         
    let log_file = File::create(&log_file_path).await.unwrap();

    // ログの設定
    fern::Dispatch::new()
    .format(|out, message, record| {
        out.finish(format_args!(
            "[{}][{}][{}]: {}",
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
            record.level(),
            record.target(),
            message
        ));
    })
    .chain(std::io::stdout())
    .chain(fern::log_file(PathBuf::from(log_file_path)).unwrap())
    .level_for("tokio_tungstenite", log::LevelFilter::Off)
    .apply()
    .unwrap();

    // print!("\x1B[2J\x1B[1;1H"); // ! コンソールをクリア

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
        // .plugin(tauri_plugin_log::Builder::default().target(LogTarget::Folder(dir::Directories::get_log_dir().unwrap())).build()) // TODO: default().taget(Folder(/* ディレクトリ */))
        .plugin(ardeck_studio::ardeck::tauri::init())
        .plugin(ardeck_studio::plugin::tauri::init().await)
        .plugin(ardeck_studio::settings::tauri::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
