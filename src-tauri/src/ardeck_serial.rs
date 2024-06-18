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
    state: Arc<AtomicBool>,
    // Serial: Option<serialport::SerialPortBuilder>,
    // port_list: Option<Vec<serialport::SerialPortInfo>>,

    // Use only while connected
    // port: Option<Arc<Mutex<Box<dyn SerialPort>>>>,
    port: Arc<Mutex<Box<dyn SerialPort>>>,
    port_data: Arc<Mutex<ArdeckData>>,
    // listen_thread: Option<Arc<thread::JoinHandle<()>>>,
    // port_name: Option<String>,
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
                    state: Arc::new(AtomicBool::new(true)),
                    port: Arc::new(Mutex::new(port)),
                    port_data: Arc::new(Mutex::new(ArdeckData::new())),
                    // listen_thread: None,
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
        self.state.load(Ordering::Relaxed)
    }
    
    pub fn state(&self) -> Arc<AtomicBool> {
        self.state.clone()
    }
    
    pub fn port(&self) -> Arc<Mutex<Box<dyn SerialPort>>> {
        self.port.clone()
    }
    
    pub fn port_data(&self) -> Arc<Mutex<ArdeckData>> {
        self.port_data.clone()
    }
}

impl Drop for ArdeckSerial {
    fn drop(&mut self) {
        // self.reset();
        self.state.store(false, Ordering::Relaxed);
    }
}
