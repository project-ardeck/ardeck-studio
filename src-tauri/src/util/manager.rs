use std::collections::HashMap;

pub trait Manager<V> {
    fn new() -> Self;
    fn add(&mut self, key: String, value: V) -> Result<(), String>;
    fn get(&self, key: String) -> Option<&V>;
    fn get_all(&self) -> &HashMap<String, V>;
    fn get_all_mut(&mut self) -> &mut HashMap<String, V>;
    fn delete(&mut self, key: String) -> Result<(), String>;
}