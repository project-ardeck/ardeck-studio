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

async fn init_logger() {
    // TODO: ロガーの初期化時のエラーハンドリング

    let log_file_path = format!(
        "{}/{}.log",
        Directories::get_log_dir().unwrap().display(),
        chrono::Local::now().format("%Y-%m-%d-%H-%M-%S")
    );
    File::create(&log_file_path).await.unwrap();

    let base_config = fern::Dispatch::new();

    // TODO: debugやtraceは、コンフィグ次第で出力できるようにする
    let stdout_config = fern::Dispatch::new()
        .level(log::LevelFilter::Info)
        .chain(std::io::stdout())
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}][{}][{}][{:?}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                record.level(),
                record.target(),
                record.module_path(),
                message
            ));
        });
    let file_config = fern::Dispatch::new()
        .level(log::LevelFilter::Info)
        .chain(fern::log_file(PathBuf::from(log_file_path)).unwrap())
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}][{}][{}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                record.level(),
                record.target(),
                message
            ));
        });

    base_config
        .chain(stdout_config)
        .chain(file_config)
        .apply()
        .unwrap();
}

#[tokio::main]
async fn main() {
    init_logger().await;
    log::info!("Ardeck Studio v{}", env!("CARGO_PKG_VERSION"));

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
