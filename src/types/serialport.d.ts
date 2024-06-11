type serialPortInfo = {
    
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