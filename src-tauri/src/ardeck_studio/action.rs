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

use action_target::ActionTarget;
use serde::{Deserialize, Serialize};

use super::{
    settings::{definitions::mapping_presets::MappingPresetsJSON, SettingsStore},
    switch_info::SwitchInfo,
};

pub mod action_map;
pub mod action_target;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Action {
    pub switch: SwitchInfo,
    pub target: ActionTarget,
}

impl Action {
    /// スイッチの情報から、そのスイッチが割り当てられているアクションを見つけ、ActionのVecを返す
    pub async fn from_switch_info(switch: SwitchInfo) -> Vec<Self> {
        log::trace!(
            "# Action::from_switch_info\n\tswitch_state: {}",
            switch.switch_state
        );
        let target = Self::search_action_target(switch.clone()).await;

        let mut actions: Vec<Action> = Vec::new();

        for t in target.iter() {
            actions.push(Action {
                switch: switch.clone(),
                target: t.clone(),
            });
        }

        actions
    }

    /// スイッチの情報から、そのスイッチが割り当てられているアクションを見つけ、Vec<ActionTarget>を返す
    async fn search_action_target(switch_info: SwitchInfo) -> Vec<ActionTarget> {
        let mapping_presets = match MappingPresetsJSON::new().load().await {
            Some(presets) => presets,
            None => return Vec::new(),  // TODO: Error
        };
        
        let mut target: Vec<ActionTarget> = Vec::new();

        // switch_typeとswitch_idと一致するマッピングを探し、アクションのターゲットを返す
        for preset in mapping_presets.iter() {
            for map in preset.mapping.iter() {
                if map.switch_type == switch_info.switch_type
                    && map.switch_id == switch_info.switch_id
                {
                    target.push(ActionTarget {
                        plugin_id: map.plugin_id.clone(),
                        action_id: map.action_id.clone(),
                    });
                }
            }
        }

        target
    }
}
