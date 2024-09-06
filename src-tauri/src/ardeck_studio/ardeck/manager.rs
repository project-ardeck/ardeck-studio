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


use std::{collections::HashMap, hash::Hash, sync::{Arc, Mutex}};

use super::Ardeck;


pub trait ArdeckManager {

}

impl ArdeckManager for HashMap<&str, Ardeck> {

}

impl ArdeckManager for HashMap<&str, Arc<Mutex<Ardeck>>> {

}

// pub struct ArdeckManager {
//     ardecks: HashMap<String, Ardeck>,
// }

// impl ArdeckManager {
//     pub fn new() -> ArdeckManager {
//         ArdeckManager {
//             ardecks: HashMap::new(),
//         }
//     }

//     pub fn add(&mut self, plugin: Ardeck) {
//         // keyの部分はもうちょいどうにかする
//         self.ardecks.insert(plugin.lock().unwrap().port().name().unwrap(), plugin);
//     }

//     pub fn get(&self, id: &str) -> Option<&Ardeck> {
//         self.ardecks.get(id)
//     }

//     pub fn get_all(&self) -> &HashMap<String, Ardeck> {
//         &self.ardecks
//     }

//     pub fn get_all_mut(&mut self) -> &mut HashMap<String, Ardeck> {
//         &mut self.ardecks
//     }

//     pub fn remove(&mut self, id: &str) {
//         self.ardecks.remove(id);
//     }
// }