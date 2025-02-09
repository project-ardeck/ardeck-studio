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

pub mod core;
pub mod manager;
pub mod tauri;

use serialport::{self, SerialPort};

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc
};

use tokio::sync::Mutex;

use crate::ardeck_studio::switch_info::ActionDataParser;

#[derive(Clone)]
pub struct Ardeck {
    continue_flag: Arc<Mutex<AtomicBool>>,

    port: Arc<Mutex<Box<dyn SerialPort>>>,
    port_data: Arc<Mutex<ActionDataParser>>,
}

/* State List
// - 0: Inited, Not Work.
// - 1: Port Connecting.
*/

pub enum OpenError {
    Unknown,
}

impl Ardeck {
    pub fn open(port_name: &str, baud_rate: u32) -> Result<Ardeck, OpenError> {
        println!("Open Port: {} - {}", port_name, baud_rate);
        let port = serialport::new(port_name, baud_rate).open();

        match port {
            Ok(port) => {
                println!("Port Opened.");
                Ok(Ardeck {
                    continue_flag: Arc::new(Mutex::new(AtomicBool::new(true))),
                    port: Arc::new(Mutex::new(port)),
                    port_data: Arc::new(Mutex::new(ActionDataParser::new())),
                })
            }
            Err(_) => Err(OpenError::Unknown),
        }
    }

    pub fn get_ports() -> Vec<serialport::SerialPortInfo> {
        let ports = serialport::available_ports().expect("Ports Not Found.");

        ports
    }

    pub async fn is_continue(&self) -> bool {
        self.continue_flag.lock().await.load(Ordering::Relaxed)
    }

    pub fn continue_flag(&self) -> Arc<Mutex<AtomicBool>> {
        Arc::clone(&self.continue_flag)
    }

    pub fn port(&self) -> Arc<Mutex<Box<dyn SerialPort>>> {
        Arc::clone(&self.port)
    }

    pub fn port_data(&self) -> Arc<Mutex<ActionDataParser>> {
        Arc::clone(&self.port_data)
    }

    pub async fn close_request(&self) {
        self.continue_flag
            .lock()
            .await
            .store(false, Ordering::SeqCst)
    }
}

impl Drop for Ardeck {
    fn drop(&mut self) {
        // self.reset();
        // self.continue_flag.store(false, Ordering::Relaxed);
    }
}
