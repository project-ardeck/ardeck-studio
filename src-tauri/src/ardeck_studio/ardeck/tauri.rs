use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
};

#[tauri::command]
async fn command_name<R: Runtime>(app: tauri::AppHandle<R>, window: tauri::Window<R>) -> Result<(), String> {
  Ok(())
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("ardeck")
        .build()
}
