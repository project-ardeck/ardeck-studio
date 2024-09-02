use std::collections::HashMap;

use super::Ardeck;

#[derive(Clone)]
pub struct ArdeckManager {
    ardecks: HashMap<String, Ardeck>,
}

impl ArdeckManager {
    pub fn new() -> ArdeckManager {
        ArdeckManager {
            ardecks: HashMap::new(),
        }
    }

    pub fn add(&mut self, plugin: Ardeck) {
        // keyの部分はもうちょいどうにかする
        self.ardecks.insert(plugin.port().lock().unwrap().name().unwrap(), plugin);
    }

    pub fn get(&self, id: &str) -> Option<&Ardeck> {
        self.ardecks.get(id)
    }

    pub fn get_all(&self) -> &HashMap<String, Ardeck> {
        &self.ardecks
    }

    pub fn get_all_mut(&mut self) -> &mut HashMap<String, Ardeck> {
        &mut self.ardecks
    }

    pub fn remove(&mut self, id: &str) {
        self.ardecks.remove(id);
    }
}