use std::{
    collections::HashMap,
    hash::Hash,
    process::Child,
    sync::{Arc, Mutex},
};

use crate::util::manager::Manager;

use super::{Plugin, PluginManifest};

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
