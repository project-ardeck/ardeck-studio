use serialport::{self, SerialPort};

use serde::{Deserialize, Serialize};

pub struct ArdeckSerial {
    State: u8,
    // Serial: Option<serialport::SerialPortBuilder>,
    port_list: Option<Vec<serialport::SerialPortInfo>>,

    // Use only while connected
    port: Option<Arc<Mutex<Box<dyn SerialPort>>>>,
    port_name: Option<String>,
}

/* State List
// - 0: Inited, Not Work.
// - 1: Port Connecting.
*/

impl ArdeckSerial {
    pub fn new() -> ArdeckSerial {
        ArdeckSerial {
            State: 0,
            // Serial: None,
            port_list: None,
            port: None,
            port_name: None,
        }
    }

    pub fn get_state(&self) -> u8 {
        self.State
    }

    pub fn get_ports() -> Vec<serialport::SerialPortInfo> {
        let ports = serialport::available_ports().expect("Ports Not Found.");

        ports
    }

    pub fn refresh_ports(&mut self) -> Vec<serialport::SerialPortInfo> {
        let ports = serialport::available_ports().expect("Ports Not Found.");

        self.port_list = Some(ports.clone());

        ports
    }

    // fn open(&mut self, ports_index: usize) -> serialport::SerialPortBuilder {
    //     let select_port = &self.Ports.clone().unwrap()[ports_index];
    //     serialport::new(select_port, 9600)
    //         .open().unwrap();
    // }

    pub fn open(
        &mut self,
        port_name: String,
    ) -> Result<Arc<Mutex<Box<dyn SerialPort>>>, serialport::Error> {
        let port = serialport::new(port_name, 9600).open();

        match port {
            Ok(p) => {
                println!("Ardeck Serial: opened port {:?}", &p.name());

                self.port = Some(Arc::new(Mutex::new(p)));
                self.state = 1;
            }
            Err(p) => {
                println!("Oops! port is not found or other reason Error");
                return Err(p);
            }
        }

        Ok(self.port.clone().unwrap())
    }

    pub fn reset(&mut self) {
        if self.state == 1 {
            self.port
                .as_mut()
                .unwrap()
                .lock()
                .unwrap()
                .clear_break()
                .unwrap();
        }
        self.state = 0;
        self.port_list = None;
    }
}

impl Drop for ArdeckSerial {
    fn drop(&mut self) {
        self.reset();
    }
}

/// -------------- Ardeck  Seroial2 Draft --------------------
/// -------------- Ardeck  Seroial2 Draft --------------------
/// -------------- Ardeck  Seroial2 Draft --------------------
/// -------------- Ardeck  Seroial2 Draft --------------------
/// -------------- Ardeck  Seroial2 Draft --------------------
/// -------------- Ardeck  Seroial2 Draft --------------------
/// -------------- Ardeck  Seroial2 Draft --------------------
/// -------------- Ardeck  Seroial2 Draft --------------------
/// -------------- Ardeck  Seroial2 Draft --------------------
/// -------------- Ardeck  Seroial2 Draft --------------------
/// -------------- Ardeck  Seroial2 Draft --------------------
/// -------------- Ardeck  Seroial2 Draft --------------------
/// -------------- Ardeck  Seroial2 Draft --------------------
/// -------------- Ardeck  Seroial2 Draft --------------------
/// -------------- Ardeck  Seroial2 Draft --------------------
/// -------------- Ardeck  Seroial2 Draft --------------------
pub struct ArdeckSerial2 {
    state: u8,
    // Serial: Option<serialport::SerialPortBuilder>,

    // Use only while connected
    port: Box<dyn SerialPort>,
}

/* State List
// - 0: Inited, Not Work.
// - 1: Port Connecting.
*/

impl ArdeckSerial2 {
    pub fn get_ports() -> Vec<serialport::SerialPortInfo> {
        let ports = serialport::available_ports().expect("Ports Not Found.");

        ports
    }

    pub fn new(port_name: String, baud_rate: u32) -> Result<ArdeckSerial2, serialport::Error> {
        let port = serialport::new(port_name, baud_rate).open();

        match port {
            Ok(p) => {
                println!("Ardeck Serial: opened port {:?}", &p.name());

                // self.port = Some(Arc::new(Mutex::new(p)));
                // self.state = 1;
                Ok(ArdeckSerial2 {
                    state: 1,
                    // Serial: None,
                    port: p,
                })
            }
            Err(p) => {
                println!("Oops! port is not found or other reason Error");
                return Err(p);
            }
        }
    }

    pub fn get_state(&self) -> u8 {
        self.state
    }

    // fn open(&mut self, ports_index: usize) -> serialport::SerialPortBuilder {
    //     let select_port = &self.Ports.clone().unwrap()[ports_index];
    //     serialport::new(select_port, 9600)
    //         .open().unwrap();
    // }

    pub fn reset(&mut self) {
        
    }
}

impl Drop for ArdeckSerial2 {
    fn drop(&mut self) {
        self.reset();
    }
}
