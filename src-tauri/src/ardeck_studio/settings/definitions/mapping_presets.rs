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

use serde::{Deserialize, Serialize};
use struct_field_names_as_array::FieldNamesAsArray;

use crate::{
    ardeck_studio::{action::action_map::ActionMap, settings::{SettingFile, SettingsStore}},
    service::dir::Directories,
};

#[derive(Debug, Serialize, Deserialize, Clone, FieldNamesAsArray)]
#[serde(rename_all = "camelCase")]
pub struct MappingPreset {
    // uuid
    pub uuid: String,
    // 表示名
    pub preset_name: String,

    // マッピングリスト
    pub mapping: Vec<ActionMap>,
}

// setting! {
pub type MappingPresetsJSON = Vec<MappingPreset>;
// }

impl SettingFile for MappingPresetsJSON {
    fn name(&self) -> &'static str {
        "mapping_presets"
    }

    fn dir(&self) -> std::path::PathBuf {
        // TODO: Log
        Directories::get_settings_dir().unwrap()
    }
}

impl SettingsStore for MappingPresetsJSON {}