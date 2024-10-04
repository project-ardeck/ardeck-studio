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

use super::{manager::ArdeckManager, Ardeck};

pub struct ArdeckCore {
    ardeck_manager: ArdeckManager,
}

impl ArdeckCore {
    pub fn new() -> Self {
        Self {
            ardeck_manager: ArdeckManager::new(),
        }
    }

    pub fn manager(&mut self) -> &mut ArdeckManager {
        &mut self.ardeck_manager
    }

    pub async fn start_read(ardeck: Ardeck) {
        
    }
}