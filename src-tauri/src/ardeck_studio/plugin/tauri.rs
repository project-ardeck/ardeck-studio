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
