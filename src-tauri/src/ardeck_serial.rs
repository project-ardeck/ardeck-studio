use serialport::{self, SerialPort};

use std::option::Option;
use serde::{Deserialize, Serialize};

pub struct ArdeckSerial {
    State: u8,
    // Serial: Option<serialport::SerialPortBuilder>,
    PortList: Option<Vec<serialport::SerialPortInfo>>,
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
            PortList: None,
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

        self.PortList = Some(ports.clone());

        ports
    }

    // fn open(&mut self, ports_index: usize) -> serialport::SerialPortBuilder {
    //     let select_port = &self.Ports.clone().unwrap()[ports_index];
    //     serialport::new(select_port, 9600)
    //         .open().unwrap();
    // }

    pub fn open(&mut self, port_name: String) -> Result<Box<dyn SerialPort>, serialport::Error> {
        let port = serialport::new(port_name, 9600)
            .open();

        match &port {
            Ok(p) => {
                self.State = 1;
            }
            Err(p) => {}
        }

        port
    }

    pub fn reset(&mut self) {
        self.State = 0;
        self.PortList = None;
    }
}