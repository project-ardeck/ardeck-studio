use std::sync::{atomic::AtomicBool, Mutex, OnceLock};
use std::fs;
use serde::{Deserialize, Serialize};

static WAS_CHANGED_SETTING: AtomicBool = AtomicBool::new(false);

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DeviceSettingOptions {
    pub serial_number: String,

    pub device_name: Option<String>,
    pub baud_rate: Option<u32>, // default: 19200
    pub description: Option<String>,
}

#[derive(Debug)]
pub enum GetDeviceSettingError {
    NotFound,
    // IoError(std::io::Error),
    SerdeError(serde_json::Error),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DeviceSettings;
impl DeviceSettings {
    // TODO: アプリで使うディレクトリを(存在しなければ)作成する関数
    pub fn init_dir() {}

    pub fn get_settings() -> Result<Vec<DeviceSettingOptions>, GetDeviceSettingError> {
        let settings_path = "settings/device_settings.json";
        let settings_str = match fs::read_to_string(settings_path) {
            Ok(s) => s,
            Err(_) => return Ok(Vec::new()),
        };
        match serde_json::from_str(&settings_str) {
            Ok(settings) => Ok(settings),
            Err(e) => Err(GetDeviceSettingError::SerdeError(e)),
        }
    }

    pub fn get_settings_device(serial_number: &str) -> Result<DeviceSettingOptions, GetDeviceSettingError> {
        let settings = Self::get_settings()?;
        for setting in settings {
            if setting.serial_number == serial_number {
                return Ok(setting);
            }
        }
        Err(GetDeviceSettingError::NotFound)
    }
}
