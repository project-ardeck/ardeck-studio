use std::sync::Mutex;

use once_cell::sync::Lazy;
use tauri::{plugin::{Builder, TauriPlugin}, Runtime};

use super::{core::PluginCore, manager::PluginManager};

static PLGUIN_CORE: Lazy<Mutex<PluginCore>> = Lazy::new(|| {
    Mutex::new(PluginCore::new())
});

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("ardeck-plugin")
        .setup(|app| {
            Ok(())
        })
        .build()
}