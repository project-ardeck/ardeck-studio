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


pub mod command;
pub mod data;
pub mod manager;

use command::ArdeckCommand;
use data::{
    ArdeckData,
    ActionData,
};

use serialport::{self, SerialPort};

use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread,
    time::Duration,
};


pub struct Ardeck {
    continue_flag: AtomicBool,

    port: Arc<Mutex<Box<dyn SerialPort>>>,
    port_data: ArdeckData,
}

/* State List
// - 0: Inited, Not Work.
// - 1: Port Connecting.
*/

pub enum OpenError {
    Unknown,
}

impl Ardeck {
    pub fn open(port_name: &String, baud_rate: u32) -> Result<Ardeck, OpenError> {
        println!("Open Port: {} - {}", port_name, baud_rate);
        let port = serialport::new(port_name, baud_rate).open();

        match port {
            Ok(mut port) => {
                println!("Port Opened.");
                Ok(Ardeck {
                    continue_flag: AtomicBool::new(true),
                    port: Arc::new(Mutex::new(port)),
                    port_data: ArdeckData::new(),
                }) // TODO: Arcを外す
            }
            Err(_) => Err(OpenError::Unknown),
        }
    }

    pub fn get_ports() -> Vec<serialport::SerialPortInfo> {
        let ports = serialport::available_ports().expect("Ports Not Found.");

        ports
    }
    
    pub fn get_state(&self) -> bool {
        self.continue_flag.load(Ordering::Relaxed)
    }
    
    pub fn state(&self) -> &AtomicBool {
        &self.continue_flag
    }
    
    pub fn port(&self) -> Arc<Mutex<Box<dyn SerialPort>>> {
        // &self.port
        // &self.port
        Arc::clone(&self.port)
    }


    
    pub fn port_data(&self) -> &ArdeckData {
        &self.port_data
    }
    
    pub fn continue_flag(&self) -> &AtomicBool {
        &self.continue_flag
    }
}

impl Drop for Ardeck {
    fn drop(&mut self) {
        // self.reset();
        // self.continue_flag.store(false, Ordering::Relaxed);
    }
}
