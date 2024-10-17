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

use std::sync::Mutex;

use once_cell::sync::Lazy;
use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, RunEvent, Runtime,
};

use crate::ardeck_studio::action::Action;

use super::{core::PluginCore, manager::PluginManager};

static PLGUIN_CORE: Lazy<Mutex<PluginCore>> = Lazy::new(|| Mutex::new(PluginCore::new()));

pub async fn init<R: Runtime>() -> TauriPlugin<R> {
    println!("[init] plugin init");
    let mut core = PLGUIN_CORE.lock().unwrap();
    let serve = core.start().await;

    println!("[init] serve started.");

    Builder::new("ardeck-plugin")
        .setup(|app| Ok(()))
        .on_event(|app, event| match event {
            RunEvent::Ready => {
                let mut core = PLGUIN_CORE.lock().unwrap();

                core.execute_plugin_all();
            }
            _ => {}
        })
        .build()
}

pub fn put_action(data: Action) {
    println!("Got push_action in plugin.tauri: {:?}", data);
    // TODO: 1つ前のデータ(同じスイッチのアクションデータ)と値が変わっていればCoreのput_actionに投げる
}
