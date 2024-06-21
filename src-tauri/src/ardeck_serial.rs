mod ardeck_data;
use ardeck_data::ArdeckData;

use serialport::{self, SerialPort};

use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread,
    time::Duration,
};

#[derive(Clone)]
pub struct ArdeckSerial {
    continue_flag: Arc<AtomicBool>,

    port: Arc<Mutex<Box<dyn SerialPort>>>,
    port_data: Arc<Mutex<ArdeckData>>,
}

/* State List
// - 0: Inited, Not Work.
// - 1: Port Connecting.
*/

pub enum OpenError {
    Unknown,
}

impl ArdeckSerial {
    pub fn open(port_name: &String, baud_rate: u32) -> Result<ArdeckSerial, OpenError> {
        println!("Open Port: {} - {}", port_name, baud_rate);
        let port = serialport::new(port_name, baud_rate).open();

        match port {
            Ok(mut port) => {
                println!("Port Opened.");

                // port.set_timeout(Duration::from_millis(1000))
                //     .expect("Set Timeout Error.");
                Ok(ArdeckSerial {
                    continue_flag: Arc::new(AtomicBool::new(true)),
                    port: Arc::new(Mutex::new(port)),
                    port_data: Arc::new(Mutex::new(ArdeckData::new())),
                })
            }
            Err(_) => Err(OpenError::Unknown),
        }

        // ArdeckSerial {
        //     state: Arc::new(AtomicBool::new(true)),
        //     port: Arc::new(Mutex::new(port)),
        //     port_data: Arc::new(Mutex::new(ArdeckData::new())),
        // }
    }

    pub fn get_ports() -> Vec<serialport::SerialPortInfo> {
        let ports = serialport::available_ports().expect("Ports Not Found.");

        ports
    }
    
    pub fn get_state(&self) -> bool {
        self.continue_flag.load(Ordering::Relaxed)
    }
    
    pub fn state(&self) -> Arc<AtomicBool> {
        self.continue_flag.clone()
    }
    
    pub fn port(&self) -> Arc<Mutex<Box<dyn SerialPort>>> {
        self.port.clone()
    }
    
    pub fn port_data(&self) -> Arc<Mutex<ArdeckData>> {
        self.port_data.clone()
    }
    
    pub fn continue_flag(&self) -> &AtomicBool {
        &self.continue_flag
    }
}

impl Drop for ArdeckSerial {
    fn drop(&mut self) {
        // self.reset();
        // self.continue_flag.store(false, Ordering::Relaxed);
    }
}
