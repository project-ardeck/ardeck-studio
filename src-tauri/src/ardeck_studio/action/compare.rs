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

use std::collections::HashMap;

use super::{Action, SwitchId};

// #[derive(Clone)]
pub struct ActionCompare {
    actions: HashMap<SwitchId, Action>,
    prev_actions: HashMap<SwitchId, Action>,
    on_change_action: Vec<Box<dyn Fn(Action) + Send + 'static>>,
}

impl ActionCompare {
    pub fn new() -> Self {
        Self {
            actions: HashMap::new(),
            prev_actions: HashMap::new(),
            on_change_action: Vec::new(),
        }
    }

    pub fn put_action(&mut self, action: Action) {
        let switch_id = action.get_switch_id();
        // if
        if let Some(current_action) = self.actions.get(&switch_id) {
            self.prev_actions.insert(switch_id, current_action.clone());
        }

        self.actions.insert(switch_id, action);

        self.compare(switch_id);
    }

    fn compare(&self, switch_id: SwitchId) {
        if let Some(prev_action) = self.prev_actions.get(&switch_id) {
            let now_action = self.actions.get(&switch_id).unwrap();
            if now_action.get_switch_state() != prev_action.get_switch_state() {
                self.on_change_action_emit_all(now_action.clone());
            }
        } else {
            let now_action = self.actions.get(&switch_id).unwrap();
            self.on_change_action_emit_all(now_action.clone());
        }
    }

    pub fn on_change_action<F: Fn(Action) + Send + 'static>(&mut self, callback: F) {
        self.on_change_action.push(Box::new(callback));
    }

    fn on_change_action_emit_all(&self, action: Action) {
        for c in self.on_change_action.iter() {
            c(action.clone())
        }
    }
}
