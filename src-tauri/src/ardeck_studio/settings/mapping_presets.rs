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

use crate::ardeck_studio::action::map::ActionMap;

use super::Settings;

#[derive(Debug, Serialize, Deserialize, Clone, FieldNamesAsArray)]
#[serde(rename_all = "camelCase")]
pub struct MappingPreset {
    pub preset_id: String,
    pub preset_name: Option<String>,

    pub mapping: Vec<ActionMap>,
}

pub type MappingPresetsJSON = Vec<MappingPreset>;

impl Settings for MappingPresetsJSON {
    fn config_file() -> &'static str {
        "mapping_presets.json"
    }
}