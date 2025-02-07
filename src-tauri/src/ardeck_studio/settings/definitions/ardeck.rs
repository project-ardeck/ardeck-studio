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

use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use struct_field_names_as_array::FieldNamesAsArray;

use crate::{
    ardeck_studio::settings::{SettingFile, SettingsStore},
    service::dir::Directories,
};

#[derive(Debug, Serialize, Deserialize, Clone, FieldNamesAsArray)]
#[serde(rename_all = "camelCase")]
// TODO: 名称の変更：ConfigField
pub struct ArdeckProfileConfigItem {
    // シリアルポート名
    pub serial_number: String,

    // シリアルデバイスの表示名
    pub device_name: Option<String>,
    // このデバイスで今後デフォルトで利用するレート
    pub baud_rate: Option<u32>,
    // デバイスの説明
    pub description: Option<String>,

    // このデバイスでデフォルトで割り当てるマッピングプリセットのID
    pub mapping_preset: Option<String>, // mapping preset id
}

// setting! {
pub type ArdeckProfileConfigJSON = Vec<ArdeckProfileConfigItem>;
// }

impl SettingFile for ArdeckProfileConfigJSON {
    fn name(&self) -> &'static str {
        "ardeck_profile"
    }

    fn dir(&self) -> PathBuf {
        // TODO: Log
        Directories::get_settings_dir().unwrap()
    }
}

impl SettingsStore for ArdeckProfileConfigJSON {}
