use std::{
    collections::HashMap,
    hash::Hash,
    process::Child,
    sync::{Arc, Mutex},
};

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

    pub fn add_plugin(&mut self, plugin: Plugin) {
        self.plugins.insert(plugin.manifest.clone().id, plugin);
    }

    pub fn get_plugin(&self, id: &str) -> Option<&Plugin> {
        self.plugins.get(id)
    }

    pub fn get_plugins(&self) -> &HashMap<String, Plugin> {
        &self.plugins
    }

    pub fn get_plugins_mut(&mut self) -> &mut HashMap<String, Plugin> {
        &mut self.plugins
    }

    pub fn remove_plugin(&mut self, id: &str) {
        self.plugins.remove(id);
    }
}
