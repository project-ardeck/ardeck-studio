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

#![allow(deprecated)]

mod ardeck_studio;
mod service;

use std::time::SystemTime;

use fern::colors::ColoredLevelConfig;
use service::dir::Directories;
use tauri::{
    image::Image, menu::{MenuBuilder, MenuItemBuilder}, tray::{TrayIcon, TrayIconBuilder, TrayIconEvent}, Manager
};
use tokio::fs::{self, File};
use window_shadows::set_shadow;

async fn init_logger() {
    if let Err(e) = init_logger_internal().await {
        eprintln!("Failed to initialize logger: {}", e);
        std::process::exit(1);
    };
}

async fn init_logger_internal() -> Result<(), Box<dyn std::error::Error>> {
    const MAX_FILE: usize = 10;

    let log_dir = Directories::get_log_dir()?;
    std::fs::create_dir_all(&log_dir)?;
    let log_file_name = format!("{}.log", chrono::Local::now().format("%Y-%m-%d-%H-%M-%S"));
    let log_file_path = log_dir.join(&log_file_name);
    File::create(&log_file_path).await?;
    delete_old_logs(MAX_FILE).await?;

    let colors = ColoredLevelConfig::new()
        .error(fern::colors::Color::Red)
        .warn(fern::colors::Color::Yellow)
        .info(fern::colors::Color::Blue)
        .debug(fern::colors::Color::White)
        .trace(fern::colors::Color::BrightBlack);

    let base_config = fern::Dispatch::new();

    // TODO: debugやtraceは、コンフィグ次第で出力できるようにする
    let stdout_config = fern::Dispatch::new()
        .level(log::LevelFilter::Info)
        .chain(std::io::stdout())
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{}][{}][{}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                colors.color(record.level()),
                record.target(),
                message
            ));
        });
    let file_config = fern::Dispatch::new()
        .level(log::LevelFilter::Info)
        .chain(fern::log_file(log_file_path)?)
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
        .apply()?;

    Ok(())
}

async fn delete_old_logs(max_file: usize) -> Result<(), Box<dyn std::error::Error>> {
    let log_dir = Directories::get_log_dir()?;

    let mut files = std::fs::read_dir(log_dir)?
        .filter_map(|f| {
            f.ok().filter(|entry| {
                entry
                    .path()
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .map_or(false, |ext| ext == "log")
            })
        })
        .collect::<Vec<_>>();

    // タイムスタンプでソート（古い順）
    files.sort_by_key(|f| {
        f.metadata()
            .and_then(|m| m.created())
            .unwrap_or_else(|_| SystemTime::now())
    });
    files.reverse();

    for (i, d) in files.iter().enumerate() {
        if i >= max_file {
            fs::remove_file(d.path()).await?;
        }
    }

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    init_logger().await;

    // print!("\x1B[2J\x1B[1;1H"); // ! コンソールをクリア

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            window.show().unwrap();

            let hide = MenuItemBuilder::with_id("hide", "Hide").build(app)?;
            let quit = MenuItemBuilder::with_id("quit", "Quit").build(app)?;
            let menu = MenuBuilder::new(app).items(&[&hide, &quit]).build()?;

            let _tray = TrayIconBuilder::new()
                .menu(&menu)
                .show_menu_on_left_click(false)
                .icon(Image::from_path("icons/32x32.png").unwrap())
                .on_menu_event(|app, event| match event.id().as_ref() {
                    "hide" => {
                        if let Some(webview_window) = app.get_webview_window("main") {
                            let _ = webview_window.hide();
                        }
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| match event {
                    TrayIconEvent::DoubleClick { .. } => {
                        let app = tray.app_handle();
                        if let Some(webview_window) = app.get_webview_window("main") {
                            let _ = webview_window.show();
                            let _ = webview_window.set_focus();
                        }
                    },
                    _ => {}
                })
                .build(app)?;

            // log::info!(
            //     "{} {}",
            //     app.config().tauri.bundle.identifier,
            //     app.package_info().version.to_string()
            // );

            Ok(())
        })
        .plugin(ardeck_studio::ardeck::tauri::init())
        .plugin(ardeck_studio::plugin::tauri::init())
        .plugin(ardeck_studio::settings::tauri::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
