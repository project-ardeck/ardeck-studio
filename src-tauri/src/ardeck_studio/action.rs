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

use action_target::ActionTarget;
use serde::{Deserialize, Serialize};

use super::{settings::{definitions::mapping_presets, SettingsStore}, switch_info::SwitchInfo};

pub mod action_target;
pub mod action_map;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Action {
    switch: SwitchInfo,
    target: ActionTarget,
}

impl Action {
    pub fn from_switch_info(switch: SwitchInfo) -> Self {
        Action {
            switch,
            target: ActionTarget {
                action_id: String::from("foo"),
                plugin_id: String::from("bar")
            }
        }
    }

    fn search_action_target(&self, switch_info: SwitchInfo) -> Option<ActionTarget> {
        let mapping_presets = mapping_presets::MappingPresetsJSON::new().load();
        None
    }
}
