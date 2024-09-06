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


use std::{
    collections::HashMap,
    hash::Hash,
    process::Child,
    sync::{Arc, Mutex},
};

use crate::util::manager::Manager;

use super::{Plugin, PluginManifest};


// pub trait PluginManager

#[derive(Clone, Debug)]
pub struct PluginManager {
    plugins: HashMap<String, Plugin>,
}

impl PluginManager {
    pub fn new() -> PluginManager {
        PluginManager {
            plugins: HashMap::new(),
        }
    }

    pub fn add(&mut self, plugin: Plugin) {
        self.plugins.insert(plugin.manifest.clone().id, plugin);
    }

    pub fn get(&self, id: &str) -> Option<&Plugin> {
        self.plugins.get(id)
    }

    pub fn get_all(&self) -> &HashMap<String, Plugin> {
        &self.plugins
    }

    pub fn get_all_mut(&mut self) -> &mut HashMap<String, Plugin> {
        &mut self.plugins
    }

    pub fn remove(&mut self, id: &str) {
        self.plugins.remove(id);
    }
}
