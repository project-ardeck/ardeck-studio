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

use std::collections::HashMap;

use super::{SwitchInfo, SwitchId};

// #[derive(Clone)]
pub struct ActionCompare {
    actions: HashMap<SwitchId, SwitchInfo>,
    prev_actions: HashMap<SwitchId, SwitchInfo>,
    on_change_action: Vec<Box<dyn Fn(SwitchInfo) + Send + 'static>>,
}

impl ActionCompare {
    pub fn new() -> Self {
        Self {
            actions: HashMap::new(),
            prev_actions: HashMap::new(),
            on_change_action: Vec::new(),
        }
    }

    pub fn put_action(&mut self, action: SwitchInfo) {
        // let switch_id = action.get_switch_id();
        // // if
        // if let Some(current_action) = self.actions.get(&switch_id) {
        //     self.prev_actions.insert(switch_id, current_action.clone());
        // }

        // self.actions.insert(switch_id, action);

        self.compare(action);
    }

    fn compare(&mut self, new_switch_info: SwitchInfo) {
        if let Some(prev_action) = self.prev_actions.get(&new_switch_info.get_switch_id()) {
            if new_switch_info.get_switch_state() != prev_action.get_switch_state() {
                log::debug!("change state: {}", new_switch_info.get_switch_id());
                self.on_change_action_emit_all(new_switch_info.clone());
                self.prev_actions.insert(new_switch_info.get_switch_id(), new_switch_info);
            }
        } else {
            log::debug!("new switch: {}", new_switch_info.get_switch_id());
            self.on_change_action_emit_all(new_switch_info.clone());
            self.prev_actions.insert(new_switch_info.get_switch_id(), new_switch_info);
        }

    }

    pub fn on_change_action<F: Fn(SwitchInfo) + Send + 'static>(&mut self, callback: F) {
        self.on_change_action.push(Box::new(callback));
    }

    fn on_change_action_emit_all(&self, action: SwitchInfo) {
        for c in self.on_change_action.iter() {
            c(action.clone())
        }
    }
}
