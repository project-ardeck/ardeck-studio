type SerialPortInfo = {
    
    port_name: string,
    port_type: {
        UsbPort?: {
            vid: number,
            pid: number,
            serial_number: string,
            manufacturer: string,
            product: string
        },
        PciPort?: {},
        BluetoothPort?: {},
    }
}

type OnMessageSerial = {
    data: number,
    timestamp: number
}

type SwitchData = {
    switchType: "digital" | "analog",
    id: number,
    state: number,
    rawData: number[],
    timestamp: number,
}

type serialPortState = {
    port_name: string,
    status: "open" | "closed" | "error" = "closed",
    // port_state: {
    //     open: boolean,
    //     baud_rate: number,
    //     data_bits: number,
    //     stop_bits: number,
    //     parity: string,
    //     flow_control: string
    // }
}
